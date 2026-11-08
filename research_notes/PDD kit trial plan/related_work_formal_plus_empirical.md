# Related work: combining formal and empirical verification of (agent-)generated code

*Researched 2026-11-08. Companion to the trial in `trial/` and the rig intent
in `../../rig_intent/`. arXiv IDs link to abstracts; dates are submission
months. PDD Kit itself is covered in `method_and_claims.md`.*

## The one-sentence landscape

Since 2023 the field has converged on **closed loops** — generate, check with
a machine, feed the failure back — but the 2026 frontier has moved past "did
the verifier accept it" to **"did the check actually bind to what you meant"**,
which is precisely the gap our trial's mutation check and hash gate address.

## Directly on point: the vacuity turn (formal side)

- **Verified but Vacuous: Three Failure Surfaces of LLM-Driven Verified Code
  Generation** — [arXiv:2611.05590](https://arxiv.org/abs/2611.05590) (2026-11,
  days old). Argues "verified" collapses three separable properties:
  **A** verifier acceptance, **B** the proof obligation *binds* to a
  spec-defined predicate, **C** the spec *faithfully captures the informal
  requirement*. Their headline: a pipeline reaching **99.1% verifier
  acceptance on HumanEval with only 1.9% of certificates binding to a
  spec-defined predicate**. This is the academic version of three things our
  trial found by hand: the false-proof list (proof of a side copy = A without
  B), the vacuous-clause risk that motivated mutants M1–M9 (= B), and the
  hash-gated human spec approval (= C — only a person can attest fidelity).
  **Closest published kin to this research.**
- **How Stable Are LLM-Generated Proofs Under Program Refactoring?** —
  [arXiv:2610.36328](https://arxiv.org/abs/2610.36328) (2026-10). Stress-tests whether verified code stays
  verified under refactors — the re-run-without-a-model property our
  `verify.sh` requires, attacked from the proof side.
- **Grading the Graders: Verification Autonomy Levels (L0–L5)** —
  [arXiv:2608.19009](https://arxiv.org/abs/2608.19009) (2026-08). A taxonomy for how much of verification can be
  delegated to the model; useful vocabulary for rig's autonomy rungs.

## Closed loops: formal verification as the judge

- **Clover: Closed-Loop Verifiable Code Generation** —
  [arXiv:2310.17807](https://arxiv.org/abs/2310.17807) (Meta, 2023-10). The
  canonical predecessor: consistency checking among **code, docstrings, and
  formal annotations** (Dafny), with the verifier as a filter. PDD Kit's
  three-artifact structure (intent → spec → proof) and Clover's
  three-artifact consistency check (docstring → annotation → code) are the
  same shape; Clover lacks the human approval gate and the mutation check.
- **Formal-Method-Guided Vibe Coding (Forge)** —
  [arXiv:2606.22413](https://arxiv.org/abs/2606.22413) (2026-06). Closes the
  loop for **safety-critical** stacks (DO-178C / IEC 61508 / ISO 26262) using
  established toolchains instead of verification-native languages — the same
  "you can't have Verus, so approximate it" move as our rig intent, under
  certification constraints.
- **Vero: Can AI Agents Build Formally Verified Software Repositories?**
  [arXiv:2608.13522](https://arxiv.org/abs/2608.13522) (2026-08),
  **P³: Joint Program-and-Proof Planning**
  [arXiv:2608.09277](https://arxiv.org/abs/2608.09277) (2026-08),
  **Self-Spec Verifiable Code Generation**
  ([arXiv:2609.39568](https://arxiv.org/abs/2609.39568), 2026-09),
  **Agent-Guided Tree Search for FV** (2026). The end-to-end wave: agents
  planning code *and* proof together; Self-Spec's finding matters for rig —
  stage-wise spec quality does not compose into end-to-end success, i.e. the
  plan→build boundary must carry its own gate (our frozen-hash rule).

## The empirical side: properties, contracts, mutation

- **LLM-generated property-based tests**: *Understanding the Characteristics
  of LLM-Generated Property-Based Tests in Exploring Edge Cases* (2025-10),
  *PropGen* (2026-04, mobile), *LLM-based Property-based Test Generation for
  Guardrailing Cyber-Physical Systems* (2025-05), *Towards LLM-assisted
  Property Generation for Solidity* (2026-07). Consistent finding: LLMs write
  plausible properties with weak oracles and edge-case blind spots — which is
  exactly what a mutation check audits. This is the "Verus-lite" layer for
  Python/TS in the rig intent (B5's converse rule enforced by
  hypothesis/fast-check).
- **Mutation testing + LLM**: *Beyond Rule-Based Mutation Testing:
  Test-Aware Mutant Generation Using LLMs* (2026-09), *Mubric: Mutation
  Testing-Guided Rubric Generation for LLM Evaluation* (2026-09). Two
  directions: LLMs making mutation testing smarter, and mutation testing
  making LLM evaluation honest. PDD Kit's in-loop mutation check sits at
  their intersection; our B1/B2 (`killed-by` receipts, adjudication) port it
  to plain test suites.
- **Dynamic invariant detection**: *Template-Free Dynamic Invariant
  Detection* (2026-10) — the Daikon lineage: inferring formal invariants
  from *running* code, the empirical→formal direction. Paired with LLM
  spec-generation quality work (**How Powerful are LLMs in Generating Formal
  Program Specifications?**, **KBSpec**, **BODHI** for kernels, all 2026),
  the field is building both bridges between the two verification worlds.

## Deep prior art (pre-LLM, worth citing in the rig DESIGN.md)

- **Daikon** (Ernst et al.) — empirical runs yield formal invariants.
- **Pex / Code Contracts** (MSR, ~2008) — contracts written first,
  dynamic-symbolic-execution generates the tests that try to violate them:
  formal-to-empirical, human-in-the-loop, deterministic — PDD Kit's
  great-grandparent on the .NET line.
- **Design by Contract** (Eiffel, Meyer 1986) — the original "the contract is
  the artifact"; runtime checking, no prover, and it shipped in production
  for decades. The rig intent is best read as DbC + mutation receipts +
  LLM-era spec discipline.

## What remains open (and where this research fits)

1. **The intent map as a research object.** Nobody in the papers above
   publishes the human-readable bridge artifact — the tied-to-your-words
   table linking behaviour → spec section → tests → mutants (PDD Kit's
   Figure 4 is the only example found). It is also the piece rigs can build
   today with zero formal methods. **This is the most novel transferable
   artifact.**
2. **Vacuity auditing for test-land.** "Verified but Vacuous" audits proofs;
   the test-side equivalent — per-behaviour mutant receipts with
   adjudication — is not published as a method. B1/B2 of the rig intent are
   a concrete proposal.
3. **Two-sided receipt language.** No benchmark asks for the honest sentence
   ("for every generated input… one planted bug per clause killed"). PR
   receipts (B7) operationalize it.

## Suggested next reading order

1. arXiv:2611.05590 (Verified but Vacuous) — the vocabulary for why green ≠ done
2. arXiv:2310.17807 (Clover) — the loop, formal side
3. arXiv:2606.22413 (Forge) — the loop when you can't change languages
4. PDD Kit's P²/P³-style end-to-end papers if rig grows proof backends later
