# MineSweeper
In this classic game you are presented with a board of squares. Some squares contain mines, others don't. If you click on a square containing a mine, you lose. If you manage to open up all the squares without clicking on a mine, you win.

## Rules
- A square's "neighbors" are the squares above, below, left, right, and all 4 diagonals. Squares on the sides of the board or in a corner have fewer neighbors. The board does not wrap around the edges.
- If you open a square with no neighboring mines, all its neighbors will automatically open. This can cause a large area to open up quickly.
- To remove a marker from a square, point at it and right-click again (or double-tap).
- If you mark a mine incorrectly, you will have to correct the mistake before you can win. Incorrect mine marking doesn't kill you, but it can lead to mistakes which do.
- You don't have to mark all the mines to win, you just need to open all non-mine squares!

## Tools
- wasm-pack: To generate wasm code
- cargo: To run rust files and install dependency
- serve: To run static html page

## Commands
`wasm-pack build --target web`: generates wasm code for web  
`cargo build`: compile rust code  
`cargo test -- --nocapture`: run test case  

## How to run
- Run `cargo build` to install the dependencies
- Run `wasm-pack build --target web` to create wasm files
- Run `Serve` to execute the HTML file