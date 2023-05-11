/*
* Author: Logan Munoz
* Date: 5/11/2023
* CSC 4800
* Project: Knight chess board reachability
*
* Brief Description: This program creats a matrix of Square objects. These objects are used to house information
*                    in the form of tuples detailing the path a knight takes as it moves across the board. I use
*                    an approach similar to the one seen in Savitch's Theorem, where we alternate taking a single
*                    move forward from the start with a single move backwards from the end. This helps to reduce
*                    the number of possible paths to check, and allows us to just watch for the first time when 
*                    the backward movement path hits a forward movement path, or vise versa. That first touch is
*                    our minimum path!
*
* A note on chess notation: Chess boards are labeled with the top left corner being ("a", 8), and the bottom left
*                           as ("h", 1), and thus this program uses that labeling convention for in/output.
*/

//A pre-defined list of tupes to convert the chess board notation into something more useful in a matrix. 
const CHESS_TO_MATRIX_CONVERSION : [(&'static str, i32); 8] = [("a", 0), ("b", 1), ("c", 2), ("d", 3), ("e", 4), ("f", 5), ("g", 6), ("h", 7)];

//Another pre-definied list of tupes representing the possible moves a knight can take on a chess board.
const KNIGHT_MOVES: [(i32, i32); 8] = [(1, 2), (-1, 2), (2, 1), (-2, 1), (2, -1), (-2, -1), (1, -2), (-1, -2)];

/* isValidMove
* Desc: Checks whether the desired knight movement from the given position is valid, i.e. will not go off the board
* @param: position - a tuple with the row/col of the knight's current position
*         knightMove - the desired movement of the knight from the list above
* @return: boolean true if a valid move, and false if now
*/
fn isValidMove(position : (i32, i32), knightMove : (i32, i32)) -> bool {
    if (position.0 + knightMove.0 > 7) || (position.0 + knightMove.0 < 0) || (position.1 + knightMove.1 > 7) || (position.1 + knightMove.1 < 0){
        return false;
    } else {
        return true;
    }
}

/* convertToMatrix
* Desc: Converts a square label in chess board notation to a label in matrix notation for use in the program
* @param: chessNotation - a tuple with the desired chess square, in the form of ("letter", number)
* @return: tuple - representing the row and col in an 8x8 matrix that marks the same square as the chess notation
*/
fn convertToMatrix(chessNotation : (&'static str, i32)) -> (i32, i32) {
    if chessNotation.1 > 8 {
        return (-1, -1);
    }
    for conversion in CHESS_TO_MATRIX_CONVERSION {
        if conversion.0 == chessNotation.0 {
            return ((chessNotation.1 - 8).abs(), conversion.1);
        }
    }
    return (-1, -1);
}

/* convertToChess
* Desc: Converts a square label in matrix notation to a label in chess board notation for use in printint paths
* @param: matrixNotation - a tuple with the desired matrix square, in the form of (row, col)
* @return: tuple - representing the same square in chess notation, so ("letter", number)
*/
fn convertToChess(matrixNotation : (i32, i32)) -> (&'static str, i32) {
    for conversion in CHESS_TO_MATRIX_CONVERSION {
        if conversion.1 == matrixNotation.1 {
            return (conversion.0, 8 - matrixNotation.0);
        }
    }
    return ("z", -1);
}

/* isValidInput
* Desc: Checks to ensure the desired starting and ending squares are valid squares
* @param: start - a tuple with the desired chess square, in the form of ("letter", number)
*         end - a tuple with the desired chess square, in the form of ("letter", number)
* @return: boolean true if valid and false if not
*/
fn isValidInput(start: (&'static str, i32), end: (&'static str, i32)) -> bool{
    let startConvert = convertToMatrix(start);
    let endConvert = convertToMatrix(end);

    if (startConvert.0 > 7) || (startConvert.1 > 7) || (startConvert.0 < 0) || (startConvert.1 < 0) {
        return false;
    } else if (endConvert.0 > 7) || (endConvert.1 > 7) || (endConvert.0 < 0) || (endConvert.1 < 0) {
        return false;
    } else {
        return true;
    }
}

/* printBoard
* Desc: Prints an ascii representation of the chess board, with the knight's path marked by K's
* @param: path - a vector of tuples representing the path that the knight takes
* @return: none
*/
fn printBoard(path: Vec<(i32, i32)>) {
    println!("    a   b   c   d   e   f   g   h");
    println!("  - - - - - - - - - - - - - - - - -");
    for i in 0..8 {
        print!("{} | ", 8 -i);
        for j in 0..8 {
            if path.contains(&(i,j)) {
                print!("K | ");
            } else {
                print!("  | ");
            }
        }
        print!("\n");
        println!("  - - - - - - - - - - - - - - - - -");
    }
}

/* Square Struct
* Desc: A struct used to fill out our chess board and hold the necessary information
* Attributes:  label - a tuple with the (row, col) value of the square in a matrix
*              path - a vector of tuples representing the path the knight has taken to get here
*              origin - a tuple representing where the knight started before arriving at the square
*/
#[derive(Clone)]
struct Square {
    label: (i32, i32),
    path: Vec<(i32, i32)>,
    origin: (i32, i32)
}

//The following contains the methods of my Square struct
impl Square {

    /* new
    * Desc: Initializes a new Square with "null" labels and origins, and an empty vector for path
    * @param: none
    * @return: the initialized Square
    */
    fn new() -> Square {
        let newSquare = Square {
            label: (-1, -1),
            path: Vec::new(),
            origin: (-1, -1)
        };
        return newSquare;
    }
}

/* ChessBoard Struct
* Desc: A struct used to represent our chess board, containing a matrix of Squares
* Attributes:  squares - a matrix of Square structs representing the chess board
*/
struct ChessBoard {
    squares: Vec<Vec<Square<>>>
}

//The following are the methods of the ChessBoard struct
impl ChessBoard {

    /* new
    * Desc: Initializes a new 8x8 ChessBoard filled in with Squares. Each Square has the correct
    *       label, but all other information is left blank.
    * @param: none
    * @return: the initialized ChessBoard
    */
    fn new() -> ChessBoard {
        let squaresMatrix : Vec<Vec<Square<>>> = Vec::new();
        let mut newBoard = ChessBoard {
            squares: squaresMatrix,
        };

        for row in 0..8 {
            let mut thisRow : Vec<Square<>> = Vec::new();
            for  col in 0..8 {
                let tempSquare = Square {
                    label: (row, col),
                    path: Vec::new(),
                    origin: (-1, -1)
                };
                thisRow.push(tempSquare);
            }
            newBoard.squares.push(thisRow);
        }
        return newBoard;
    }

    /* determinePath
    * Desc: Determines the shortest path from the staring square to the ending square. Does so in a Savitch's Theorem
    *       manner, where we move both forward and backward and check to see when we meet in the middle
    * @param: self - a mutable reference to this ChessBoard so we can modify the squares in it
    *         start - the starting square of our knight
    *         end - the ending square of our knight
    * @return: a vector of tuples representing the path the knight takes
    */
    fn determinePath(&mut self, start: (&'static str, i32), end: (&'static str, i32)) -> Vec<(i32, i32)> {
        //Begin by initializing the path, getting a reference to our board, and converting our squares into matrix notation
        let mut minPath : Vec<(i32, i32)> = Vec::new();
        let board = &mut self.squares;
        let startMatrix = convertToMatrix(start);
        let endMatrix = convertToMatrix(end);

        //Create a square for the start
        let startSquare = Square {
            label: startMatrix,
            path: Vec::from([startMatrix]),
            origin: startMatrix
        };

        //Add that square to the board where it should be
        board[startMatrix.0 as usize][startMatrix.1 as usize] = startSquare.clone();
        
        //Create a square for the end
        let endSquare = Square {
            label: endMatrix,
            path: Vec::from([endMatrix]),
            origin: endMatrix
        };

        //Add this one as well
        board[endMatrix.0 as usize][endMatrix.1 as usize] = endSquare.clone();
    
        //Initialize all of our vectors to track knight movement
        //The "ToCheck" vectors contain the list of squares to check on the next iteration
        //The "Checking" vectors contain the list of squares we are currently looking at
        let mut forwardToCheck : Vec<(i32, i32)> = Vec::new();
        let mut forwardChecking : Vec<(i32, i32)> = Vec::new();
        forwardChecking.push(startMatrix);
        let mut backwardToCheck : Vec<(i32, i32)> = Vec::new();
        let mut backwardChecking : Vec<(i32, i32)> = Vec::new();
        backwardChecking.push(endMatrix);

        //Initialize variables to hold the squares at the end
        let mut forwardEnding = Square::new();
        let mut backwardEnding = Square::new();

        //Create two more variables, one to track when we found the path to break the loop
        //  and the other to count the number of squares and break us off if we checked them all with no solution
        let mut foundPath = false;
        let mut squareCount = 2;
    
    
        //Loop until we have looked at every possible square
        while squareCount < 64 {
            //Go through the list of squares we are currently checking
            for i in 0..forwardChecking.len() {
                //Save the current square based on the board
                let currentSquare = board[forwardChecking[i].0 as usize][forwardChecking[i].1 as usize].clone();

                //Iterate through all possible knight moves from that square
                for knightMove in KNIGHT_MOVES {
                    //If invalid, skip it
                    if !isValidMove(currentSquare.label, knightMove) {
                        continue;
                    }
                    //Make a new square based on where the knight would move to
                    let movement = (currentSquare.label.0 + knightMove.0, currentSquare.label.1 + knightMove.1);
                    let mut newSquare = board[movement.0 as usize][movement.1 as usize].clone();
                    forwardToCheck.push(newSquare.label); //Save this new one to check in the next run

                    //If the origin is "null" save over that with the origin on our current path and increment squareCount
                    if newSquare.origin == (-1, -1) {
                        newSquare.origin = currentSquare.origin.clone();
                        squareCount += 1;
                        board[movement.0 as usize][movement.1 as usize] = newSquare.clone(); //Save into the board

                    } else if newSquare.origin == endSquare.origin { //If this new square has a path starting at our desired end...
                            //We found it!
                            //Save the two squares containing the paths
                            forwardEnding = currentSquare.clone();
                            backwardEnding = newSquare.clone();
                            //Break out of the loop
                            foundPath = true;
                            break;
                    } 

                    //If the new square doesn't have a path,
                    if newSquare.path.len() == 0 {
                        //Add in the current path and then update the board
                        newSquare.path = currentSquare.path.clone();
                        newSquare.path.push(newSquare.label);
                        board[movement.0 as usize][movement.1 as usize] = newSquare.clone();
                    } 
                }
            }
            //If we found a path, then we can exit the while loop
            if foundPath {
                break;
            }

            //If not, move the squares we saved for next iteration over to the correct array
            forwardChecking = forwardToCheck.clone();

            //This loop has the same logic as the one above, but is iterating from the end instead of the start
            //For clarification, check the comments above
            for i in 0..backwardChecking.len() {
                let currentSquare = board[backwardChecking[i].0 as usize][backwardChecking[i].1 as usize].clone();
                for knightMove in KNIGHT_MOVES {
                    if !isValidMove(currentSquare.label, knightMove) {
                        continue;
                    }
                    let movement = (currentSquare.label.0 + knightMove.0, currentSquare.label.1 + knightMove.1);
                    let mut newSquare = board[movement.0 as usize][movement.1 as usize].clone();
                    backwardToCheck.push(newSquare.label);
                    squareCount += 1;
                    if newSquare.origin == (-1, -1) {
                        newSquare.origin = currentSquare.origin.clone();
                        squareCount += 1;
                        board[movement.0 as usize][movement.1 as usize] = newSquare.clone();
                    } else if newSquare.origin == startSquare.origin {
                            backwardEnding = currentSquare.clone();
                            forwardEnding = newSquare.clone();
                            foundPath = true;
                            break
                    } 

                    if newSquare.path.len() == 0 {
                        newSquare.path = currentSquare.path.clone();
                        newSquare.path.push(newSquare.label);
                        board[movement.0 as usize][movement.1 as usize] = newSquare.clone();
                    } 
                }
            }
            if foundPath {
                break;
            }
            backwardChecking = backwardToCheck.clone();
        }

        //If we checked all squares and never found a path, return our empty path
        if !foundPath {
            return minPath;
        }

        //Save the first half of the path into the minPath variable
        minPath = forwardEnding.path;

        //Add in the rest of the path from the halfway point to the end
        for step in (0..backwardEnding.path.len()).rev() {
            minPath.push(backwardEnding.path[step]);
        }

        //Return the path
        return minPath;
    }
}

fn main() {
    //Create our ChessBoard and save our starting and ending squares
    let mut chessBoard = ChessBoard::new();
    let startingSquare = ("c", 7);
    let endingSquare = ("h", 2);
    
    //Ensure that those squares are valid, if not we stop and alert the user
    if !isValidInput(startingSquare, endingSquare) {
        println!("Please double check your desired squares, they do not appear to be in a valid format.");
        return;
    }

    //Run our algorithm
    let foundPath = chessBoard.determinePath(startingSquare, endingSquare);

    //If the path has a length of 0, that means we didn't find any :(
    if foundPath.len() == 0 {
        println!("No path could be found from ({}, {}) to ({}, {})\n", startingSquare.0, startingSquare.1, endingSquare.0, endingSquare.1);
    } else {
        //Print out the found path, both in tuple notation and showing the entire board
        println!("Now determining the minimum path from ({}, {}) to ({}, {})\n", startingSquare.0, startingSquare.1, endingSquare.0, endingSquare.1);
        println!("The minimum path has a length of: {}", foundPath.len() - 1);
        println!("The path is as follows:\n");
        for i in 0..foundPath.len() - 1 {
            let chessStep1 = convertToChess(foundPath[i]);
            let chessStep2 = convertToChess(foundPath[i+1]);
            println!("({}, {})  ->  ({}, {})", chessStep1.0, chessStep1.1, chessStep2.0, chessStep2.1);
        }
        println!("\nTo see this more visually...\n\n");
        printBoard(foundPath);
    }
}

