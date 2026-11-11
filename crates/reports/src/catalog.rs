//! The object catalog: the only place where report object and field names turn into SQL.
//!
//! A report definition names objects and fields as strings. Those strings are looked up here;
//! anything that is not in the catalog is rejected, and nothing a client sends is ever pasted
//! into a query. Values are always bound parameters.

use serde::{Deserialize, Serialize};
use tradocraft_core::Permission;
use utoipa::ToSchema;

/// The type of a report field, which decides the operators and summaries it allows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportFieldType {
    Text,
    Number,
    Date,
    Datetime,
    Bool,
}

/// One field of a catalog object, with the SQL it stands for.
#[derive(Debug)]
pub struct Field {
    /// Name inside the object, e.g. `total`. In a definition it is written `quotation.total`.
    pub name: &'static str,
    pub label: &'static str,
    pub ty: ReportFieldType,
    /// SQL expression over the object's alias. Numbers are cast to `numeric`.
    pub sql: &'static str,
    /// The row counter: it can only be counted.
    pub count_only: bool,
}

const fn field(
    name: &'static str,
    label: &'static str,
    ty: ReportFieldType,
    sql: &'static str,
) -> Field {
    Field {
        name,
        label,
        ty,
        sql,
        count_only: false,
    }
}

const fn records(sql: &'static str) -> Field {
    Field {
        name: "records",
        label: "Number of records",
        ty: ReportFieldType::Text,
        sql,
        count_only: true,
    }
}

/// A catalog object: a table (with its alias) that reports can read.
#[derive(Debug)]
pub struct Object {
    pub name: &'static str,
    pub label: &'static str,
    /// SQL alias used for the object's table.
    pub alias: &'static str,
    /// What a user needs to read this object in a report.
    pub permission: Permission,
    pub fields: &'static [Field],
}

use ReportFieldType::{Bool, Date, Datetime, Number, Text};

const CUSTOMER_FIELDS: &[Field] = &[
    records("c.id"),
    field("code", "Code", Text, "c.code"),
    field("name", "Name", Text, "c.name"),
    field("vat_number", "VAT number", Text, "c.vat_number"),
    field("email", "E-mail", Text, "c.email"),
    field("phone", "Phone", Text, "c.phone"),
    field("street", "Street", Text, "c.street"),
    field("postal_code", "Postal code", Text, "c.postal_code"),
    field("city", "City", Text, "c.city"),
    field("country_code", "Country", Text, "c.country_code::text"),
    field("language", "Language", Text, "c.language::text"),
    field("active", "Active", Bool, "c.active"),
    field("created_at", "Created", Datetime, "c.created_at"),
    field(
        "discount_percent",
        "General discount %",
        Number,
        "ct.discount_percent::numeric",
    ),
    field(
        "payment_terms_days",
        "Payment terms (days)",
        Number,
        "ct.payment_terms_days::numeric",
    ),
];

const QUOTATION_FIELDS: &[Field] = &[
    records("q.id"),
    field("number", "Number", Text, "q.number"),
    field("revision", "Revision", Number, "q.revision::numeric"),
    field("status", "Status", Text, "q.status"),
    field("customer_name", "Customer", Text, "q.customer_name"),
    field(
        "customer_country_code",
        "Customer country",
        Text,
        "q.customer_country_code::text",
    ),
    field("currency", "Currency", Text, "q.currency::text"),
    field("quote_date", "Quote date", Date, "q.quote_date"),
    field("valid_until", "Valid until", Date, "q.valid_until"),
    field("subtotal", "Subtotal", Number, "q.subtotal"),
    field("vat_total", "VAT", Number, "q.vat_total"),
    field("total", "Total", Number, "q.total"),
    field("created_at", "Created", Datetime, "q.created_at"),
    field("sent_at", "Sent", Datetime, "q.sent_at"),
];

const ORDER_FIELDS: &[Field] = &[
    records("o.id"),
    field("number", "Number", Text, "o.number"),
    field("status", "Status", Text, "o.status"),
    field("customer_name", "Customer", Text, "o.customer_name"),
    field(
        "customer_reference",
        "Customer reference",
        Text,
        "o.customer_reference",
    ),
    field("currency", "Currency", Text, "o.currency::text"),
    field("order_date", "Order date", Date, "o.order_date"),
    field("subtotal", "Subtotal", Number, "o.subtotal"),
    field("vat_total", "VAT", Number, "o.vat_total"),
    field("total", "Total", Number, "o.total"),
    field("created_at", "Created", Datetime, "o.created_at"),
];

/// Every object a report can use.
pub static OBJECTS: [Object; 3] = [
    Object {
        name: "customer",
        label: "Customers",
        alias: "c",
        permission: Permission::CustomersRead,
        fields: CUSTOMER_FIELDS,
    },
    Object {
        name: "quotation",
        label: "Quotations",
        alias: "q",
        permission: Permission::QuotationsRead,
        fields: QUOTATION_FIELDS,
    },
    Object {
        name: "order",
        label: "Orders",
        alias: "o",
        permission: Permission::OrdersRead,
        fields: ORDER_FIELDS,
    },
];

/// Looks an object up by name.
pub fn object(name: &str) -> Option<&'static Object> {
    OBJECTS.iter().find(|o| o.name == name)
}

impl Object {
    /// Looks a field up by its name inside the object.
    pub fn field(&self, name: &str) -> Option<&'static Field> {
        self.fields.iter().find(|f| f.name == name)
    }
}

/// Resolves `object.field`.
pub fn resolve(qualified: &str) -> Option<(&'static Object, &'static Field)> {
    let (o, f) = qualified.split_once('.')?;
    let obj = object(o)?;
    Some((obj, obj.field(f)?))
}

/// The condition that links two objects (in either direction), or `None` when they are not
/// directly related.
pub fn link(a: &str, b: &str) -> Option<&'static str> {
    match (a.min(b), a.max(b)) {
        ("customer", "quotation") => Some("q.customer_id = c.id"),
        ("customer", "order") => Some("o.customer_id = c.id"),
        ("order", "quotation") => Some("o.quotation_id = q.id"),
        _ => None,
    }
}

/// Every pair of objects that can be joined, for the builder.
pub fn links() -> Vec<(&'static str, &'static str)> {
    let mut out = Vec::new();
    for (i, a) in OBJECTS.iter().enumerate() {
        for b in &OBJECTS[i + 1..] {
            if link(a.name, b.name).is_some() {
                out.push((a.name, b.name));
            }
        }
    }
    out
}

/// The `FROM` fragment that brings `obj` in as the first object. The company is bound by the
/// caller where `{company}` appears.
pub fn base_from(obj: &Object) -> &'static str {
    match obj.name {
        "customer" => {
            "crm.customers c LEFT JOIN crm.customer_terms ct ON ct.customer_id = c.id AND ct.company_id = {company}"
        }
        "quotation" => "sales.quotations q",
        _ => "sales.orders o",
    }
}

/// The `JOIN` fragment that adds `obj` on `on`. `{company}` as in [`base_from`].
pub fn join_from(obj: &Object) -> &'static str {
    match obj.name {
        "customer" => {
            " LEFT JOIN crm.customers c ON {on} LEFT JOIN crm.customer_terms ct ON ct.customer_id = c.id AND ct.company_id = {company}"
        }
        "quotation" => " LEFT JOIN sales.quotations q ON {on} AND q.company_id = {company}",
        _ => " LEFT JOIN sales.orders o ON {on} AND o.company_id = {company}",
    }
}

/// The condition that keeps a base object inside the company. Customers are shared master data
/// and carry no company; their per-company terms are joined in [`base_from`].
pub fn base_filter(obj: &Object) -> Option<&'static str> {
    match obj.name {
        "quotation" => Some("q.company_id = {company}"),
        "order" => Some("o.company_id = {company}"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_field_resolves_and_uses_its_objects_alias() {
        for o in &OBJECTS {
            for f in o.fields {
                let (ro, rf) = resolve(&format!("{}.{}", o.name, f.name)).unwrap();
                assert_eq!((ro.name, rf.name), (o.name, f.name));
                assert!(
                    f.sql.starts_with(o.alias) || f.sql.starts_with("ct."),
                    "{}.{}",
                    o.name,
                    f.name
                );
            }
        }
    }

    #[test]
    fn unknown_names_do_not_resolve() {
        assert!(resolve("quotation.total; drop table x").is_none());
        assert!(resolve("quotation").is_none());
        assert!(resolve("nope.total").is_none());
        assert!(object("sales.quotations").is_none());
    }

    #[test]
    fn links_are_symmetric() {
        assert_eq!(link("quotation", "customer"), link("customer", "quotation"));
        assert!(link("order", "quotation").is_some());
        assert!(link("customer", "customer").is_none());
        assert_eq!(links().len(), 3);
    }
}
