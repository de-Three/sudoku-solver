use rand::{rngs::ThreadRng, Rng};

use crate::solver::{find_next_empty, is_num_valid, solve_sudoku};

pub fn generate_sudoku() -> Vec<Vec<i32>> {
    let mut board = create_empty_board();
    let mut rng = rand::thread_rng();

    generate_solution(&mut rng, &mut board);
    trim_solution(&mut rng, &mut board);

    return board;
}

fn create_empty_board() -> Vec<Vec<i32>> {
    let mut board: Vec<Vec<i32>> = Vec::new();

    for y in 0..9 {
        board.push(Vec::new());
        for _x in 0..9 {
            board[y].push(0);
        }
    }

    return board;
}

fn generate_solution(rng: &mut ThreadRng, board: &mut Vec<Vec<i32>>) -> bool {
    if let Some((x, y)) = find_next_empty(&board) {
        // Generate a random number (1-9) and try to insert it into a given position
        let i = rng.gen_range(1..10);

        // If the number is in a valid position, then check if it can exist in a valid sudoku
        // If it can, then recurse
        // If not, then revert the number to zero
        if is_num_valid(board, i, (x, y)) {
            board[y][x] = i;

            if solve_sudoku(board) {
                return true;
            }

            board[y][x] = 0;
        }

        return false;
    }
    return true;
}

fn trim_solution(rng: &mut ThreadRng, board: &mut Vec<Vec<i32>>) {
    for _i in 0..rng.gen_range(32..64) {
        let random_x = rng.gen_range(0..9);
        let random_y = rng.gen_range(0..9);
        board[random_x][random_y] = 0;
    }
}
