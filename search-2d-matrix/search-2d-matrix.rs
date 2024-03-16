fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let width = matrix[0].len();
    let height = matrix.len();

    let mut left = 0;
    let mut right = width * height - 1;

    while left <= right {
        let middle = (right + left) / 2;

        let x = middle % width;
        let y = middle / width;

        let num = matrix[y][x];

        if target == num {
            return true;
        } else if target > num {
            left = middle + 1;
        } else if target < num {
            if middle == 0 {
                break;
            }
            right = middle - 1;
        }
    }

    false
}

fn main() {
    println!(
        "{}",
        search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        )
    );
}
