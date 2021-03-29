use std::process;
pub fn run(row: usize, col: usize, game_board: [[char; 7]; 6]){

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