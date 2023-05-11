//Ideally, I want to do this Savitch's Theorem's way, so moving from the front and the back
    //This should reduce the number of paths I need to check and allow me to stop eariler
//That being said, this is only slightly better than a brute force approach


//in a matrix, 

//For visualization of naming conventions
const chessBoardSquares: [[&str; 8] ;8] = [["a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"],
                                      ["a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7"],
                                      ["a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6"],
                                      ["a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5"],
                                      ["a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4"],
                                      ["a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3"],
                                      ["a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2"],
                                      ["a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1"]];

const knightMoves: [(i32, i32); 8] = [(1, 2), (-1, 2), (2, 1), (-2, 1), (2, -1), (-2, -1), (1, -2), (-1, -2)];

fn isValidMove(position : (i32, i32), knightMove : (i32, i32)) -> bool {
    if (position.0 + knightMove.0 > 8) || (position.0 + knightMove.0 < 0) || (position.1 + knightMove.1 > 8) || (position.1 + knightMove.1 < 0){
        return false;
    } else {
        return true;
    }
}

struct Square {
    label: (i32, i32),
    path: Option<Vec<(i32, i32)>>,
    origin: Option<(i32, i32)>
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

        for row in (0..8).rev() {
            let mut thisRow : Vec<Square<>> = Vec::new();
            for  col in 0..8 {
                let mut tempSquare = Square {
                    label: (row, col),
                    path: None,
                    origin: None
                };
                thisRow.push(tempSquare);
            }
            newBoard.squares.push(thisRow);
        }
        return newBoard;
    }
}


fn main() {
    let mut chessBoard = ChessBoard::new();
    for i in 0..8 {
        for j in 0..8 {
            println!("{}, {}", chessBoard.squares[i][j].label.0, chessBoard.squares[i][j].label.1);
        }
    }
}

