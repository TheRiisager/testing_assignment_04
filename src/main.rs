use crate::{board::Board, input::read_int, win_conditions::has_won};

mod board;
mod input;
mod win_conditions;

fn main() {
    println!("Welcome player! I will assign you crosses, as i am too lazy to write a selector");
    println!("The board is laid out as such: \n# | # | #\n# | # | #\n# | # | #");
    println!("enter the row and column number you wish to place a mark at, when it is your turn");

    let mut board = Board {
        rows: [
            [' ', ' ', ' '],
            [' ', ' ', ' '],
            [' ', ' ', ' '],
        ]
    };
    while true {
        user_move(&mut board);
        if has_won(board.rows, 'X') {
            println!("YOU WIN!");
            println!("{}", board);
            break;
        }
        board.cpu_turn();
        if has_won(board.rows, 'O') {
            println!("YOU LOSE!");
            println!("{}", board);
            break;
        }
        println!("{}", board);
    }
}

fn user_move( board: &mut Board) {
    println!("Which row would you like to place a mark at?");
    let row = read_int();
    println!("Which column would you like to place a mark at?");
    let column = read_int();
    if !board.make_move(row, column, 'X') {
        println!("Invalid move! Try again!");
        return user_move(board)
    }
}
