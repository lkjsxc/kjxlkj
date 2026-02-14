# 2026-02-14 HTTP + WS Follow-ups

Back: [/docs/log/improvements/2026/README.md](/docs/log/improvements/2026/README.md)

## Scope

Improvement backlog discovered while restoring reachable HTTP/WS/security
runtime contracts.

## Improvement Ideas

1. replace current placeholder endpoints with explicit typed request/response
   DTO modules grouped by domain (`auth`, `notes`, `automation`, `attachments`)
2. add a deterministic WS replay store keyed by `stream_id` + `event_seq` with
   bounded retention and explicit cursor-compaction policy
3. implement a dedicated CSRF middleware layer to centralize mutating-route
   checks and reduce per-handler duplication
4. add acceptance-ID annotations in test names (`API-AUTH-01`, `WS-03`, etc.)
   and generate an evidence map artifact on test completion
5. replace ad-hoc timestamp formatting with an RFC3339 utility shared across
   handlers and event emitters
6. implement explicit automation/librarian route validators that enforce
   provider kind and `xml_attrless` protocol shape before persistence
7. add WS test coverage for stale cursor (`STALE_CURSOR`) and workspace stream
   replay ordering (`WS-02`)
