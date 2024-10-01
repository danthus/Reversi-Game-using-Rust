use std::{io, simd::usizex1};

static DECLARATION: &str = "ECE1724 Rust Assignment 1 Reversi Board Game by Amos Zhou";

fn init_board(board: &mut [[i32; 8]; 8]) {
    board[3][3] = 1;
    board[3][4] = 0;
    board[4][3] = 0;
    board[4][4] = 1;
}

fn print_board(board: &[[i32; 8]; 8]) {
    println!("  a b c d e f g h");

    for (i, row) in board.iter().enumerate() {
        print!("{} ", (i+97) as u8 as char);
        for &cell in row.iter(){
            let piece = match cell {
                -1 => ".",
                0 => "B",
                1 => "W",
                _ => "?",
            };
            print!("{} ", piece);
        }
        println!();
    }
}

fn check_placing(row: usize, col: usize, player: i32, board: &[[i32; 8]; 8]) -> bool {
    if board[row][col] != -1 {
        return false;
    }

    for irow in -1..=1 {
        for icol in -1..=1 {
            if irow==0 && icol==0{
                continue;
            }
            else if check_line(row as isize, col as isize, irow, icol, player, board) {
                return true;
            }
        }
    }

    false
}

fn check_line(row: isize, col: isize, irow: isize, icol: isize, player: i32, board: &[[i32; 8]; 8]) -> bool {
    let opponent = if player == 1 {0} else {1};

    let mut opponent_exist = false;
    let mut i: isize = row + irow;
    let mut j: isize = col + icol;

    while i>=0 && i<8 && j>=0 && j<8 {
        let temp = board[i as usize][j as usize];
        if temp == -1 {
            return false
        }
        else if temp == player 
        {
            return opponent_exist;
        }
        else if temp == opponent {
            opponent_exist = true;
        }
        else {
            println!("should not go here, error in check_line()");
            return false;
        }

        i += irow;
        j += icol;
    }

    false
}

fn place_piece(row: usize, col: usize, player: i32, board: &mut [[i32; 8]; 8]) {
    board[row][col] = player;

    for irow in -1..=1 {
        for icol in -1..=1 {
            if irow==0 && icol==0{
                continue;
            }
            else if check_line(row as isize, col as isize, irow, icol, player, board) {
                reverse_pieces(row as isize, col as isize, irow, icol, player, board);
            }
        }
    }

}

fn reverse_pieces(row: isize, col: isize, irow: isize, icol: isize, player: i32, board: &mut [[i32; 8]; 8]) {
    let mut i: isize = row + irow;
    let mut j: isize = col + icol;
    
    while i>=0 && i<8 && j>=0 && j<8 {
        if board[i][j] == player {
            break;
        }

        board[i][j] = player;

        i += irow;
        j += icol;
    }
}

fn check_valid_moves(player: i32, board: &[[i32; 8]; 8]) -> bool {
    for row in 0..=8 {
        for col in 0..=8 {
            if check_placing(row, col, player, board) {
                return true;
            }
        }
    }
    false
}

fn check_win(board: &[[i32; 8]; 8]) {
    
    let (w_pieces,b_pieces) = count_pieces(board);
    if w_pieces > b_pieces {
        println!("White wins by {} points!", {w_pieces - b_pieces});
    }
    else if b_pieces > w_pieces {
        println!("Black wins by {} points!", {b_pieces - w_pieces});
    }
    else {
        println!("Draw!");
    }
}

fn count_pieces(board: &[[i32; 8]; 8]) -> (isize, isize) {
    let mut w_pieces = 0;
    let mut b_pieces = 0;

    for row in 0..=8 {
        for col in 0..=8 {
            if board[row][col] == 1 {
                w_pieces += 1;
            }
            else if board[row][col] == -1 {
                b_pieces += 1;
            }
        }
    }

    (w_pieces, b_pieces)
}

fn main() {

    // false is black, true is white
    let mut player_str: &str = "B";
    let mut player_int = 0;
    
    // -1 for empty, 1 for white, 0 for black
    let mut board: [[i32; 8]; 8] = [[-1; 8]; 8];
    init_board(&mut board);
    print_board(&board);

    loop {

        println!("Enter move for colour {} (RowCol): ", player_str);
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        if input.len() != 2 {
            println!("Invalid input. Try again.");
            continue;
        }

        let row = (input.chars().nth(0).unwrap() as usize) - 97;
        let col = (input.chars().nth(1).unwrap() as usize) - 97;

        if row > 7 || col > 7 {
            println!("Invalid input. Try again.");
            continue;
        }

        if !check_placing(row, col, player_int, &board) { 
            println!("Invalid move. Try again.");
            continue;
        }

        // valid placing
        place_piece(row, col, player_int, &mut board);

        player_str = if player_str == "B" {"W"} else {"B"};
        player_int = if player_int == 0 {1} else {0};


        if !check_valid_moves(player_int, &board) {
            println!("{} player has no valid move.", player_str);
            player_str = if player_str == "B" {"W"} else {"B"};
            player_int = if player_int == 0 {1} else {0};
            
            if !check_valid_moves(player_int, &board) {
                println!("{} player has no valid move.", player_str);
                check_win(&board);
            }
        }

        break;
    }

}
