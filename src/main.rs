use text_io::read;
use std::process;

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
                check_for_win(row, column, game_board);
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

fn check_for_win(row: usize, col: usize, game_board: [[char; 7]; 6]){

    //checks if current player wins by checking all possible win conditions from the last placed token, this means I must have 4 if statements per win type (except vertical)
    //this is because the player could place the winning token in any position of the winning line


    //vertical win
    if row >= 3
    && game_board[row][col] == game_board[row-1][col]
    && game_board[row-1][col] == game_board[row-2][col]
    && game_board[row-2][col] == game_board[row-3][col] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    } 
    

    //horizontal cases
    
    //winning chip left most
    else if col < 3
    && game_board[row][col] == game_board[row][col+1]
    && game_board[row][col+1] == game_board[row][col+2]
    && game_board[row][col+2] == game_board[row][col+3] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    }
    //winning chip second left
    else if col > 0 && col < 5 
    && game_board[row][col] == game_board[row][col-1]
    && game_board[row][col] == game_board[row][col+1]
    && game_board[row][col+1] == game_board[row][col+2] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    }
    //winning chip second right
    else if col > 1 && col < 6 
    && game_board[row][col] == game_board[row][col-2]
    && game_board[row][col] == game_board[row][col-1]
    && game_board[row][col] == game_board[row][col+1] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    }
    //winning chip right most
    else if col > 2
    && game_board[row][col] == game_board[row][col-1]
    && game_board[row][col-1] == game_board[row][col-2]
    && game_board[row][col-2] == game_board[row][col-3] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    }

    //left to right incline cases

    //winning token left most
    else if col < 4 && row < 3
    && game_board[row][col] == game_board[row+1][col+1]
    && game_board[row+1][col+1] == game_board[row+2][col+2]
    && game_board[row+2][col+2] == game_board[row+3][col+3] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    }

    //winning token second left
    else if col > 0 && col < 5 
    && row > 0 && row < 4
    && game_board[row][col] == game_board[row-1][col-1]
    && game_board[row-1][col-1] == game_board[row+1][col+1]
    && game_board[row+1][col+1] == game_board[row+2][col+2] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    }

    //winning token second right
    else if col > 1 && col < 6 
    && row > 1 && row < 5
    && game_board[row][col] == game_board[row-2][col-2]
    && game_board[row-2][col-2] == game_board[row-1][col-1]
    && game_board[row-2][col-2] == game_board[row+1][col+1] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    }
    //winning token right most
    else if col > 2
    && row > 2
    && game_board[row][col] == game_board[row-1][col-1]
    && game_board[row-1][col-1] == game_board[row-2][col-2]
    && game_board[row-2][col-2] == game_board[row-3][col-3] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    }

    //left to right decline cases
    //winning token left most
    else if col < 4 && row > 2
    && game_board[row][col] == game_board[row-1][col+1]
    && game_board[row-1][col+1] == game_board[row-2][col+2]
    && game_board[row-2][col+2] == game_board[row-3][col+3] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    }

    //winning token second left
    else if col > 0 && col < 5 
    && row > 1 && row < 5
    && game_board[row][col] == game_board[row+1][col-1]
    && game_board[row+1][col-1] == game_board[row-1][col+1]
    && game_board[row-1][col+1] == game_board[row-2][col+2] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    }

    //winning token second right
    else if col > 1 && col < 6 
    && row > 0 && row < 4
    && game_board[row][col] == game_board[row+2][col-2]
    && game_board[row+2][col-2] == game_board[row+1][col-1]
    && game_board[row+1][col-1] == game_board[row-1][col+1] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    }
    //winning token right most
    else if col > 2
    && row < 3
    && game_board[row][col] == game_board[row+1][col-1]
    && game_board[row+1][col-1] == game_board[row+2][col-2]
    && game_board[row+2][col-2] == game_board[row+3][col-3] {
        for n in (0..=5).rev() {
            println!("{:?}", game_board[n]);
        }
        if game_board[row][col] == 'X'{
            println!("Player 1 wins!");
            process::exit(1);
        } else {
            println!("Player 2 wins!");
            process::exit(1);
        }
    }
}
