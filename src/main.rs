#![allow(non_snake_case)]

use othello::board;

fn main() {
    let mut board = board::Board::new();
    board.init2();
    board.print();

    // println!("** White **");
    // let result = board.searchPos(&board::Piece::White);
    // for info in result.iter() {
    //     println!("({}, {}) = {}", info.pos.x, info.pos.y, info.scoreInfo.score);
    // }

    // println!("** Black **");
    // let result = board.searchPos(&board::Piece::Black);
    // for info in result.iter() {
    //     println!("({}, {}) = {}", info.pos.x, info.pos.y, info.scoreInfo.score);
    // }

    let mut c=0;
    let nextBoards = board.genNextBoard(&board::Piece::White);
    for nextBoard in &nextBoards {
        println!("[{}] ({}, {})", c, nextBoard.pos.x, nextBoard.pos.y);
        nextBoard.board.print();
        c=c+1;
    }

    // let mut c=0;
    // let nextBoards = board.genNextBoard(&board::Piece::White);
    // for nextBoard in &nextBoards {
    //     println!("{} ({}, {})", c, nextBoard.pos.x, nextBoard.pos.y);
    //     nextBoard.board.print();
    //     let mut d=0;
    //     let nextBoards2 = nextBoard.board.genNextBoard(&board::Piece::Black);
    //     for nextBoard2 in &nextBoards2 {
    //         println!("{}-{} ({}, {})", c, d, nextBoard2.pos.x, nextBoard2.pos.y);
    //         d=d+1;
    //         nextBoard2.board.print();
    //     }
    //     c=c+1;
    // }
}
    
