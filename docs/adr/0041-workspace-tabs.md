# 0041 — Workspace tabs for detail screens

**Status:** Accepted

**Context.** Opening a ticket, a quotation or an order replaced the list on screen, and a second
record meant leaving the first one: an unsent reply or half-edited quote was lost with the screen
(go_router rebuilt it). Support and sales staff work on several records at once.

**Decision.**

* **Tabs above the work area.** The shell shows a strip with the main area ("Overview": the lists
  and settings pages, as before) and one tab per open detail screen: helpdesk tickets, quotations
  (also new ones), orders, repairs, jobs and contracts, products, opportunities, stock products and
  counts, knowledge-base articles, production orders and delivery notes (the configurator
  screens stay plain routes). Narrow layouts have no room for the strip and list the tabs in
  an app bar menu.
* **Tabs are keyed by location.** Opening a location that is already open focuses its tab.
  Middle-click or ctrl-click on a list row opens it in the background; right-click or long-press
  offers "open in new tab"; middle-click on a tab closes it.
* **One `Navigator` per tab, all alive.** The tabs sit in an `IndexedStack`, so a tab keeps its
  screen state while another tab is shown. A tab built from a restored location is only built when
  first shown. Inactive tabs do not tick and cannot take focus.
* **`push` and `pop(result)` keep working.** Screens call `context.push/go/pop/canPop` as before.
  Inside a tab, `GoRouter.of(context)` is a small stand-in (`TabRouter`) acting on the tab's
  navigator, so a quotation pushed from a ticket stays in the ticket's tab and returns its result
  to it. A `go` to a list (the usual back button when nothing is left to pop) closes the tab and
  shows the list in the main area. Nested real `GoRouter`s were rejected: each would also listen
  to platform route changes (the browser's back button on web).
* **Lists are unchanged.** The main router still has the detail routes; they build a
  `DetailPortal` that opens or focuses the tab and removes itself, so `context.push('/orders/..')`
  from a list works as before (its future completes at once, and `EntityList` reloads).
* **Unsaved edits.** Closing a tab with unsaved edits asks first. Only the quotation screen reports
  dirty state today (`TabScope.reportDirty`); other screens save explicitly.
* **Background tabs are quiet.** The ticket presence beat (every 20 s) runs only while its tab is
  focused ([ADR 0023](0023-helpdesk-collaboration.md)).
* **Persistence.** Open tabs (locations and titles) are stored per profile and user in the profile
  store ([ADR 0040](0040-profiles.md)) and restored on start. Screen state is not persisted.
* `QuotationScreen` now handles a changed `id` (`didUpdateWidget`), and every detail screen gets a
  key made of its location.

**Consequences.**

* A list does not refresh when a record changed in a tab; it reloads when the user opens another
  record or searches (a later step could invalidate list providers on tab focus).
* Android's back button does not close tabs yet.
* Dialogs use the root navigator and appear above all tabs; `GoRouter.of` inside a dialog is the
  main router.
