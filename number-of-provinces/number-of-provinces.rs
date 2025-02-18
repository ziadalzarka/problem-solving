struct Solution {}

impl Solution {
    pub fn dfs(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, i: usize) -> bool {
        if visited[i] {
            return false;
        }

        visited[i] = true;

        for j in 0..is_connected[i].len() {
            if is_connected[i][j] == 1 && !visited[j] {
                Solution::dfs(is_connected, visited, j);
            }
        }

        true
    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![false; is_connected.len()];
        let mut count = 0;

        for i in 0..is_connected.len() {
            if Solution::dfs(&is_connected, &mut visited, i) {
                count += 1;
            }
        }

        count
    }
}

fn main() {
    dbg!(Solution::find_circle_num(vec![
        vec![1, 1, 0],
        vec![1, 1, 0],
        vec![0, 0, 1]
    ]));
}
