#![allow(dead_code)]

fn nqueens(n: i32) -> i32 {
    nqueens_recur(n, 0, &mut vec![])
}

fn nqueens_recur(n: i32, level: i32, queens: &mut Vec<i32>) -> i32 {
    if level == n {
        return 1;
    }

    let possible_moves: Vec<i32> = (0..n)
        .filter(|new_queen| is_valid(new_queen, queens))
        .collect();

    possible_moves
        .into_iter()
        .map(|new_queen| {
            queens.push(new_queen);
            let result = nqueens_recur(n, level + 1, queens);
            queens.pop();
            result
        })
        .sum()
}

fn is_valid(new_queen: &i32, queens: &Vec<i32>) -> bool {
    let mut levels_apart = 1;
    for queen in queens.iter().rev() {
        if queen == new_queen
            || queen + levels_apart == *new_queen
            || queen - levels_apart == *new_queen
        {
            return false;
        }
        levels_apart += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wikipedia_results() {
        assert_eq!(nqueens(1), 1);
        assert_eq!(nqueens(2), 0);
        assert_eq!(nqueens(3), 0);
        assert_eq!(nqueens(4), 2);
        assert_eq!(nqueens(5), 10);
        assert_eq!(nqueens(6), 4);
        assert_eq!(nqueens(7), 40);
    }
}
