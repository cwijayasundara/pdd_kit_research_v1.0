# pdd_kit_research_v1.0 — testing the "Proof-Driven Development" claim

In November 2026, GitHub Next and Microsoft Research announced **PDD Kit**:
an agent harness inside [Chopin](https://githubnext.com/projects/chopin/)
where the agent must first get an approved formal spec, prove it with
[Verus](https://github.com/verus-lang/verus), survive a mutation check, and
only then ship. This repo is a hands-on investigation of that claim: what it
says, what it actually gives you, and how much of it survives *without* the
proprietary parts.

**Status of the product itself:** PDD Kit has no public install, license, or
repo (checked 2026-11-07/08). It runs as a partner inside Chopin's research
preview. One correction discovered along the way: **Chopin is self-hostable**
([githubnext/chopin](https://github.com/githubnext/chopin)), but the PDD Kit
partner code is not in its 3,131-file tree — so the full loop cannot be run
outside GitHub today.

## What's in here

| Path | What it is |
|---|---|
| [`trial/`](trial/) | **Experiment B** — a Verus-only reconstruction of the published Pet Store login verification: a real Axum + SQLite app whose auth handler is specified in Verus, with a frozen spec hash, trusted-boundary tests, and a planted-bug-per-clause protocol. Gates green at last run. |
| [`research_notes/PDD kit trial plan/`](research_notes/PDD%20kit%20trial%20plan/) | Source-grounded notes: the exact claims and marketing gaps, the trial design, correct-usage rules, the Pet Store reference spec, and a survey of formal + empirical verification research for agent-generated code. |
| [`rig_intent/`](rig_intent/) | The transfer: eight numbered behaviours porting PDD's discipline into a Python/TS agent harness (rig) that has no proof checker — per-behaviour mutant receipts, a pure-core sensor, the intent map. |
| [`reports/`](reports/) | The long-form write-up of the trial plan. |

## The claim tested (pre-committed before the work)

> A Rust auth handler on the production path meets an approved Verus spec
> for every input, with the password check and the account lookup explicitly
> unproved and tested — and one planted bug per `ensures` clause is rejected.

**Honest labeling:** this is *not* a PDD Kit trial — PDD Kit cannot be
installed. It is a reconstruction with the same published checker (Verus
0.2026.11.02.8bf68ee), the same spec shape (eight clauses on an async
`login_flow`), and the same protocol gates.

## What the reconstruction found

1. **The discipline decomposes.** The valuable parts of PDD Kit — spec-first
   with a frozen hash, an explicit trusted boundary, two-sided (iff)
   properties, per-clause mutation checks, a deterministic no-model gate —
   all survive without an SMT prover. Only "for every input" needs the prover.
2. **The checker forces the design.** Compiling the draft spec under Verus
   surfaced six formalization decisions the prose spec never mentioned
   (trial decision card D8): `external_fn_specification` impossible for
   own-crate fns, `unimplemented!()` bodies *fail* verification, `matches`
   binders don't cross `==>`, `sqlx::Error` unrepresentable, opaque newtypes
   for private-field types, role modeled as its JWT string form.
3. **A clause is only *checked* once its body is proved.** `external_body`
   placeholders mean the contract is assumed. Mutation checks (M1–M9) are
   only valid against the aimed function's real proof.
4. **A surviving mutant is a finding, not a failure.** The smoke file's
   `>=`→`>` mutant survived — correctly, it still implements max. Only
   adjudication (vacuous clause vs. non-bug) distinguishes a weak spec from
   a wrong mutant. Published the same week as
   [arXiv:2611.05590](https://arxiv.org/abs/2611.05590) (*Verified but
   Vacuous*): 99.1% verifier acceptance, 1.9% of proofs binding to a spec.
5. **The intent map is the most portable artifact.** PDD Kit's human-readable
   bridge — each requirement tied to the developer's own words, the spec
   section that models it, and the tests that cover it — needs no formal
   methods at all ([`rig_intent/intent-map-example.md`](rig_intent/intent-map-example.md)).

## Reproduce

```bash
git clone https://github.com/cwijayasundara/pdd_kit_research_v1.0
cd pdd_kit_research_v1.0/trial

curl https://sh.rustup.rs | sh                    # Rust 1.99.1 toolchain
scripts/install-verus.sh                          # prebuilt Verus + Gatekeeper fix
cargo test -p backend                             # 10/10 trusted-boundary + e2e tests
scripts/verify.sh                                 # deterministic gate: smoke + crate verification
```

`scripts/verify.sh` involves no model in its pass/fail: Verus (Z3) checks
the committed files, and the exact command is pinned in
[`trial/intent/decisions.md`](trial/intent/decisions.md) (card D-toolchain)
for a second-machine re-run.

## Where it stopped / what's next

- Trial protocol: decision cards D1–D8 drafted, spec frozen at
  `shasum -a 256 = 3dfb3c62…` with all proved functions still
  `unimplemented!()`; awaiting human sign-offs (D2/D5/D6/D7/D8) before the
  proof stage writes bodies.
- After the proof stage: the nine-mutant check, the deterministic re-run on
  a second machine, and the clause-change test (expiry 24h → 1h must break
  the old proof).
- Harness side: the incorporations are filed as
  [`rig_v1/docs/superpowers/specs/2026-11-08-intent-map-incorporations.md`](https://github.com/cwijayasundara/rig_v1/blob/main/docs/superpowers/specs/2026-11-08-intent-map-incorporations.md).
