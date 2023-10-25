use generator::generate_sudoku;

use crate::solver::solve_sudoku;

mod generator;
mod solver;

pub fn print_board(board: &Vec<Vec<i32>>) {
    for y in 0..board.len() {
        if y % 3 == 0 && y != 0 {
            println!("- - - - - - - - - - - -");
        }

        for x in 0..board[0].len() {
            if x % 3 == 0 && x != 0 {
                print!(" | ");
            }

            if x == 8 {
                println!("{}", board[y][x]);
            } else {
                print!("{} ", board[y][x]);
            }
        }
    }
}

fn main() {
    let mut board = generate_sudoku();
    print_board(&board);
    println!("\n");
    solve_sudoku(&mut board);
    print_board(&board);
}
