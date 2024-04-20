struct Solution {}

impl Solution {
    pub fn dfs(
        grid: &Vec<Vec<char>>,
        (width, height): (usize, usize),
        (x, y): (usize, usize),
        visited: &mut Vec<Vec<bool>>,
    ) -> () {
        if x >= width || y >= height || visited[y][x] {
            return;
        }

        (*visited)[y][x] = true;

        if grid[y][x] == '0' {
            return;
        }

        Solution::dfs(grid, (width, height), (x + 1, y), visited);
        Solution::dfs(grid, (width, height), (x, y + 1), visited);
        if x > 0 {
            Solution::dfs(grid, (width, height), (x - 1, y), visited);
        }

        if y > 0 {
            Solution::dfs(grid, (width, height), (x, y - 1), visited);
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let width = grid[0].len();
        let height = grid.len();

        let mut number_of_islands = 0;

        let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];

        for y in 0..height {
            for x in 0..width {
                if visited[y][x] {
                    continue;
                }

                if grid[y][x] == '1' {
                    Solution::dfs(&grid, (width, height), (x, y), &mut visited);
                    number_of_islands += 1;
                }

                visited[y][x] = true;
            }
        }

        number_of_islands
    }
}

fn main() {
    let grid = vec![
        vec!['1', '1', '1'],
        vec!['0', '1', '0'],
        vec!['1', '1', '1'],
    ];

    let result = Solution::num_islands(grid);
    println!("Number of islands: {}", result);
}
