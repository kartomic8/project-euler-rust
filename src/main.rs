#![allow(dead_code)]
mod benchmark;
mod eleven_to_nineteen;
mod first_ten;
mod prime;
mod scratch_pad;

fn main() {
    first_ten::solve();
    eleven_to_nineteen::solve();
}
