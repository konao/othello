#![allow(non_snake_case)]

use othello::board;
use std::io::*;
use std::thread;
use std::time;

fn getUserInput() -> Option<board::Pos> {
    print!("? ");
    stdout().flush().unwrap();

    let mut line = String::new();
    if stdin().read_line(&mut line).is_err() {
        return None;
    }

    line = line.trim().to_uppercase().to_string();

    let first = line.chars().nth(0);
    if first.is_none() {
        return None;
    }
    let x = match first.unwrap() {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        'E' => 5,
        'F' => 6,
        'G' => 7,
        'H' => 8,
        _ => 0
    };
    if x == 0 {
        return None;
    }

    let second = line.chars().nth(1);
    if second.is_none() {
        return None;
    }
    if let Some(y) = second.unwrap().to_string().parse::<i32>().ok() {
        if (y >= 1) && (y <= 8) {
            return Some(board::Pos { x, y });
        }
    }
    return None;
}

fn main() {
    let mut board = board::Board::new();
    board.init();
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

    // let maybeResult = board.getBestMove(&board::Piece::White, 3);
    // if let Some(result) = maybeResult {
    //     print!("path: ");
    //     let n = &result.path.len();
    //     for i in 0..*n {
    //         let p = &result.path[i];
    //         print!("({}, {})", p.pos.x, p.pos.y);
    //         if i<n-1 {
    //             print!("-");
    //         }
    //     }
    //     println!();
    
    //     println!("score: {}", result.score);
    //     result.board.print();
    // } else {
    //     println!("no result found");
    // }

    let mut giveup = false;
    while !giveup {
        // Human
        let result = getUserInput();
        if let Some(pos) = result {
            println!("({}, {})", pos.x, pos.y);

            if let Some(ret) = board.put(&board::Piece::Black, &pos) {
                board = ret.board;  // 新しい盤に更新
                board.print();

                // Computer
                print!("Hmm ");
                stdout().flush().unwrap();
                for _ in 0..6 {
                    thread::sleep(time::Duration::from_secs_f64(0.5));
                    print!(".");
                    stdout().flush().unwrap();
                }
                println!();

                let maybeResult = board.getBestMove(&board::Piece::White, 5);
                if let Some(result) = maybeResult {
                    if result.path.len() > 0 {
                        let nextPos = &result.path[0].pos;
                        if let Some(ret) = board.put(&board::Piece::White, nextPos) {
                            println!("I put on ({}, {})", nextPos.x, nextPos.y);
                            board = ret.board;
                            board.print();
                        } else {
                            println!("no place to move. give up!! (1)");
                            giveup = true;
                        }
                    } else {
                        println!("no place to move. give up!! (2)");
                        giveup = true;
                    }
                } else {
                    println!("no result found");
                }
            } else {
                println!("You cannot place on ({}, {})", pos.x, pos.y);
            }
        }
    }
}
    
