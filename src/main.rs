use std::io;
use std::io::Write;
use std::process;

// initial board configutation
fn init_board(board: &mut [[i32; 8]; 8]) {
    board[3][3] = 1;
    board[3][4] = 0;
    board[4][3] = 0;
    board[4][4] = 1;
}

// function to visualize board data
fn print_board(board: &[[i32; 8]; 8]) {
    println!("  abcdefgh");

    for (i, row) in board.iter().enumerate() {
        print!("{} ", (i+97) as u8 as char);
        for &cell in row.iter(){
            let piece = match cell {
                -1 => ".",
                0 => "B",
                1 => "W",
                _ => "?",
            };
            print!("{}", piece);
        }
        println!();
    }
}

// check the user input is valid to place
fn check_placing(row: usize, col: usize, player: i32, board: &[[i32; 8]; 8]) -> bool {
    if board[row][col] != -1 {
        return false;
    }

    // check the surronding 8 positions
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

// check the given position has opponent piece and player piece exit on the continuous line
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

// place the piece and reverse other pieces
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

// reverse other pieces
fn reverse_pieces(row: isize, col: isize, irow: isize, icol: isize, player: i32, board: &mut [[i32; 8]; 8]) {
    let mut i: isize = row + irow;
    let mut j: isize = col + icol;
    
    while i>=0 && i<8 && j>=0 && j<8 {
        // reverse until a player's piece is met
        if board[i as usize][j as usize] == player {
            break;
        }

        board[i as usize][j as usize] = player;

        i += irow;
        j += icol;
    }
}

// check the given player has valid moves
fn check_valid_moves(player: i32, board: &[[i32; 8]; 8]) -> bool {
    for row in 0..8 {
        for col in 0..8 {
            if check_placing(row, col, player, board) {
                return true;
            }
        }
    }
    false
}

// check winning condition and print
fn check_win(board: &[[i32; 8]; 8]) {
    
    let (w_pieces,b_pieces) = count_pieces(board);
    if w_pieces > b_pieces {
        print!("White wins by {} points!", {w_pieces - b_pieces});
    }
    else if b_pieces > w_pieces {
        print!("Black wins by {} points!", {b_pieces - w_pieces});
    }
    else {
        print!("Draw!");
    }
}

// count while and black pieces
fn count_pieces(board: &[[i32; 8]; 8]) -> (isize, isize) {
    let mut w_pieces = 0;
    let mut b_pieces = 0;

    for row in 0..8 {
        for col in 0..8 {
            if board[row][col] == 1 {
                w_pieces += 1;
            }
            else if board[row][col] == 0 {
                b_pieces += 1;
            }
        }
    }

    (w_pieces, b_pieces)
}

fn main() {
    let mut player_str: &str = "W"; // "B" for black, "W" for white, start with "B"
    let mut player_int = 1; // 0 for black, 1 for white

    // -1 for empty, 1 for white, 0 for black, start with all -1
    let mut board: [[i32; 8]; 8] = [[-1; 8]; 8];
    init_board(&mut board);

    // main game lopp starts
    loop {
        print_board(&board);

        player_str = if player_str == "B" {"W"} else {"B"};
        player_int = if player_int == 0 {1} else {0};

        if !check_valid_moves(player_int, &board) {
            // if no valid move, switch back to previous player
            println!("{} player has no valid move.", player_str);
            // flag = false;
            player_str = if player_str == "B" {"W"} else {"B"};
            player_int = if player_int == 0 {1} else {0};

            // check previous player has valid moves or not
            if !check_valid_moves(player_int, &board) {
                // if both players have no valid moves, finish the game
                println!("{} player has no valid move.", player_str);
                check_win(&board);
                process::exit(0);
            }
        }

        print!("Enter move for colour {} (RowCol): ", player_str);
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let input = input.trim();

        // special cmd to exit program
        if input == "!q" {
            process::exit(0);
        }

        if input.len() != 2 {
            println!("Invalid input. Try again.");
            player_str = if player_str == "B" {"W"} else {"B"};
            player_int = if player_int == 0 {1} else {0};
            continue;
        }

        let row = (input.chars().nth(0).unwrap() as usize) - 97;
        let col = (input.chars().nth(1).unwrap() as usize) - 97;

        if row > 7 || col > 7 {
            println!("Invalid input. Try again.");
            player_str = if player_str == "B" {"W"} else {"B"};
            player_int = if player_int == 0 {1} else {0};
            continue;
        }

        // check the placing is valid or not
        if !check_placing(row, col, player_int, &board) { 
            println!("Invalid move. Try again.");
            player_str = if player_str == "B" {"W"} else {"B"};
            player_int = if player_int == 0 {1} else {0};
            continue;
        }

        // valid placing
        place_piece(row, col, player_int, &mut board);
    }

}
