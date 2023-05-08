struct Node<'a> {
    name: &'a str,
    edges: Option<Vec<i32>>
}

struct Graph<'a> {
    nodes: Vec<Node<'a>>
}

impl<'a> Graph<'a> {

    fn new() -> Graph<'a> {
        let mut squaresList : Vec<Node<'a>> = Vec::new();
        for row in 0..9 {
            for col in 1..9 {
                let mut name: &'a str = &(col.to_string() + &row.to_string());
                let mut tempNode = Node {
                    name: name,
                    edges: None,
                };
                squaresList.push(tempNode);
            }
        }
        let mut newGraph = Graph {
            nodes: squaresList,
        };
        return newGraph;
    }
}


fn main() {
    let mut chessBoard = Graph::new();
    for i in 0..chessBoard.nodes.len() {
        println!("{}", chessBoard.nodes[i].name);
    }
}

