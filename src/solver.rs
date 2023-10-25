pub fn solve_sudoku(board: &mut Vec<Vec<i32>>) -> bool {
    // Find the next empty cell
    if let Some((x, y)) = find_next_empty(&board) {
        // Try inserting a number (1-9) into the cell
        for i in 1..10 {
            // Check if the given number (1-9) is valid
            if is_num_valid(board, i, (x, y)) {
                board[y][x] = i;

                // If there is a solution for the next empty cell, return true (solved)
                if solve_sudoku(board) {
                    return true;
                }

                // If a solution for the next empty cell can't be found, reset cell back to 0 and continue to the next number (1-9)
                board[y][x] = 0;
            }
        }
        // If any number (1-9) does not work, return false (unsolved)
        return false;
    }
    // If there are no empty cells, return true (solved)
    return true;
}

pub fn find_next_empty(board: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
    // Iterate over the board and find the next empty cell
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == 0 {
                return Some((x, y));
            }
        }
    }
    return None;
}

pub fn is_num_valid(board: &Vec<Vec<i32>>, n: i32, pos: (usize, usize)) -> bool {
    // Check if the given number (n) exists within the current row
    for i in 0..board[0].len() {
        if board[pos.1][i] == n && pos.0 != i {
            return false;
        }
    }

    // Check if the given number (n) exists within the current column
    for i in 0..board.len() {
        if board[i][pos.0] == n && pos.1 != i {
            return false;
        }
    }

    // Check if the given number (n) exists elsewhere within its box
    let box_x = pos.0 / 3;
    let box_y = pos.1 / 3;

    for y in box_y * 3..box_y * 3 + 3 {
        for x in box_x * 3..box_x * 3 + 3 {
            if board[y][x] == n && (x, y) != pos {
                return false;
            }
        }
    }

    // If the number (n) is not a duplicate, it is valid
    return true;
}
