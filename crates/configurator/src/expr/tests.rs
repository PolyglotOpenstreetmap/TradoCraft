//! Tests of the type checker, the interpreter and templates.

use std::collections::BTreeMap;

use rust_decimal_macros::dec;

use super::*;

struct TestHost;

impl Host for TestHost {
    fn lookup(
        &self,
        table: &str,
        keys: &BTreeMap<String, Value>,
        offset: i64,
    ) -> Result<Value, String> {
        if table != "sizes" {
            return Err(format!("unknown table {table}"));
        }
        let n = keys.get("n").and_then(Value::as_int).unwrap_or(0) + offset;
        Ok(if (1..=3).contains(&n) {
            let mut r = BTreeMap::new();
            r.insert("size".into(), Value::Int(n * 10));
            Value::Record(r)
        } else {
            Value::Null
        })
    }
    fn rows(&self, _: &str) -> Result<Value, String> {
        Ok(Value::List(vec![]))
    }
    fn product(&self, code: &str) -> Value {
        if code != "MOT-1" {
            return Value::Null;
        }
        let mut attrs = BTreeMap::new();
        attrs.insert("kw".into(), Value::Dec(dec!(1.1)));
        let mut m = BTreeMap::new();
        m.insert("code".into(), Value::Str(code.into()));
        m.insert("active".into(), Value::Bool(true));
        m.insert("attributes".into(), Value::Record(attrs));
        Value::Record(m)
    }
}

fn env() -> BTreeMap<String, Value> {
    let mut m = BTreeMap::new();
    m.insert("a".into(), Value::Int(7));
    m.insert("x".into(), Value::Dec(dec!(2.5)));
    m.insert("name".into(), Value::Str("Hall 1".into()));
    m.insert("none".into(), Value::Null);
    m.insert("on".into(), Value::Bool(true));
    m.insert(
        "items".into(),
        Value::from_json(&serde_json::json!([
            {"len": 10, "kw": "0.5", "hall": "A"},
            {"len": 4, "kw": "1.25", "hall": "B"},
            {"len": 6, "kw": null, "hall": "A"}
        ])),
    );
    // from_json keeps "0.5" as text; make the kw values decimals.
    if let Some(Value::List(items)) = m.get_mut("items") {
        for it in items {
            if let Value::Record(r) = it {
                if let Some(Value::Str(s)) = r.get("kw").cloned() {
                    r.insert("kw".into(), Value::Dec(s.parse().unwrap()));
                }
            }
        }
    }
    m
}

fn run(src: &str) -> Value {
    let e = parse(src).unwrap_or_else(|e| panic!("{src}: {e:?}"));
    eval(&e, &env(), &TestHost, &Budget::new(10_000)).unwrap_or_else(|e| panic!("{src}: {e:?}"))
}

fn run_err(src: &str) -> ExprError {
    let e = parse(src).unwrap();
    eval(&e, &env(), &TestHost, &Budget::new(10_000)).unwrap_err()
}

fn d(s: &str) -> Value {
    Value::Dec(s.parse().unwrap())
}

#[test]
fn arithmetic_is_exact_and_keeps_integers() {
    assert_eq!(run("1 + 2 * 3"), Value::Int(7));
    assert_eq!(run("0.1 + 0.2"), d("0.3"));
    assert_eq!(run("a / 2"), d("3.5"));
    assert_eq!(run("a % 4"), Value::Int(3));
    assert_eq!(run("x * 2"), d("5.0"));
    assert_eq!(run("-a"), Value::Int(-7));
    assert_eq!(run("'A' + 'B'"), Value::Str("AB".into()));
    assert!(run_err("1 / 0").message.contains("division by zero"));
    assert_eq!(run("9223372036854775807 + 1"), d("9223372036854775808"));
}

#[test]
fn comparisons_logic_ternary_and_null() {
    assert_eq!(run("a > 5 and x < 3"), Value::Bool(true));
    assert_eq!(run("not on or a == 7.0"), Value::Bool(true));
    assert_eq!(run("a >= 8 ? 'big' : 'small'"), Value::Str("small".into()));
    assert_eq!(run("none + 1"), Value::Null);
    assert_eq!(run("none > 1"), Value::Null);
    assert_eq!(run("none ?? 5"), Value::Int(5));
    assert_eq!(run("a ?? 5"), Value::Int(7));
    assert_eq!(run("none == null"), Value::Bool(true));
    assert_eq!(run("none ? 1 : 2"), Value::Int(2));
    assert_eq!(run("none.field"), Value::Null);
    assert_eq!(run("'b' > 'a'"), Value::Bool(true));
    // Short circuit: the right side is never evaluated.
    assert_eq!(run("false and 1 / 0 > 1"), Value::Bool(false));
    let e = run_err("a and on");
    assert!(e.message.contains("yes/no"), "{e:?}");
    assert_eq!((e.start, e.end), (0, 1));
}

#[test]
fn math_and_text_functions() {
    assert_eq!(run("ceil(2.1)"), Value::Int(3));
    assert_eq!(run("floor(-2.1)"), Value::Int(-3));
    assert_eq!(run("round(2.345, 2)"), d("2.35"));
    assert_eq!(run("round(2.5)"), Value::Int(3));
    assert_eq!(run("ceil(2.341, 1)"), d("2.4"));
    assert_eq!(run("min(3, a, 1.5)"), d("1.5"));
    assert_eq!(run("max(items, i => i.len)"), Value::Int(10));
    assert_eq!(run("abs(-2.5)"), d("2.5"));
    assert_eq!(run("sqrt(16)"), d("4"));
    assert_eq!(run("round(sin(pi / 2), 6)"), d("1.000000"));
    assert_eq!(run("round(cos(0))"), Value::Int(1));
    assert_eq!(run("pow(2, 10)"), d("1024"));
    assert_eq!(run("concat('M', a, '-', x)"), Value::Str("M7-2.5".into()));
    assert_eq!(run("pad(a, 3)"), Value::Str("007".into()));
    assert_eq!(run("pad('x', 3, '_')"), Value::Str("__x".into()));
    assert_eq!(run("upper('ab') + lower('CD')"), Value::Str("ABcd".into()));
    assert_eq!(run("len(name)"), Value::Int(6));
    assert_eq!(
        run("matches('DRV-11T', '^DRV-[0-9]+T$')"),
        Value::Bool(true)
    );
    assert_eq!(
        run("capture('DRV-11T', 'DRV-([0-9]+)')"),
        Value::Str("11".into())
    );
    assert_eq!(run("capture('X', '([0-9]+)')"), Value::Null);
    assert_eq!(run("number('1.5') * 2"), d("3.0"));
    assert_eq!(run("str(1.50)"), Value::Str("1.5".into()));
    assert_eq!(run("contains([1, 2], 2.0)"), Value::Bool(true));
    assert!(run_err("capture('a', '(', 1)")
        .message
        .contains("invalid pattern"));
    assert!(run_err("capture('a', 'a', 2)")
        .message
        .contains("no group 2"));
}

#[test]
fn aggregates_lists_and_lambdas() {
    assert_eq!(run("sum(items, i => i.len)"), Value::Int(20));
    assert_eq!(run("sum(items, i => i.kw)"), d("1.75"));
    assert_eq!(run("count(items)"), Value::Int(3));
    assert_eq!(run("count(items, i => i.hall == 'A')"), Value::Int(2));
    assert_eq!(run("any(items, i => i.len > 8)"), Value::Bool(true));
    assert_eq!(run("all(items, i => i.len > 8)"), Value::Bool(false));
    assert_eq!(
        run("distinct(items, i => i.hall)"),
        Value::List(vec![Value::Str("A".into()), Value::Str("B".into())])
    );
    assert_eq!(
        run("map(filter(items, i => i.len > 5), i => i.len)"),
        run("[10, 6]")
    );
    assert_eq!(run("first(items, i => i.hall == 'B').len"), Value::Int(4));
    assert_eq!(run("first([], i => true)"), Value::Null);
    assert_eq!(run("last([1, 2, 3])"), Value::Int(3));
    assert_eq!(run("sort([3, 1, 2])"), run("[1, 2, 3]"));
    assert_eq!(
        run("map(sort(items, i => i.len), i => i.len)"),
        run("[4, 6, 10]")
    );
    assert_eq!(run("range(1, 4)"), run("[1, 2, 3, 4]"));
    assert_eq!(run("[1, 2, 3][-1]"), Value::Int(3));
    assert_eq!(run("[1, 2, 3][5]"), Value::Null);
    assert_eq!(run("avg([1, 2])"), d("1.5"));
    assert_eq!(run("{a: 1, b: 'x'}.b"), Value::Str("x".into()));
    // A lambda sees the surrounding names, and parameters shadow them.
    assert_eq!(run("map([1, 2], v => v + a)"), run("[8, 9]"));
    assert_eq!(run("map([1, 2], a => a * 2)"), run("[2, 4]"));
}

#[test]
fn packing_functions() {
    assert_eq!(run("chunks(9, 4)"), run("[4, 4, 1]"));
    assert_eq!(run("chunks(8, 4)"), run("[4, 4]"));
    assert_eq!(run("chunks(0, 4)"), run("[]"));
    assert!(run_err("chunks(4, 0)").message.contains("at least 1"));
    assert_eq!(run("bins([3, 2, 2, 1], 4)"), run("[[3, 1], [2, 2]]"));
    assert_eq!(run("len(bins([1, 1, 1], 2))"), Value::Int(2));
    assert!(run_err("bins([5], 4)").message.contains("does not fit"));
}

#[test]
fn tables_and_products_through_the_host() {
    assert_eq!(run("lookup('sizes', {n: 2}).size"), Value::Int(20));
    assert_eq!(run("lookup('sizes', {n: 2}, 1).size"), Value::Int(30));
    assert_eq!(run("lookup('sizes', {n: 3}, 1)"), Value::Null);
    // "Try 1, 2 or 4 until one fits."
    assert_eq!(
        run("first([4, 2, 1], n => lookup('sizes', {n: n}) != null)"),
        Value::Int(2)
    );
    assert_eq!(run("product('MOT-1').attr('kw')"), d("1.1"));
    assert_eq!(run("product('NOPE').attr('kw')"), Value::Null);
    assert_eq!(run("exists('MOT-1')"), Value::Bool(true));
    assert_eq!(run("exists('NOPE')"), Value::Bool(false));
    let e = run_err("lookup('other', {n: 1})");
    assert!(e.message.contains("unknown table"));
    assert_eq!(e.start, 0);
}

#[test]
fn runtime_errors_have_positions() {
    let e = run_err("1 + nothing");
    assert_eq!(e.message, "unknown name nothing");
    assert_eq!((e.start, e.end), (4, 11));
    let e = run_err("a.b");
    assert!(e.message.contains("cannot read .b of integer"));
    let e = run_err("items[0].missing");
    assert!(e.message.contains("no field missing"));
    let e = run_err("name * 2");
    assert!(e.message.contains("cannot use *"));
}

#[test]
fn the_step_budget_stops_runaway_formulas() {
    let e = parse("sum(range(1, 10000), i => sum(range(1, 100), j => j))").unwrap();
    let budget = Budget::new(5_000);
    let err = eval(&e, &env(), &TestHost, &budget).unwrap_err();
    assert!(err.message.contains("step budget"), "{err:?}");
    // A normal formula uses few steps.
    let budget = Budget::new(5_000);
    eval(&parse("a + 1").unwrap(), &env(), &TestHost, &budget).unwrap();
    assert_eq!(budget.used(), 3);
    assert!(run_err("range(1, 100000)").message.contains("at most"));
}

#[test]
fn templates_interpolate_and_report_positions() {
    let t = Template::parse("DRV-{pad(a, 2)}{x < 3 ? 'L' : 'H'} {{raw}}").unwrap();
    let s = t
        .render(&[], &env(), &TestHost, &Budget::new(1000))
        .unwrap();
    assert_eq!(s, "DRV-07L {raw}");
    let t = Template::parse("{ {k: 'v'}.k }").unwrap();
    assert_eq!(
        t.render(&[], &env(), &TestHost, &Budget::new(1000))
            .unwrap(),
        "v"
    );
    let t = Template::parse("{'}'}").unwrap();
    assert_eq!(
        t.render(&[], &env(), &TestHost, &Budget::new(1000))
            .unwrap(),
        "}"
    );
    let e = Template::parse("ab{1 +}").unwrap_err();
    assert_eq!(e.start, 6, "{e:?}");
    assert!(Template::parse("ab{")
        .unwrap_err()
        .message
        .contains("unclosed"));
    assert!(Template::parse("a}b")
        .unwrap_err()
        .message
        .contains("unmatched"));
    let t = Template::parse("{none}-{each}").unwrap();
    let s = t
        .render(
            &[("each".into(), Value::Int(4))],
            &env(),
            &TestHost,
            &Budget::new(100),
        )
        .unwrap();
    assert_eq!(s, "-4");
}

struct TEnv;

impl TypeEnv for TEnv {
    fn name(&self, n: &str) -> Option<Type> {
        Some(match n {
            "a" => Type::Int,
            "x" => Type::Dec,
            "name" => Type::Str,
            "on" => Type::Bool,
            "items" => {
                let mut f = BTreeMap::new();
                f.insert("len".into(), Type::Int);
                f.insert("hall".into(), Type::Str);
                Type::List(Box::new(Type::Record(Some(f))))
            }
            _ => return None,
        })
    }
    fn table(&self, n: &str) -> Option<TableInfo> {
        (n == "sizes").then(|| {
            let mut columns = BTreeMap::new();
            columns.insert("n".into(), Type::Int);
            columns.insert("size".into(), Type::Int);
            TableInfo {
                columns,
                keys: vec!["n".into()],
            }
        })
    }
}

fn ty(src: &str) -> Result<Type, Vec<ExprError>> {
    let e = parse(src).unwrap();
    let (t, errs) = check(&e, &TEnv);
    if errs.is_empty() {
        Ok(t)
    } else {
        Err(errs)
    }
}

#[test]
fn type_checker_infers_types() {
    assert_eq!(ty("a + 1"), Ok(Type::Int));
    assert_eq!(ty("a + x"), Ok(Type::Dec));
    assert_eq!(ty("a / 2"), Ok(Type::Dec));
    assert_eq!(ty("a > 1 ? 'x' : null"), Ok(Type::Str));
    assert_eq!(ty("sum(items, i => i.len)"), Ok(Type::Int));
    assert_eq!(ty("count(items, i => i.hall == 'A')"), Ok(Type::Int));
    assert_eq!(ty("lookup('sizes', {n: a}).size"), Ok(Type::Int));
    assert_eq!(
        ty("map(items, i => i.hall)"),
        Ok(Type::List(Box::new(Type::Str)))
    );
    assert_eq!(ty("chunks(a, 4)"), Ok(Type::List(Box::new(Type::Int))));
    assert_eq!(ty("product('X').attr('kw')"), Ok(Type::Any));
    assert_eq!(ty("round(x)"), Ok(Type::Int));
    assert_eq!(ty("round(x, 2)"), Ok(Type::Dec));
    assert_eq!(ty("first(items).len"), Ok(Type::Int));
    assert_eq!(ty("pi * 2"), Ok(Type::Dec));
}

#[test]
fn type_checker_reports_mistakes_with_positions() {
    let e = ty("a + nope").unwrap_err();
    assert_eq!(e[0].message, "unknown name nope");
    assert_eq!((e[0].start, e[0].end), (4, 8));
    let e = ty("name * 2").unwrap_err();
    assert!(e[0].message.contains("cannot use * on text"), "{e:?}");
    let e = ty("lookup('sizes', {m: 1}).size").unwrap_err();
    assert!(e[0].message.contains("m is not a key"), "{e:?}");
    let e = ty("lookup('nope', {n: 1})").unwrap_err();
    assert!(e[0].message.contains("unknown table nope"));
    assert_eq!(e[0].start, 7);
    let e = ty("lookup('sizes', {n: 1}).width").unwrap_err();
    assert!(e[0].message.contains("no field width"), "{e:?}");
    let e = ty("first(items).color").unwrap_err();
    assert!(e[0].message.contains("no field color"));
    let e = ty("frobnicate(1)").unwrap_err();
    assert!(e[0].message.contains("unknown function"));
    let e = ty("round(1, 2, 3)").unwrap_err();
    assert!(e[0].message.contains("takes 1 to 2"), "{e:?}");
    let e = ty("filter(items, i => i.len)").unwrap_err();
    assert!(e[0].message.contains("yes/no"), "{e:?}");
    let e = ty("sum(a, i => i)").unwrap_err();
    assert!(e[0].message.contains("needs a list"));
    let e = ty("on ? 1 : 2 + name").unwrap_err();
    assert!(e[0].message.contains("cannot use +"));
    let e = ty("matches(name, '(')").unwrap_err();
    assert!(e[0].message.contains("invalid pattern"));
    let e = ty("not a").unwrap_err();
    assert!(e[0].message.contains("'not'"));
    let e = ty("map(items, a)").unwrap_err();
    assert!(e[0].message.contains("expected a function"), "{e:?}");
    // Several mistakes are reported together.
    assert_eq!(ty("nope1 + nope2").unwrap_err().len(), 2);
    // An open environment accepts any name.
    let (_, errs) = check(&parse("anything + 1").unwrap(), &OpenEnv);
    assert!(errs.is_empty());
}
