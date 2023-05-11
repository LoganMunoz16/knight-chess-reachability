//Ideally, I want to do this Savitch's Theorem's way, so moving from the front and the back
    //This should reduce the number of paths I need to check and allow me to stop eariler
//That being said, this is only slightly better than a brute force approach


//in a matrix, 

//For visualization of naming conventions
const chessMatrixConversions : [(&'static str, i32); 8] = [("a", 0), ("b", 1), ("c", 2), ("d", 3), ("e", 4), ("f", 5), ("g", 6), ("h", 7)];

const knightMoves: [(i32, i32); 8] = [(1, 2), (-1, 2), (2, 1), (-2, 1), (2, -1), (-2, -1), (1, -2), (-1, -2)];

fn isValidMove(position : (i32, i32), knightMove : (i32, i32)) -> bool {
    if (position.0 + knightMove.0 > 7) || (position.0 + knightMove.0 < 0) || (position.1 + knightMove.1 > 7) || (position.1 + knightMove.1 < 0){
        return false;
    } else {
        return true;
    }
}

fn convertToMatrix(chessNotation : (&'static str, i32)) -> (i32, i32) {
    for conversion in chessMatrixConversions {
        if conversion.0 == chessNotation.0 {
            return ((chessNotation.1 - 8).abs(), conversion.1);
        }
    }
    return (-1, -1);
}

fn convertToChess(matrixNotation : (i32, i32)) -> (&'static str, i32) {
    for conversion in chessMatrixConversions {
        if conversion.1 == matrixNotation.1 {
            return (conversion.0, 8 - matrixNotation.0);
        }
    }
    return ("z", -1);
}

#[derive(Clone)]
struct Square {
    label: (i32, i32),
    path: Vec<(i32, i32)>,
    origin: (i32, i32)
}

impl Square {
    fn new() -> Square {
        let mut newSquare = Square {
            label: (-1, -1),
            path: Vec::new(),
            origin: (-1, -1)
        };
        return newSquare;
    }
}

struct ChessBoard {
    squares: Vec<Vec<Square<>>>
}

impl ChessBoard {

    fn new() -> ChessBoard {
        let mut squaresMatrix : Vec<Vec<Square<>>> = Vec::new();
        let mut newBoard = ChessBoard {
            squares: squaresMatrix,
        };

        for row in 0..8 {
            let mut thisRow : Vec<Square<>> = Vec::new();
            for  col in 0..8 {
                let mut tempSquare = Square {
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

    fn determinePath(&mut self, start: (&'static str, i32), end: (&'static str, i32)) -> Vec<(i32, i32)> {
        let mut minPath : Vec<(i32, i32)> = Vec::new();
        let board = &mut self.squares;
        let startMatrix = convertToMatrix(start);
        println!("Starting at: ({}, {})", startMatrix.0, startMatrix.1);
        let endMatrix = convertToMatrix(end);
        println!("Ending at: ({}, {})", endMatrix.0, endMatrix.1);

        let startSquare = Square {
            label: startMatrix,
            path: Vec::from([startMatrix]),
            origin: startMatrix
        };

        board[startMatrix.0 as usize][startMatrix.1 as usize] = startSquare.clone();
        
        let endSquare = Square {
            label: endMatrix,
            path: Vec::from([endMatrix]),
            origin: endMatrix
        };

        board[endMatrix.0 as usize][endMatrix.1 as usize] = endSquare.clone();
    
        let mut forwardToCheck : Vec<(i32, i32)> = Vec::new();
        let mut forwardChecking : Vec<(i32, i32)> = Vec::new();
        forwardChecking.push(startMatrix);
        let mut backwardToCheck : Vec<(i32, i32)> = Vec::new();
        let mut backwardChecking : Vec<(i32, i32)> = Vec::new();
        backwardChecking.push(endMatrix);

        let mut forwardEnding = Square::new();
        let mut backwardEnding = Square::new();

        let mut foundPath = false;
    
    
        loop {
            for i in 0..forwardChecking.len() {
                let mut currentSquare = board[forwardChecking[i].0 as usize][forwardChecking[i].1 as usize].clone();
                println!("Looking at square forward: ({}, {})", currentSquare.label.0, currentSquare.label.1);
                for knightMove in knightMoves {
                    if !isValidMove(currentSquare.label, knightMove) {
                        continue;
                    }
                    let movement = (currentSquare.label.0 + knightMove.0, currentSquare.label.1 + knightMove.1);
                    let mut newSquare = board[movement.0 as usize][movement.1 as usize].clone();
                    forwardToCheck.push(newSquare.label);
                    if newSquare.origin == (-1, -1) {
                        newSquare.origin = currentSquare.origin.clone();
                        board[movement.0 as usize][movement.1 as usize] = newSquare.clone();
                    } else if newSquare.origin == endSquare.origin {
                            forwardEnding = currentSquare.clone();
                            backwardEnding = newSquare.clone();
                            if forwardEnding.path.len() > 1 {
                                forwardEnding.path.push(currentSquare.label);
                            }
                            foundPath = true;
                            break;
                    } 

                    if newSquare.path.len() == 0 {
                        newSquare.path = currentSquare.path.clone();
                        newSquare.path.push(newSquare.label);
                        board[movement.0 as usize][movement.1 as usize] = newSquare.clone();
                    } else if currentSquare.path.len() > newSquare.path.len() {
                        currentSquare.path = newSquare.path.clone();
                        board[forwardChecking[i].0 as usize][forwardChecking[i].1 as usize] = currentSquare.clone();
                    }
                }
            }
            if foundPath {
                break;
            }
            forwardChecking = forwardToCheck.clone();

            for i in 0..backwardChecking.len() {
                let mut currentSquare = board[backwardChecking[i].0 as usize][backwardChecking[i].1 as usize].clone();
                for knightMove in knightMoves {
                    if !isValidMove(currentSquare.label, knightMove) {
                        continue;
                    }
                    let movement = (currentSquare.label.0 + knightMove.0, currentSquare.label.1 + knightMove.1);
                    let mut newSquare = board[movement.0 as usize][movement.1 as usize].clone();
                    backwardToCheck.push(newSquare.label);
                    if newSquare.origin == (-1, -1) {
                        newSquare.origin = currentSquare.origin.clone();
                        board[movement.0 as usize][movement.1 as usize] = newSquare.clone();
                    } else if newSquare.origin == startSquare.origin {
                            backwardEnding = currentSquare.clone();
                            forwardEnding = newSquare.clone();
                            if backwardEnding.path.len() > 1 {
                                backwardEnding.path.push(currentSquare.label);
                            }
                            foundPath = true;
                            break
                    } 

                    if newSquare.path.len() == 0 {
                        newSquare.path = currentSquare.path.clone();
                        newSquare.path.push(newSquare.label);
                        board[movement.0 as usize][movement.1 as usize] = newSquare.clone();
                    } else if currentSquare.path.len() > newSquare.path.len() {
                        currentSquare.path = newSquare.path.clone();
                        board[forwardChecking[i].0 as usize][forwardChecking[i].1 as usize] = currentSquare.clone();
                    }
                }
            }
            if foundPath {
                break;
            }
            backwardChecking = backwardToCheck.clone();
        }

        minPath = forwardEnding.path;

        for step in (0..backwardEnding.path.len()).rev() {
            minPath.push(backwardEnding.path[step]);
        }

        return minPath;
    }
}


fn main() {
    let mut chessBoard = ChessBoard::new();
    let foundPath = chessBoard.determinePath(("a", 8), ("a", 4));

    println!("The minimum path has a length of: {}", foundPath.len() - 1);
    println!("The path is as follows:");
    for step in foundPath {
        let chessStep = convertToChess(step);
        println!("({}, {})", chessStep.0, chessStep.1);
    }
}

