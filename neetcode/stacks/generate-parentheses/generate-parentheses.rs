fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut stack: Vec<&str> = vec![];
    let mut result: Vec<String> = vec![];

    fn backtrack(
        open_count: i32,
        closed_count: i32,
        n: i32,
        stack: &mut Vec<&str>,
        result: &mut Vec<String>,
    ) {
        if open_count == closed_count && open_count == n {
            result.push(stack.join(""));
        }

        if open_count < n {
            stack.push("(");
            backtrack(open_count + 1, closed_count, n, stack, result);
            stack.pop();
        }

        if closed_count < open_count {
            stack.push(")");
            backtrack(open_count, closed_count + 1, n, stack, result);
            stack.pop();
        }
    }

    backtrack(0, 0, n, stack.as_mut(), result.as_mut());

    result
}

fn main() {
    println!("{}", generate_parenthesis(3).join(","));
}
