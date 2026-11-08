# Mutants — one planted bug per `ensures` clause

The mechanical guard against "vibe verification": a spec clause that
verifies but rules out nothing is worthless, and a green check on it is
worse than no check. So: plant ONE bug per clause, in a scratch copy, and
Verus must REJECT each mutant on the clause it targets. A mutant that
verifies means the clause constrains nothing — it comes back as a GAP, not
a green check.

## Procedure

1. From the trial root: `git worktree add ../pdd-scratch HEAD` (or copy the
   tree — never mutate the approved spec file itself).
2. Apply exactly one mutant below (edit `login_impl`/`login.rs` bodies).
3. Run `scripts/verify.sh`. EXPECT: verification FAILS, pointing at the
   aimed clause.
4. Record date + result in the table. Remove the worktree. Next mutant.
5. If a mutant passes: do NOT delete the clause. Record it as a gap, and
   fix the SPEC (a human decision), then re-approve per the hash gate.

## The mutants

| # | Aimed at | Planted bug (pick ONE per run) | Expected Verus verdict |
|---|----------|-------------------------------|------------------------|
| M1 | clause 3 ("right password always works") | refuse everyone: every request returns 400 | FAIL on clause 3 (clauses 1 and 2 still pass — that is the teaching point: clause 3 is the only thing ruling out a locked door) |
| M2 | clause 2 (same-account tie) | check the password against the account looked up, but sign the token with a DIFFERENT account's id/role (Bob's password, Alice's token) | FAIL on clause 2 — the wiring bug tests rarely think to look for |
| M3 | clause 2 ("only with the right password") | issue a token on wrong password too | FAIL on clause 2 |
| M4 | clause 7 (admin role only via admins-by-username) | swap the order: check username BEFORE email, so email no longer wins | FAIL on clause 7/8 (the quiz case mints an admin token) |
| M5 | clause 4 (unknown account ⇒ 401) | return 400 instead of 401 for an unknown account | FAIL on clause 4 |
| M6 | clause 5 (wrong password ⇒ 401) | return 500 for a wrong password | FAIL on clause 5 |
| M7 | clause 6 (unreadable hash ⇒ 500) | treat bcrypt Err as 401 | FAIL on clause 6 |
| M8 | expiry (`expiry_from_now` ensures) | expiry = now + 3,600 | FAIL on the clock wrapper clause |
| M9 | clause 8 (email token ⇒ customer role) | copy the role from the request body if present, else the table | FAIL on clause 8 |

## What must NOT happen

- Deleting or weakening a clause to make a mutant pass. That is the
  documented forbidden move.
- Counting "cargo test passes" as a mutant verdict. Only the deterministic
  Verus run decides.
- Running mutants against a copy the route does not call.

## Results log

| Mutant | Date | Verus verdict | Failed on the aimed clause? (y/n) |
|--------|------|---------------|------------------------------------|
| — | — | — | — |
