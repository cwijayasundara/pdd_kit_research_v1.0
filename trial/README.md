# Verus login trial — a simple-app test of the PDD claim

**Which experiment this is:** Experiment B from `../research_notes/PDD kit trial plan/`
— a **Verus-only reconstruction** of the published Pet Store login
verification. PDD Kit itself (the `pdd-plan` / `pdd-prove` / `pdd-ship`
workflows inside Chopin) has no public install as of 2026-11-07. This
scaffold tests the checker, the spec discipline, and the trusted-boundary
story. It is **not** a PDD Kit run, and the write-up must say so.

**The claim, in one sentence (pre-committed):**

> A Rust auth handler on the production path meets an approved Verus spec
> for every input, with the password check and the account lookup
> explicitly unproved and tested — and one planted bug per `ensures` clause
> is rejected.

## Layout

```
trial/
├── intent/intent.md              frozen rough request (two sentences)
├── intent/decisions.md           decision cards D1–D7 + spec-approval hash gate
├── backend/                      the simple app (Axum + SQLite, ~350 lines)
│   └── src/handlers/auth.rs      verification target: login_impl, pre-verification shape
│   └── src/verified/login.rs     SPEC TEMPLATE — contracts first, bodies unimplemented!()
│   └── src/trusted.rs            the trusted ring (bcrypt, clock, JWT, db error)
│   └── tests/trusted_boundary.rs pinning e2e tests + trusted-ring tests
├── mutations/mutants.md          one planted bug per clause + procedure + log
└── scripts/
    ├── install-verus.sh          prebuilt arm64-macOS Verus + Gatekeeper fix
    ├── verify.sh                 the deterministic gate (no model in pass/fail)
    └── verus_smoke.rs            toolchain smoke (and a mutant for the smoke itself)
```

## Setup

```bash
# 0) Rust toolchain (absent on this machine as of 2026-11-07)
curl https://sh.rustup.rs | sh

# 1) Verus (prebuilt arm64 macOS binary exists as of release 0.2026.11.02.8bf68ee)
scripts/install-verus.sh

# 2) Sanity: the app builds and today's behaviour is pinned
cd trial && cargo test -p backend

# 3) Toolchain smoke
scripts/verify.sh     # runs verus on the self-contained smoke file
```

## The ladder (phases from the research notes, condensed)

1. **Baseline (behaviour only).** `cargo test -p backend` green. No proof
   exists. Every e2e test pins today's 200/400/401 matrix — the spec's
   "pin today's refusals" clauses must match these.
2. **Task 1 — wire the spec.** Make `verified/src/login.rs` build under
   Verus *inside* the backend crate while the tests still pass. Change no
   contract. If a construct cannot be expressed: stop and record a
   decision — do NOT weaken a clause or drop to a non-async helper.
3. **Spec approval (the hash gate).** Fill in every decision card in
   `intent/decisions.md` (both directions of the password property — D1 —
   is not optional), record `shasum -a 256 backend/src/verified/login.rs`
   while the five proved functions are still `unimplemented!()`.
4. **Proof stage.** Have a coding agent fill the bodies of `login_flow`,
   `login_core`, `issue_token`, `lookup_target`, `decide_login`. Verus is
   the only pass/fail. Then rewire `handlers/auth.rs` to unpack-and-await
   `login_flow` (the commented shell) and confirm the e2e tests still pass.
5. **Mutation check.** Run all nine mutants in `mutations/mutants.md`.
   Every one must FAIL Verus on the clause it aims at. A surviving mutant
   is a vacuous clause → fix the spec, re-approve, re-prove.
6. **Deterministic re-run.** Re-run `scripts/verify.sh` from a clean clone
   on the committed files, no model involved. Record the command in
   D-toolchain.
7. **Change one clause.** E.g. expiry 24h → 1h, or email no longer wins.
   The old proof must FAIL; re-approve; re-prove; unrelated trusted tests
   still pass.

## Pass checklist (all must be recorded)

- [ ] Person can point at each `ensures` clause and name the English sentence behind it
- [ ] Both directions of the password property are proved (D1 = both ways)
- [ ] Verus exit 0 on the committed files, re-run without a model
- [ ] Every clause has a rejected mutant (9/9 in the log)
- [ ] Trusted list explicit, each piece individually accepted (D6), each with a test written before the test existed
- [ ] The route handler calls the proved function (no side copy)
- [ ] One later clause change broke the old proof and forced a new one

## False proofs — a green check is worthless if any of these is true

Types/cargo check alone; a spec copied from the code (restating what it
returns); only the "only-if" half of the property; the blog's weaker
`Internal(_)` paste where GitHub Next ties 500 to `lookup_fails ||
signing_fails`; agent-written tests of the proved core added after the
fact; UI walkthrough counted as verification; a proof of a copy the route
never calls; calling the app "verified" because login's spec passed;
installing `pdd-toolkit` / `pdd-kit.dev` (the Product-Driven Development
package — no Verus, wrong project).

## Stop conditions (limitations, not lack of enthusiasm)

- Verus cannot express an async construct and the only way forward is
  weakening a contract — that is the published workflow's own hard stop.
- The property you actually care about is latency, rate limits, the UI, or
  anything else on the D7 "not promised" list — record as out of class.
- You catch yourself writing a store (inventory, cart, Leptos). Login
  scale is the claim; the rest is decoration.

## Provenance

- https://githubnext.com/projects/pdd-kit/
- https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/
- Local research notes: `../research_notes/PDD kit trial plan/` (method and
  claims, correct usage, trial design, pet-store reference). Where the two
  primary pages disagree, this scaffold follows the GitHub Next clause text.

*Note: the scaffold has not been compiled yet (no Rust toolchain on this
machine at scaffold time); `cargo check` is step 0 of setup and may need
trivial version drift fixes.*
