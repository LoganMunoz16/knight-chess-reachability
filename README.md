# Rust - Knight Piece Chess Reachability

This program takes a knight piece on a chess board, and finds the minimum path between two designated squares, using only valid knight movements. These initial squares must be given in traditional chess notation. A picture of this labeling is included below for clarity when trying out new squares, or when reading the output describing your found path.

![Chess Labeling](https://github.com/LoganMunoz16/knight-chess-reachability/assets/59589283/59ee6db6-9504-4258-ae34-93f7b5fcc8f2)

## Operation
This repository contains a single source code file. As such, the recommended way of compiling and running this program (at least in an Ubuntu environment) is pasted below.

First, compile the rust source file using rustc:
```
rustc knight-chess-reachability.rs -o test
```

Second, run the "test" executable you just created:
```
./test
```

## Brief Description

The following is a brief description of the structure of this chess Board and how the two-phased commit is implemented:

### Squares
There is a new struct called a Square, which is used to make our chess board. Squares contain the following information, which assists the program in keeping track of the paths created by trying knight moves across the board:

* label: This is a tuple containing a (row, col) pair that tells us where the square is in our matrix
* path: This is a vector containing tuples, which tells us the path the knight has taken to get to this square
* origin: This is another tuple telling us where the knight started for easy comparison access to check if we have found our minimum path

### Chess Board
The chess board has a fairly simple implementation, with only one attribute inside of the struct. The primary use of the ChessBoard struct is that it contains the methods used to both initialize our board, and then find the minimum path. The single attribute is listed below:

* squares: An 8x8 matrix of Squares representing a traditional 8x8 chess board

### Algorithm Overview
For more detailed information on how the algorithm operates, please see the comments within the code included in this repository.

Generally, this algorithm approaches the reachability question by pulling on ideas from Savitch's Theorem. We could find every possible path on the board, but that would involve checking many paths we don't need to, and thus a lot of excess computations. Therefore, we implement an approach that iterates both forward and backward from the starting and ending squares respectively. 

The algorithm takes ones step forward from the starting square, checking all possible moves from that point. If the path is not complete, then we do the same thing from the ending square, checking all possible first moves. Then, we check all second degree moves going forward, and second degree moves going backward, and so on. This still generates many different paths, but it allows the algorithm to stop once our forward movement and backward movement meet. Additionally, because we stop the very first time the paths meet, that path is guaranteed to be the shortest path between those two squares.

Once we have obtained this path, we then print it out for the user, both in a view that shows the user what individual moves to make, and then in a full-board view that allows a user to see all the places on the board that the knight went.

## Future Work
Part of the future work for this algorithm will be to add in handling for a chess board that has certain squares blocked off, for example where your own piece may be. This is a required feature, and thus will be implemented into this repository soon.

Other than the required work, I also feel as though I can reduce the complexity of my code a little. There are some repeated sections of code that might be able to be cut out or consilidated, and things like the origin attribute of a Square may not be needed if I change my code up a bit. The algorithm does work well from my testing, but I still think that the code itself could be optimized just a bit. Additionally, I think it would be interesting to add in a feature that allows the user to have a full board of other chess pieces. This way, we would restrict the user from landing on a space with their own piece, but they could land on a square with an opposing piece and steal it. Not only could we find the minimum path, but we could try to find the path that maximizes the number of pieces you can capture along the way.

