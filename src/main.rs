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

    // let mut c=0;
    // let nextBoards = board.genNextBoard(&board::Piece::White);
    // for nextBoard in &nextBoards {
    //     println!("[{}] ({}, {})", c, nextBoard.pos.x, nextBoard.pos.y);
    //     nextBoard.board.print();
    //     c=c+1;
    // }

    // let tree = board.genSearchTree(&board::Piece::White, 3);
    // for elem in &tree {
    //     print!("path: ");
    //     let n = &elem.path.len();
    //     for i in 0..*n {
    //         let p = &elem.path[i];
    //         print!("({}, {})", p.pos.x, p.pos.y);
    //         if i<n-1 {
    //             print!("-");
    //         }
    //     }
    //     println!();

    //     println!("score: {}", elem.score);
    //     elem.board.print();
    // }

    let maybeResult = board.getBestMove(&board::Piece::White, 3);
    if let Some(result) = maybeResult {
        print!("path: ");
        let n = &result.path.len();
        for i in 0..*n {
            let p = &result.path[i];
            print!("({}, {})", p.pos.x, p.pos.y);
            if i<n-1 {
                print!("-");
            }
        }
        println!();
    
        println!("score: {}", result.score);
        result.board.print();
    } else {
        println!("no result found");
    }
}
    
