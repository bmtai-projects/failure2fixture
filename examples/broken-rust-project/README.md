# broken-rust-project

Demo target for Failure2Fixture.

`discount()` divides by 10 instead of 100, so 10% off 1000 gives 0 instead of 900. One test fails, one passes.

Fix: change `/ 10` to `/ 100` in `src/lib.rs`.
