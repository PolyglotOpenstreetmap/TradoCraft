# 0010 — Flutter, desktop-first for the office

**Status:** Accepted

**Decision.**
* One Flutter codebase. Office users get the Windows desktop app (dense tables, keyboard entry,
  multiple windows later); technicians get Android/iOS. Flutter web is a convenience target, not
  the primary office client (weaker text selection, browser find and very large tables).
* Riverpod for state, go_router for navigation, Dio + generated client for the API.
* A future public customer portal may use a lighter web technology if SEO or accessibility
  requirements demand it.
