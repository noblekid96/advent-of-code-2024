pub fn inside(x: usize, y: usize, m: usize, n: usize) -> bool {
    return x >= 0 && x < m && y >= 0 && y < n
}

pub fn process_part1(input: &str) -> String {
    let mut result: usize = 0;

    let grid: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|line| line.chars().collect())
        .collect();

    let m = grid.len();
    let n = grid[0].len();

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] != 'X' {
                continue;
            }
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue
                    }

                    let mut temp_string = "".to_string();
                    for k in 0..4 {
                        let nx = i as isize + (k*dx);
                        let ny = j as isize + (k*dy);
                        if !inside(nx as usize,ny as usize,m,n) {
                            break;
                        }
                        temp_string.push(grid[nx as usize][ny as usize]);
                    }

                    if temp_string.len() == 4 && temp_string == "XMAS" {
                        result += 1
                    }
                }
            }
        }
    }

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut result: usize = 0;

    let grid: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|line| line.chars().collect())
        .collect();

    let m = grid.len();
    let n = grid[0].len();

    let diagonal_directions: Vec<(isize, isize)> = vec![
        (-1, -1), // Top-Left
        (-1, 1),  // Top-Right
        (1, 1),  // Bottom-Right
        (1, -1),  // Bottom-Left
    ];

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] != 'A' {
                continue;
            }

            let mut temp = String::from("");
            for (dx,dy) in &diagonal_directions {
                let nx = (i as isize + (dx)) as usize;
                let ny = (j as isize + (dy)) as usize;

                if inside(nx, ny, m , n) {
                    temp.push(grid[nx as usize][ny as usize]);
                }
            }

            if temp.len() == 4 && (temp == "SSMM" || temp == "MSSM" || temp == "MMSS" || temp == "SMMS") {
                result += 1
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";
        let result = process_part1(input);
        assert_eq!(result, "18");
    }

    #[test]
    fn it_works_2() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let result = process_part2(input);
        assert_eq!(result, "9");
    }
}
