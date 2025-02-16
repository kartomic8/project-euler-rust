pub fn run<F>(name: &str, f: F)
where
    F: Fn() -> (),
{
    use std::time::Instant;
    let now = Instant::now();
    println!("Running case: {name}");
    f();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
