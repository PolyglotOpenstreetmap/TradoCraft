//! BOM explosion and cycle detection. Pure functions, no I/O.
//!
//! A product structure is a map from product to the lines of its *active* BOM. A component with
//! its own entry is a subassembly; one without is a leaf (purchased part, or a manufactured part
//! without a BOM yet).

use std::collections::HashMap;

use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineSpec {
    pub component_id: Uuid,
    pub quantity: Decimal,
    pub scrap_percent: Decimal,
}

pub type Structure = HashMap<Uuid, Vec<LineSpec>>;

/// One node of an exploded structure, in depth-first order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub product_id: Uuid,
    /// Index of the parent node in the output, `None` for direct components of the root.
    pub parent: Option<usize>,
    /// 1 = direct component of the exploded product.
    pub level: u32,
    pub quantity_per_parent: Decimal,
    pub scrap_percent: Decimal,
    /// Quantity needed for the requested root quantity, scrap included (exact, unrounded).
    pub required_quantity: Decimal,
    pub is_leaf: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycle(pub Vec<Uuid>);

/// Gross quantity of one BOM line for one parent: quantity × (1 + scrap %).
pub fn gross(line: &LineSpec) -> Decimal {
    line.quantity * (Decimal::ONE_HUNDRED + line.scrap_percent) / Decimal::ONE_HUNDRED
}

/// Returns a cycle reachable from `root` (as a path that starts and ends with the same product).
pub fn find_cycle(structure: &Structure, root: Uuid) -> Option<Cycle> {
    fn visit(structure: &Structure, id: Uuid, path: &mut Vec<Uuid>) -> Option<Cycle> {
        if let Some(pos) = path.iter().position(|p| *p == id) {
            let mut cycle = path[pos..].to_vec();
            cycle.push(id);
            return Some(Cycle(cycle));
        }
        let lines = structure.get(&id)?;
        path.push(id);
        for line in lines {
            if let Some(c) = visit(structure, line.component_id, path) {
                return Some(c);
            }
        }
        path.pop();
        None
    }
    visit(structure, root, &mut Vec::new())
}

/// Explodes `quantity` of `root` through all levels.
pub fn explode(structure: &Structure, root: Uuid, quantity: Decimal) -> Result<Vec<Node>, Cycle> {
    if let Some(c) = find_cycle(structure, root) {
        return Err(c);
    }
    fn walk(
        structure: &Structure,
        id: Uuid,
        qty: Decimal,
        parent: Option<usize>,
        level: u32,
        out: &mut Vec<Node>,
    ) {
        for line in structure.get(&id).map(Vec::as_slice).unwrap_or_default() {
            let required = qty * gross(line);
            let children = structure.get(&line.component_id);
            out.push(Node {
                product_id: line.component_id,
                parent,
                level,
                quantity_per_parent: line.quantity,
                scrap_percent: line.scrap_percent,
                required_quantity: required,
                is_leaf: children.is_none_or(|c| c.is_empty()),
            });
            let index = out.len() - 1;
            walk(
                structure,
                line.component_id,
                required,
                Some(index),
                level + 1,
                out,
            );
        }
    }
    let mut out = Vec::new();
    walk(structure, root, quantity, None, 1, &mut out);
    Ok(out)
}

/// Total leaf requirements (what must be bought or made without a BOM), in order of first use.
pub fn leaf_requirements(nodes: &[Node]) -> Vec<(Uuid, Decimal)> {
    let mut totals: Vec<(Uuid, Decimal)> = Vec::new();
    for n in nodes.iter().filter(|n| n.is_leaf) {
        match totals.iter_mut().find(|(id, _)| *id == n.product_id) {
            Some((_, q)) => *q += n.required_quantity,
            None => totals.push((n.product_id, n.required_quantity)),
        }
    }
    totals
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    fn line(component: u128, qty: Decimal, scrap: Decimal) -> LineSpec {
        LineSpec {
            component_id: id(component),
            quantity: qty,
            scrap_percent: scrap,
        }
    }

    /// 1 = unit, 2 = frame assembly, 3 = pump, 4 = steel profile (m), 5 = bolt
    fn sample() -> Structure {
        HashMap::from([
            (
                id(1),
                vec![
                    line(2, dec!(1), dec!(0)),
                    line(3, dec!(2), dec!(0)),
                    line(5, dec!(8), dec!(0)),
                ],
            ),
            (
                id(2),
                vec![line(4, dec!(6.5), dec!(10)), line(5, dec!(24), dec!(0))],
            ),
        ])
    }

    #[test]
    fn explodes_multiple_levels_with_cumulative_quantities() {
        let nodes = explode(&sample(), id(1), dec!(3)).unwrap();
        let summary: Vec<_> = nodes
            .iter()
            .map(|n| (n.product_id, n.level, n.parent, n.required_quantity))
            .collect();
        assert_eq!(
            summary,
            vec![
                (id(2), 1, None, dec!(3)),
                (id(4), 2, Some(0), dec!(21.45)), // 3 × 6.5 × 1.10
                (id(5), 2, Some(0), dec!(72)),    // 3 × 24
                (id(3), 1, None, dec!(6)),
                (id(5), 1, None, dec!(24)),
            ]
        );
        assert!(!nodes[0].is_leaf);
        assert!(nodes[1].is_leaf);
    }

    #[test]
    fn leaf_requirements_sum_the_same_component_across_levels() {
        let nodes = explode(&sample(), id(1), dec!(3)).unwrap();
        assert_eq!(
            leaf_requirements(&nodes),
            vec![(id(4), dec!(21.45)), (id(5), dec!(96)), (id(3), dec!(6))]
        );
    }

    #[test]
    fn product_without_bom_explodes_to_nothing() {
        assert!(explode(&sample(), id(3), dec!(1)).unwrap().is_empty());
    }

    #[test]
    fn detects_direct_and_indirect_cycles() {
        let mut s = sample();
        s.insert(id(4), vec![line(1, dec!(1), dec!(0))]); // profile contains the unit
        let Cycle(path) = find_cycle(&s, id(1)).unwrap();
        assert_eq!(path, vec![id(1), id(2), id(4), id(1)]);
        assert!(explode(&s, id(1), dec!(1)).is_err());

        let self_ref = HashMap::from([(id(9), vec![line(9, dec!(1), dec!(0))])]);
        assert_eq!(
            find_cycle(&self_ref, id(9)),
            Some(Cycle(vec![id(9), id(9)]))
        );
    }

    #[test]
    fn shared_subassembly_is_not_a_cycle() {
        // Diamond: 1 -> 2 -> 4 and 1 -> 3 -> 4.
        let s = HashMap::from([
            (
                id(1),
                vec![line(2, dec!(1), dec!(0)), line(3, dec!(1), dec!(0))],
            ),
            (id(2), vec![line(4, dec!(2), dec!(0))]),
            (id(3), vec![line(4, dec!(3), dec!(0))]),
        ]);
        assert_eq!(find_cycle(&s, id(1)), None);
        assert_eq!(
            leaf_requirements(&explode(&s, id(1), dec!(1)).unwrap()),
            vec![(id(4), dec!(5))]
        );
    }
}
