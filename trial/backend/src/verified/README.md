# verified/ — spec-first module (NOT yet wired into the crate)

`login.rs` is the spec template: contracts + ghost definitions first, bodies
as `unimplemented!()`. It is deliberately **excluded from the crate build**
(see the commented `pub mod verified;` in `../lib.rs`) because compiling it
under Verus *inside* the backend crate — while `cargo test` still passes — is
build-plan **Task 1** and part of what the trial tests.

Progression:

1. `cargo test` passes without this module (behaviour pinned by the existing
   handler + tests/trusted_boundary.rs).
2. Task 1: wire the module, get it building under Verus, change no contract.
3. Spec approval: hash `login.rs` (shasum -a 256) into intent/decisions.md
   **while bodies are still `unimplemented!()`**.
4. Proof stage: fill bodies; `verus` must accept; then run mutations/.

`mod_note.rs` is a placeholder so the directory exists in git before Task 1;
delete it when the module is wired.
