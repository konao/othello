#![allow(non_snake_case)]

use othello::board;
use std::io::*;
use std::thread;
use std::time;

fn getUserInput(piece: &board::Piece) -> Option<board::Pos> {
    match piece {
        board::Piece::White => {
            print!("● ");
        },
        board::Piece::Black => {
            print!("○ ");
        },
        _ => {
            // do nothing 
        }
    }
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

fn test00(optBoardPath: Option<String>) {
    let mut board = board::Board::new();

    if let Some(boardPath) = optBoardPath {
        if !board.load(&boardPath) {
            println!("failed to load {}", boardPath);
            return;
        }
    } else {
        // なければ初期状態にする
        board.init();
    }
    
    board.print();

    println!("** White **");
    let result = board.searchPos(&board::Piece::White);
    for info in result.iter() {
        println!("{} = {}", board::Pos::toDesc(info.pos.x, info.pos.y), info.score);
    }

    println!("** Black **");
    let result = board.searchPos(&board::Piece::Black);
    for info in result.iter() {
        println!("{} = {}", board::Pos::toDesc(info.pos.x, info.pos.y), info.score);
    }

}

fn test01(optBoardPath: Option<String>) {
    let mut board = board::Board::new();

    if let Some(boardPath) = optBoardPath {
        if !board.load(&boardPath) {
            println!("failed to load {}", boardPath);
            return;
        }
    } else {
        // なければ初期状態にする
        board.init();
    }
    
    board.print();

    let mut c=0;
    let nextBoards = board.genNextBoard(&board::Piece::White);
    for nextBoard in &nextBoards {
        println!("[{}] ({}, {}) : ntake={}, score={}", c, nextBoard.pos.x, nextBoard.pos.y, nextBoard.ntake, nextBoard.score);
        nextBoard.board.print();
        c=c+1;
    }
}

fn test02(optBoardPath: Option<String>) {
    let mut board = board::Board::new();

    if let Some(boardPath) = optBoardPath {
        if !board.load(&boardPath) {
            println!("failed to load {}", boardPath);
            return;
        }
    } else {
        // なければ初期状態にする
        board.init();
    }
    
    board.print();

    let tree = board.genSearchTree(&board::Piece::White, 1);
    for elem in &tree {
        print!("path: ");
        let n = &elem.path.len();
        for i in 0..*n {
            let p = &elem.path[i];
            print!("({}, {})", p.pos.x, p.pos.y);
            if i<n-1 {
                print!("-");
            }
        }
        println!();

        println!("score: {}", elem.score);
        elem.board.print();
    }
}

fn test03(optBoardPath: Option<String>) {
    let mut board = board::Board::new();

    if let Some(boardPath) = optBoardPath {
        if !board.load(&boardPath) {
            println!("failed to load {}", boardPath);
            return;
        }
    } else {
        board.init();
    }
    
    board.print();

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

fn test04(optBoardPath: Option<String>) {
    let mut board = board::Board::new();

    if let Some(boardPath) = optBoardPath {
        if !board.load(&boardPath) {
            println!("failed to load {}", boardPath);
            return;
        }
    }

    board.print();
}

fn game(optBoardPath: Option<String>) {
    let VERSION = 0.2;
    
    println!("***************************");
    println!("  Othello Game (ver {})", VERSION);
    println!("***************************");

    let mut board = board::Board::new();

    if let Some(boardPath) = optBoardPath {
        if !board.load(&boardPath) {
            println!("failed to load {}", boardPath);
            return;
        }
    } else {
        // なければ初期状態にする
        board.init();
    }
    
    board.print();

    let playerPiece = &board::Piece::Black;
    let computerPiece = board::Piece::getOpponent(&playerPiece);
    let mut playerPass = false;
    let mut computerPass = false;

    while (!playerPass) || (!computerPass) {
        // Human
        
        // 先に置ける場所があるかチェックする
        let possibleMoves = board.searchPos(&playerPiece);
        if possibleMoves.len() > 0 {
            let playerInput = getUserInput(&playerPiece);
            if playerInput.is_none() {
                continue;
            }
            let playerPos = playerInput.unwrap();
    
            // println!("({}, {})", playerPos.x, playerPos.y);
    
            if let Some(ret) = board.put(&playerPiece, &playerPos) {
                board = ret.board;  // 新しい盤に更新
                board.print();
                board.printScore();
                playerPass = false;
            } else {
                println!("You cannot place on {}", board::Pos::toDesc(playerPos.x, playerPos.y));
                continue;
            }    
        } else {
            println!("Sorry. No place for your piece.");
            playerPass = true;
        }

        // Computer
        println!("Hmm ... ");
        // stdout().flush().unwrap();
        // for _ in 0..6 {
        //     thread::sleep(time::Duration::from_secs_f64(0.5));
        //     print!(".");
        //     stdout().flush().unwrap();
        // }
        // println!();

        let maybeResult = board.getBestMove(&computerPiece, 12);
        if let Some(result) = maybeResult {
            if result.path.len() > 0 {
                let nextPos = &result.path[0].pos;
                if let Some(ret) = board.put(&computerPiece, nextPos) {
                    board = ret.board;
                    board.print();
                    println!("I put on {}", board::Pos::toDesc(nextPos.x, nextPos.y));
                    board.printScore();
                }
            } else {
                println!("No place for me.");
                computerPass = true;
            }
        } else {
            println!("No place for me.");
            computerPass = true;
        }
    }

    println!("*** Game Over ***");
    board.printScore();
}

fn main() {
    let mut optBoardPath: Option<String> = None;
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        optBoardPath = Some(args[1].to_string());
    }

    // test00(optBoardPath);
    // test01(optBoardPath);
    // test02(optBoardPath);
    // test03(optBoardPath);
    // test04(optBoardPath);

    game(optBoardPath);
}