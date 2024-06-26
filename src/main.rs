#![allow(non_snake_case)]
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::Font;

use std::io::*;
use std::thread;
use std::time;
use std::time::Duration;

use othello::board;
use othello::Util;

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
    let nextBoards = board.genNextBoards(&board::Piece::White);
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

fn drawBoard<'a>(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    font: &Font,
    texture_creator: &'a TextureCreator<WindowContext>,
    board: &board::Board) -> () 
{
    let lm = 32;    // left margin
    let tm = 32;    // top margin
    let ps = 96;    // piece size

    for _y in 1..=8 {
        for _x in 1..=8 {
            let optPiece = board.getPiece(_x, _y);
            if let Some(piece) = optPiece {
                let src: Rect;
                match piece {
                    &board::Piece::White => {
                        src = Rect::new(192, 0, 96, 96);
                    },
                    &board::Piece::Black => {
                        src = Rect::new(96, 0, 96, 96);
                    },
                    &board::Piece::Space => {
                        src = Rect::new(0, 0, 96, 96);
                    }
                }
                let dest: Rect = Rect::new((_x-1)*96+lm, (_y-1)*96+tm, 96, 96);
                canvas.copy(texture, Some(src), Some(dest)).expect("copy texture to canvas failed");
            }
        }
    }

    let wl = 16;    // font width
    let hl = 24;    // font height
    let xChars = ["A", "B", "C", "D", "E", "F", "G", "H"];
    for i in 0..xChars.len() {
        let x = (i*ps + (ps/2)) as i32 + lm - 2;
        Util::textOut(canvas, &font, &texture_creator, xChars[i], 255, 255, 255, x, 5, wl, hl);
        Util::textOut(canvas, &font, &texture_creator, xChars[i], 255, 255, 255, x, (ps as i32) * 8 + (tm as i32) + 2, wl, hl);
    }
    for i in 0..8 {
        let y = (i*ps + (ps/2)) as i32 + tm - 4;
        Util::textOut(canvas, &font, &texture_creator, &((i+1) as i32).to_string(), 255, 255, 255, 8, y, wl, hl);
        Util::textOut(canvas, &font, &texture_creator, &((i+1) as i32).to_string(), 255, 255, 255, 8 + (ps as i32) * 8 + (tm - 2) as i32, y, wl, hl);
    }

    canvas.present();
}

fn getPosOnMouseClick(x: i32, y: i32) -> Option<board::Pos> {
    let lm = 32;    // left margin
    let tm = 32;    // top margin
    let ps = 96;    // piece size

    let px = x - lm;
    let py = y - tm;
    
    if (px < 0) || (py < 0) {
        return None;
    }

    let ix = px / ps;
    let iy = py / ps;

    if (ix > 7) || (iy > 7) {
        return None;
    }

    return Some(board::Pos {x: ix+1, y: iy+1});
}

fn game(optBoardPath: Option<String>) {
    let VERSION = 0.3;
    let title = format!("*** Othello (ver {}) ***", VERSION);
    
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    let width: u32 = 960;
    let height: u32 = 832;
    let window = video_subsystem
        .window(&title, width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let image_texture = texture_creator.load_texture("assets/Image1.png").expect("load image failed");

    // init font stuff
    let ttf_context = sdl2::ttf::init().expect("failed to init SDL TTF");
    let font = ttf_context.load_font("assets/arial.ttf", 128).expect("failed to load font");
    
    // clear canvas
    canvas.set_draw_color(Color::RGB(0, 0, 0)); // black
    canvas.clear();
    canvas.present();
    
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
    let mut bFirst = true;

    let mut event_pump = sdl2_context.event_pump().unwrap();
    'running: loop {
        let mut playerInput: Option<board::Pos> = None;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => break 'running,
                | Event::MouseButtonDown {
                    x, y, ..
                } => {
                    // println!("(x,y)=({},{})", x, y);
                    playerInput = getPosOnMouseClick(x, y);
                },
                _ => {}
            }
        }

        drawBoard(&mut canvas, &image_texture, &font, &texture_creator, &board);
        
        if (!playerPass) || (!computerPass) {
            // Human
            
            // 先に置ける場所があるかチェックする
            let possibleMoves = board.searchPos(&playerPiece);
            if possibleMoves.len() > 0 {
                // let playerInput = getUserInput(&playerPiece);
                if playerInput.is_none() {
                    continue;
                }
                let playerPos = playerInput.unwrap();
        
                println!("({}, {})", playerPos.x, playerPos.y);
        
                if let Some(ret) = board.put(&playerPiece, &playerPos) {
                    board = ret.board;  // 新しい盤に更新
                    board.print();
                    board.printScore();
                    drawBoard(&mut canvas, &image_texture, &font, &texture_creator, &board);
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
            stdout().flush().unwrap();
            // for _ in 0..6 {
            //     thread::sleep(time::Duration::from_secs_f64(0.5));
            //     print!(".");
            //     stdout().flush().unwrap();
            // }
            // println!();

            let maybeResult = board.getBestMove(&computerPiece, 13);
            if let Some(result) = maybeResult {
                if result.path.len() > 0 {
                    let nextPos = &result.path[0].pos;
                    if let Some(ret) = board.put(&computerPiece, nextPos) {
                        board = ret.board;
                        board.print();
                        println!("I put on {}", board::Pos::toDesc(nextPos.x, nextPos.y));
                        board.printScore();
                        drawBoard(&mut canvas, &image_texture, &font, &texture_creator, &board);
                    }
                } else {
                    println!("No place for me.");
                    computerPass = true;
                }
            } else {
                println!("No place for me.");
                computerPass = true;
            }
        } else {
            if bFirst {
                println!("*** Game Over ***");
                board.printScore();
                bFirst = false; // ここへはもう来ない
            }
        }
    }
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