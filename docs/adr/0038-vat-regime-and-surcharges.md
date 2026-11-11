# 0038 — VAT regime per customer and negative rebates

**Status:** Accepted

**Context.** VAT was a per-product rate that every quotation charged and printed. Most customers
are businesses for whom VAT is irrelevant on the quote (reverse charge inside the EU, or export),
and a customer with a negative general rebate (a surcharge) saw the lower list price on the PDF.

**Decision.** Migration `20270121000001_vat_regime_and_surcharge.sql`.

* **`crm.customers.vat_regime`**: `domestic` (default, what existed before: VAT charged),
  `intra_eu` (business in another EU country, VAT reverse-charged) or `outside_eu` (export). It is
  snapshotted on the quotation (`sales.quotations.vat_regime`) with the VAT number, and travels in
  the customer CSV (column `vat_regime`, [ADR 0029](0029-csv-export-import.md)).
* **Pricing.** Only `domestic` lines carry the product's VAT rate; the others are priced with
  rate 0, so totals, orders made from the quotation and stored VAT groups follow.
* **Quotation PDF.** For `intra_eu` and `outside_eu` the document has no VAT column, no VAT
  groups and a single "Total", plus a line saying why (reverse charge, or export).
* **Negative general rebate.** `customer_terms.discount_percent` and `customer_rebates.percent`
  may be -100..100; -10 raises the list price by 10 %. The extra discount of a quotation line
  is -100..100 as well (negative = extra charge on top of the rebate). On the quotation PDF a
  line whose rebate or net result is a surcharge shows only the resulting unit price, no list
  price and no discount; the app still shows list price, rebate and net.
* **Sub-items.** The quotation screen gets a "+" on each line to attach any product below it
  (the data model already supported parents, [ADR 0033](0033-configured-quotations.md)).

**Consequences.** Order confirmations and other documents still print what they stored; they
are not changed here (order documents do not yet hide the list price for a surcharge).
