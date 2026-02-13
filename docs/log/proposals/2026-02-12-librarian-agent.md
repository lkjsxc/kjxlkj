# Proposal: Autonomous Librarian Agent

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Date

2026-02-12

## Decision

Add an autonomous librarian agent that structures documentation-level information
using LLM providers through a strict attribute-less XML-like protocol.

## Scope

- use automation domain for librarian rules and run lifecycle
- support provider modes for OpenRouter and LM Studio
- define deterministic parser and retry contract for `xml_attrless`
- add staged reconstruction plan and ledger tracking for librarian runtime

## Rationale

- enable high-leverage autonomous documentation structuring
- keep deterministic behavior with small-parameter models
- avoid provider lock-in with OpenAI-compatible adapter contract

## Follow-Up

Execution is tracked in [/docs/todo/waves/README.md](/docs/todo/waves/README.md).
