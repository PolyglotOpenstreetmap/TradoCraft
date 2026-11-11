# 0009 — PDF documents with Typst

**Status:** Accepted

**Decision.** Business documents are Typst templates (`templates/*.typ`) rendered by the `typst`
CLI. The API passes pre-formatted data as `data.json`; templates never calculate. Labels are
selected by the customer's language (nl / fr / en). Drafts carry a watermark.

**Why not HTML + headless Chromium.** Typst is a single small binary, starts in milliseconds,
has precise pagination and needs no browser to run, patch and sandbox on the server.
