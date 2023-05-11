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

fn isValidMove(x : i32, y : i32, xMove : i32, yMove: i32) -> bool {
    if (x + xMove > 8) || (x + xMove < 0) || (y + yMove > 8) || (y + yMove < 0){
        return false;
    } else {
        return true;
    }
}

struct Node<'a> {
    name: &'a str,
    moves: Option<Vec<Box<Node<'a>>>>,
    path: Option<Vec<&'a str>>,
    origin: Option<&'a str>
}

struct Graph<'a> {
    nodes: Vec<Node<'a>>
}

impl<'a> Graph<'a> {

    fn new() -> Graph<'a> {
        let mut squaresList : Vec<Node<'a>> = Vec::new();
        for row in chessBoardSquares {
            for col in row {
                let mut tempNode = Node {
                    name: col,
                    moves: None,
                    path: None,
                    origin: None
                };
                squaresList.push(tempNode);
            }
        }
        let mut newGraph = Graph {
            nodes: squaresList,
        };
        return newGraph;
    }

    fn addValidMoves(&self, takenPositions : Vec<&str>) {
        for row in 0..8 {
            for col in 0..8 {
                let nodeIndex = col + row * 8;
                for x in 0..3 {
                    for y in 0..3 {
                        //Iterate through all knight moves
                        //Checking if valid
                        //If valid, then add resulting node to moves list
                        //Just use the offset values and formula above to find node in list
                    }
                }
            }
        }
    }
}


fn main() {
    let mut chessBoard = Graph::new();
    for i in 0..chessBoard.nodes.len() {
        println!("{}", chessBoard.nodes[i].name);
    }
}

