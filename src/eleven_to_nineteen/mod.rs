mod highly_divisible_triangular_number;
mod large_sum;
mod largest_product_in_a_grid;

pub fn solve() {
    largest_product_in_a_grid::solve_for_official_input();
    println!(
        "Solution 12: {}",
        highly_divisible_triangular_number::solve()
    );
    large_sum::solve_with_default_input();
}
