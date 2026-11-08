// Self-contained smoke test for the Verus install. Not part of the crate.
//
//   ./verus scripts/verus_smoke.rs        must exit 0.
//
// Mutation sanity check for the toolchain itself: change the else branch to
// return `a` as well (`if a >= b { a } else { a }`) — Verus must then FAIL the
// `ensures` clause. If a mutant of even this toy passes, the checker or the
// invocation is wrong, and nothing downstream can be trusted.
// (Note: `>=` to `>` in the condition is NOT a bug — it still implements max;
// that distinction is the whole game.)

use vstd::prelude::*;

verus! {

fn max(a: u64, b: u64) -> (r: u64)
    ensures
        r == a || r == b,
        r >= a,
        r >= b,
{
    if a >= b { a } else { b }
}

fn main() {
}

}
