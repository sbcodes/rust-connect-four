use text_io::read;
use std::process;
mod win_conditions;

fn main() {
    //init empty game_board, starting player as red, and an empty counter for logic
    let mut game_board: [[char; 7]; 6] = [[' '; 7]; 6];
    let mut player = 1;
    let mut column: usize;
    let mut row: usize = 0;


    //main game loop
    loop{
        //user input loop
        loop{
            println!("Player {} enter column number (1-7): ", player);
            column = read!();
            //loops if column number is invalid
            if column > 7 || column < 1 {
                println!("Column number must be between 1 and 7");
            } else if game_board[5][column-1] != ' ' {
                // really gross if statement to check if the board is full and draw the game if it is
                if game_board[5][0] != ' ' && game_board[5][1] != ' ' && game_board[5][2] != ' ' && game_board[5][3] != ' ' && game_board[5][4] != ' ' && game_board[5][5] != ' ' && game_board[5][6] != ' ' {
                    println!("DRAW!");
                    process::exit(1);
                }
                println!("Column is full, pick another!");
            } else {
                //there is space, continue to adding tokens
                column -= 1;
                break;
            }
        }
        loop {
            //if the slot is empty
            if game_board[row][column] == ' '  {
            
                if player == 1 {
                    game_board[row][column] = 'X';
                    player = 2;
                } else {
                    game_board[row][column] = 'O';
                    player = 1;
                }
                //check for win here
                win_conditions::run(row, column, game_board);
                row = 0;
                break;
            } else { row += 1; } // if the slot if full it moves up to the next row and checks there
        }
        //prints game board
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        
    }
}
