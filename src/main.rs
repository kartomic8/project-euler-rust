#![allow(dead_code)]
mod benchmark;
mod eleven_to_nineteen;
mod first_ten;
mod prime;

fn main() {
    first_ten::solve();
    eleven_to_nineteen::solve();
}
