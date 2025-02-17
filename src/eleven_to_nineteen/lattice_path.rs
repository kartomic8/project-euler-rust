fn lattice_path(n: usize) -> u64 {
    let mut grid: Vec<Vec<u64>> = Vec::new();

    for _i in 0..n {
        grid.push(Vec::new());
    }

    for i in 0..n {
        for j in 0..n {
            let mut incoming = 0;
            if j == 0 && i == 0 {
                incoming = 0;
            } else if j == 0 || i == 0 {
                incoming += 1;
            } else {
                incoming += grid[i - 1][j];
                incoming += grid[i][j - 1];
            }
            grid[i].push(incoming);
        }
    }

    grid[n - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let path = lattice_path(2);
        assert_eq!(path, 2);
    }
    #[test]
    fn test_3() {
        let path = lattice_path(3);
        assert_eq!(path, 6);
    }

    #[test]
    fn test_4() {
        let path = lattice_path(4);
        assert_eq!(20, path)
    }
    #[test]
    fn test_5() {
        let path = lattice_path(21);
        println!("{}", path);
    }
}
