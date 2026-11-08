# Intent — PDD-Kit-grade verification for rig, without a proof checker

**Goal.** Port the load-bearing parts of Proof-Driven Development (PDD Kit /
Chopin, MSR + GitHub Next, Nov 2026) into rig, for stacks with no Verus
(Python, TypeScript). The finding of the trial at
`pdd_kit_research_v1.0/trial/` is that the discipline decomposes: spec-first
with a frozen hash, an explicit trusted boundary, two-sided properties, and a
per-clause mutation receipt survive without an SMT prover. What replaces the
prover is: runtime contracts at the boundary, property-based tests over a pure
core, and mutation testing as the meta-check.

**Scope.** rig itself: the intent/behaviour format, one new architectural
sensor, the PR receipt, the plan-phase intent map. No new runtime dependencies
in user repos beyond the mutation/property tools rig already can invoke
(mutmut or Stryker, hypothesis or fast-check).

**Out of scope.** Formal proofs (no SMT/Verus integration); changing the
plan-approval model; new autonomy rungs.

## Behaviours

### B1 — Every behaviour carries a killing-mutant receipt (`killed-by`)
A behaviour Bₙ is "done" only when its entry in `intent.md` names a recorded
mutant (mutmut/Stryker ID) whose failure is attributable to Bₙ, or carries an
explicit waiver with a reason. A green suite without per-behaviour receipts is
incomplete, the same way a Verus `ensures` without an obligation is.

*Testable:* start a task with 3 behaviours; complete the build without adding
any `killed-by` entries → `/rig:test` reports the missing receipts and
`/rig:pr` refuses (tier-1).

### B2 — Surviving mutants must be adjudicated, not ignored
When a mutant survives, rig requires a one-line adjudication: either
"vacuous — clause Bₙ does not bind; spec fix queued (re-approval)" or
"non-bug — still correct behaviour; documented example". Silent survival is
not allowed. (Trial lesson: a surviving mutant can mean the mutant wasn't a
bug — the smoke `>=`→`>` case — but only adjudication distinguishes that from
a vacuous clause.)

*Testable:* plant a surviving mutant in the fixture repo → `/rig:test`
blocks until `mutations/adjudications.md` gains the adjudication line.

### B3 — Pure-core architectural sensor
Decision logic must live in pure functions: the core module may not import
IO, network, DB, clock, or crypto. IO/crypto/time live in named wrappers at
the edge. This is what made `login_flow` provable in Rust and is what makes
property-based testing possible in Python/TS.

*Testable:* fixture repo with `import requests` inside the decision module →
the sensor fails with the file/line; moving the call behind a named wrapper →
sensor passes.

### B4 — Trusted-boundary wrappers get contract + pinning test
Every wrapper (bcrypt-equivalents, JWT-equivalents, DB, clock) gains a
contract line in `spec.md` ("`Ok(true)` means exactly the right password;
Err only on unreadable hash") and at least one pinning test that would fail
if the wrapper's meaning drifted. Mirrors the trial's `trusted_boundary.rs`
(10 tests, one per signed acceptance).

*Testable:* fixture repo adds an unpinched wrapper → `/rig:test` lists the
wrapper as trusted-but-untested; PR template shows the gap.

### B5 — Implication behaviours need the converse, or a recorded waiver
For every Bₙ phrased as "only X then Y" (an implication), rig requires either
a converse property/test ("and always with X") or a waiver recording that the
converse is not required. An agent left alone picks one direction silently;
PDD Kit asks; rig asks too. Property-based tests are the cheap enforcer
(one `hypothesis`/`fast-check` property encodes both directions).

*Testable:* intent with an implication behaviour and no converse test →
`/rig:plan` emits a decision card before any code exists.

### B6 — Plan phase emits a human-readable intent map
`/rig:plan` produces (and keeps current) a rough-work section at the top of
the plan: a flowchart of the decision core, a spec landscape (what is
checked vs. taken on trust), and an intent map tying each behaviour to (a)
the developer's own words, (b) the spec section that models it, (c) the
tests and mutants that cover it. Mermaid, in-repo, reviewable without
reading the implementation. (PDD Kit's Step 2, minus the IDE.)

*Testable:* run `/rig:plan` on the fixture intent → `plan.md` contains the
three artifacts; each intent-map entry links behaviour → spec section →
test name → mutant ID; deleting a behaviour from `intent.md` and re-planning
removes the entry (the map is generated, not decorative).

### B7 — The PR receipt is a sentence with receipts, not a checklist
`/rig:pr` generates a receipt stating: the frozen spec hash, the mutation
score, per-behaviour `killed-by` receipts, the trusted wrappers accepted,
and the honest epistemic sentence — "for every generated input, against the
frozen spec, with one planted bug per clause killed; the boundary contracts
are pinned by tests, not proved." (Trial pre-committed sentence, adapted to
test-land.)

*Testable:* receipt appears in the PR body; tampering with `spec.md` after
approval (hash change) re-opens approval before the PR can merge.

### B8 — Green-by-weakening is a tier-2 stop
If the only way to make a sensor green is to weaken the spec, delete a
property, drop the converse (B5), or test a side copy, rig escalates to the
person (tier-2 / always-human) instead of allowing a waiver. A change to
`spec.md` after approval is a new spec: new hash, new approval.

*Testable:* fixture where the implementation cannot satisfy B1 as approved →
rig stops with the escalation path, never edits the spec to pass.

## Non-goals / not promised
No machine-checked "for every input" guarantee — the receipt is explicitly
weaker and says so. No claim about timing, UI, or anything not written down.
The mutation score is a floor, not a ceiling: a high score with missing
`killed-by` attributions still blocks.

## Decisions rig must ask before building (expected, like PDD Kit's cards)
- D1: mutation tool per stack — mutmut (Python) / Stryker (TS); pin versions?
- D2: where do adjudications live — `mutations/adjudications.md` vs. PR body?
- D3: does B3 run as a lint (import rules) or an architecture test?
- D4: intent map: regenerated each `/rig:plan` run, or diffed?
