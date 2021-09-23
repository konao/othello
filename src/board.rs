#![allow(non_snake_case)]

use std::io::{BufRead, BufReader};
use std::fs::File;

// 駒
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Space,  // 空白
    White,  // 白
    Black   // 黒
}

impl Piece {
    // pieceに対する相手の駒を返す
    pub fn getOpponent(piece: &Piece) -> Piece {
        return match *piece {
            Piece::Space => Piece::Space,
            Piece::White => Piece::Black,
            Piece::Black => Piece::White
        };
    }

    // 文字列表現を返す
    pub fn to_str(&self) -> &str {
        return match &self {
            Piece::Space => "Space",
            Piece::White => "Black",
            Piece::Black => "White"
        };
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pos {
    pub x: i32,  // 1..8
    pub y: i32   // 1..8
}

impl Pos {
    // Pos構造体からBoardのインデックスを返す
    //
    // (ex)
    // pub fn toIdx(&self) -> usize {
    //     return ((self.y-1) * 8 + (self.x-1)).into();
    // }

    pub fn idx(x: i32, y: i32) -> Option<usize> {
        if (x >= 1) && (x <= 8) && (y >= 1) && (y <= 8) {
            return Some(((y-1) * 8 + (x-1)) as usize);
        } else {
            return None;
        }
    }

    pub fn getDxDy(dir: i32) -> (i32, i32) {
        return match dir {
            0 => (0, -1),
            1 => (1, -1),
            2 => (1, 0),
            3 => (1, 1),
            4 => (0, 1),
            5 => (-1, 1),
            6 => (-1, 0),
            7 => (-1, -1),
            _ => (0, 0)
        };
    }

    pub fn toDesc(x: i32, y: i32) -> String {
        let xstr = match x {
            1 => "A",
            2 => "B",
            3 => "C",
            4 => "D",
            5 => "E",
            6 => "F",
            7 => "G",
            8 => "H",
            _ => " "
        };
        return format!("{}{}", xstr, y);
    }
}

// 探索結果を表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult1 {
    pub pos: Pos,   // 置ける位置
    pub ntake: i32, // 取れる駒の数
    pub score: i32, // スコア（値が大きいほど有利）
    pub dirs: Vec<i32>  // 相手の駒を取れる方向
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult2 {
    pub pos: Pos,   // posに
    pub piece: Piece,   // このpieceを置いたら
    pub board: Board,   // このボードになる
    pub ntake: i32,
    pub score: i32,
    pub capturedPieceLocs: Vec<Pos>
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult3 {
    pub path: Vec<SearchResult3Sub>,  // 駒を置いたパス
    pub board: Board,   // pathに置いた結果
    pub ntake: i32,
    pub score: i32
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult3Sub {
    pub pos: Pos,
    pub piece: Piece
}

#[derive(Clone, Debug, PartialEq)]
pub struct Count {
    pub nWhitePieces: i32,
    pub nBlackPieces: i32
}

// オセロ盤
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pieces: Vec<Piece>,
    coefs: Vec<i32> // スコア計算用の係数（盤上の場所ごとに決まる）
}

impl Board {
    // 空のボードを作る
    pub fn new() -> Self {
        let mut pieces = Vec::<Piece>::new();
        for _y in 1..=8 {
            for _x in 1..=8 {
                pieces.push(Piece::Space);
            }
        }

        let mut coefs = Vec::<i32>::new();
        for _y in 1..=8 {
            for _x in 1..=8 {
                let mut c = 1;
                if 
                    ((_x == 1) && (_y == 1)) ||
                    ((_x == 8) && (_y == 1)) ||
                    ((_x == 1) && (_y == 8)) ||
                    ((_x == 8) && (_y == 8)) {
                        // 4隅はスコアを上げる
                        c = 12;
                }
                else if
                    ((_x == 2) && (_y == 1)) ||
                    ((_x == 1) && (_y == 2)) ||
                    ((_x == 2) && (_y == 2)) ||
                    ((_x == 7) && (_y == 1)) ||
                    ((_x == 8) && (_y == 2)) ||
                    ((_x == 7) && (_y == 2)) ||
                    ((_x == 1) && (_y == 7)) ||
                    ((_x == 2) && (_y == 8)) ||
                    ((_x == 2) && (_y == 7)) ||
                    ((_x == 8) && (_y == 7)) ||
                    ((_x == 7) && (_y == 8)) ||
                    ((_x == 7) && (_y == 7)) {
                        // 4隅の隣はスコアを下げる
                        c = -4;
                }

                coefs.push(c);
            }
        }

        return Board {
            pieces: pieces,
            coefs: coefs
        };
    }

    // コンソールに表示する
    pub fn print(&self) -> () {
        println!("   A B C D E F G H");
        println!(" -----------------");
        for y in 1..=8 {
            print!("{}|", y);
            for x in 1..=8 {
                if let Some(idx) = Pos::idx(x, y) {
                    let piece = &self.pieces[idx];
                    let pstr = match piece {
                        Piece::White => "●",
                        Piece::Black => "○",
                        _ => "・"
                    };
                    print!("{}", pstr);    
                }
            }
            println!("|{}", y);
        }
        println!(" -----------------");
        println!("   A B C D E F G H");
    }

    // 初期状態にする
    pub fn init(&mut self) -> () {
        self.setPiece(4, 4, Piece::White);
        self.setPiece(5, 5, Piece::White);
        self.setPiece(4, 5, Piece::Black);
        self.setPiece(5, 4, Piece::Black);
    }

    pub fn load(&mut self, boardPath: &str) -> bool {
        if let Some(lines) = self.readTextFile(boardPath).ok() {
            for i in 2..10 {
                let line = lines[i].chars().collect::<Vec<char>>();
                for j in 2..10 {
                    let ch = line[j];
                    let piece = match ch {
                        '●' => Piece::White,
                        '○' => Piece::Black,
                        _ => Piece::Space
                    };
                    let x = (j-1) as i32;
                    let y = (i-1) as i32;
                    self.setPiece(x, y, piece);
                }
            }
            return true;
        }

        return false;
    }

    // ------------------------------------------------
    // テキストファイルを読み込んで文字列ベクトルで返す
    //
    // @param fname 読み込むファイル
    //
    // @return Result<Vec<String>> 各行の内容
    // ------------------------------------------------
    pub fn readTextFile(&self, fname: &str) -> std::io::Result<Vec<String>> {
        let f = File::open(fname)?;
        let reader = BufReader::new(f);
    
        let mut lines = Vec::<String>::new();
    
        for line in reader.lines() {
            lines.push(line.unwrap());
        }
    
        Ok(lines)
    }
    
    pub fn setPiece(&mut self, x: i32, y: i32, piece: Piece) -> () {
        if let Some(idx) = Pos::idx(x, y) {
            self.pieces[idx] = piece;
        }
    }

    pub fn getPiece(&self, x: i32, y: i32) -> Option<&Piece> {
        if let Some(idx) = Pos::idx(x, y) {
            return Some(&self.pieces[idx]);
        } else {
            return None;
        }
    }

    pub fn getCoef(&self, x: i32, y: i32) -> i32 {
        if let Some(idx) = Pos::idx(x, y) {
            return self.coefs[idx];
        } else {
            return 0;
        }
    }

    // pieceが次に置ける場所を全て探す
    pub fn searchPos(&self, piece: &Piece) -> Vec<SearchResult1> {
        let mut result = Vec::new();
        for y in 1..=8 {
            for x in 1..=8 {
                if let Some(p) = self.getPiece(x, y) {
                    if *p == Piece::Space {
                        if let Some(res) = self.searchPosSub(piece, &Pos{x, y}) {
                            result.push(res);
                        }
                    }
                }
            }
        }
        return result;
    }

    // posにpieceを置いたら何個相手の駒を取れるかを返す
    //
    // @param piece [i]
    // @param pos [i] 置く位置
    //
    // @return posにpieceを置けない場合はNoneが返る
    pub fn searchPosSub(&self, piece: &Piece, pos: &Pos) -> Option<SearchResult1> {
        let opponent = Piece::getOpponent(piece);
        let mut ntake = 0;
        let mut score = 0;
        let mut dirs = vec!();

        if let Some(p) = self.getPiece(pos.x, pos.y) {
            if *p == Piece::Space {
                for dir in 0..8 {
                    let (dx, dy) = Pos::getDxDy(dir);
                    let mut x1 = pos.x + dx;
                    let mut y1 = pos.y + dy;
                    if let Some(q) = self.getPiece(x1, y1) {
                        if *q == opponent {
                            // (dx, dy)に進めたら敵の駒があった
                            // さらに進めて、空白になる前に自分の駒があれば、(x, y)にpieceを置ける
                            let mut n = 1;
                            let mut c = self.getCoef(x1, y1);

                            // さらに進める
                            x1 = x1 + dx;
                            y1 = y1 + dy;
                            let mut proceeding = false;
                            let mut got = false;
                            if let Some(r) = self.getPiece(x1, y1) {
                                if *r == opponent {
                                    proceeding = true;  // 続いている
                                }
                                else if *r == *piece {
                                    got = true; // 囲んだ
                                }
                            }
                            while proceeding {
                                n = n + 1;
                                c = c + self.getCoef(x1, y1);

                                x1 = x1 + dx;
                                y1 = y1 + dy;
                                proceeding = false;
                                got = false;
                                if let Some(r) = self.getPiece(x1, y1) {
                                    if *r == opponent {
                                        proceeding = true;
                                    }
                                    else if *r == *piece {
                                        got = true;
                                    }
                                }
                            }
                            if got {
                                // 囲んだ
                                ntake = ntake + n;
                                score = score + c;
                                dirs.push(dir);
                            }
                        }
                    }
                }
            }
        }

        if ntake>0 {
            score += self.getCoef(pos.x, pos.y);
            Some(SearchResult1 { 
                pos: pos.clone(),
                ntake: ntake, 
                score: score, 
                dirs: dirs 
            })
        } else {
            None
        }
    }

    // pieceを置ける位置を探し、
    // そこに置いた場合の新しい盤のリストを返す
    //
    // 返り値のスコア(score)は
    // pieceにとっての得点．
    //
    // scoreは負の値になることもある．
    // pieceにとって不利になる場合に置いた場合（例：四隅の斜め隣りに置いた場合など）
    //
    // 返り値のcapturedPieceLocsはひっくり返された駒の位置（アニメーション用）
    pub fn genNextBoards(&self, piece: &Piece) -> Vec<SearchResult2> {

        // 現在のボードに、pieceを置ける場所を探す
        let places = self.searchPos(&piece);

        let mut results = vec!();
        if places.len() == 0 {
            // 1個も置けるところはなかった
            return results;
        }

        // 置けるところがあった
        // println!("possible next moves for {}:", piece.to_str());
        for pi in &places {
            let optResult = self.put(piece, &pi.pos);
            let freedom = places.len() as i32; // 自由度（置ける場所の数）大きいほど有利
            let mut result = optResult.unwrap();
            result.score *= freedom;
            // println!("{} score:{}", Pos::toDesc(pi.pos.x, pi.pos.y), result.score);
            results.push(result);
        }
        // println!("done");

        return results;
    }

    // 駒を指定位置に置く
    // 囲まれた駒は反転され、新しい盤が返ってくる
    // 指定位置に置けない場合はNoneが返る
    //
    // @param piece [i] 駒
    // @param pos [i] 位置
    pub fn put(&self, piece: &Piece, pos: &Pos) -> Option<SearchResult2> {
        // 相手の駒
        let opponent = Piece::getOpponent(piece);

        let optRes = self.searchPosSub(piece, pos);
        if optRes.is_none() {
            // posには置けない
            return None;
        }
        let res = optRes.unwrap();

        let mut newBoard = self.clone();
        newBoard.setPiece(pos.x, pos.y, *piece);

        let mut capturedPieceLocs = vec!();
        // (pi.x, pi.y)からres.dirs方向に置ける
        for dir in &res.dirs {
            // println!("dir={}", dir);
            let (dx, dy) = Pos::getDxDy(*dir);
            let mut x1 = pos.x + dx;
            let mut y1 = pos.y + dy;
            let mut p = self.getPiece(x1, y1).unwrap(); // (x1, y1)に置けることがわかっているのでunwrapでok
            while *p == opponent {
                // (x1, y1)は敵の駒だった．
                // (x1, y1)を自分の駒にする
                newBoard.setPiece(x1, y1, *piece);
                capturedPieceLocs.push(Pos {x: x1, y: y1});

                // さらに進める
                x1 = x1 + dx;
                y1 = y1 + dy;
                p = self.getPiece(x1, y1).unwrap();
            }
        }

        Some(SearchResult2 {
            pos: pos.clone(),
            piece: piece.clone(),
            board: newBoard,
            ntake: res.ntake,
            score: res.score,
            capturedPieceLocs: capturedPieceLocs
        })
    }

    // pieceの手番でdepth手先まで読む
    pub fn genSearchTree(&self, piece: &Piece, depth: i32) -> Vec<SearchResult3> {
        let root = SearchResult3 {
            path: vec!(),
            board: self.clone(),
            ntake: 0,
            score: 0
        };
        return self.genSearchTreeSub(piece, piece, depth, &root);
    }

    pub fn genSearchTreeSub(&self, origPiece: &Piece, piece: &Piece, depth: i32, tree: &SearchResult3) -> Vec<SearchResult3> {
        let mut results = vec!();

        if depth > 0 {
            let nextBoards: Vec<SearchResult2> = self.genNextBoards(piece);
            if nextBoards.len() == 0 {
                // println!("no next board found! for {}", piece.to_str());
                // self.print();
                results.push(SearchResult3 {
                    path: tree.path.clone(),
                    board: tree.board.clone(),
                    ntake: tree.ntake,
                    score: tree.score
                });
                return results;
            }

            // nextBoardsから最もスコアの高いものを選び出す（複数個あり得る）
            let maxScore = nextBoards.iter().max_by_key(|&elem| elem.score).unwrap().score;
            let maxScoreBoards = nextBoards.iter().filter_map(
                |elem| if elem.score == maxScore {
                    return Some(elem);
                } else {
                    return None;
                }
            );

            for nextBoard in maxScoreBoards {   // ここで&maxScoreBoardsにできないのはなぜ？
                let np = nextBoard.pos.clone();
                // for _ in 0..depth {
                //     print!(" ");
                // }
                // println!("{}", Pos::toDesc(np.x, np.y));
                let mut newPath = tree.path.clone();
                newPath.push(SearchResult3Sub {
                    pos: nextBoard.pos,
                    piece: *piece
                });

                let mut newNtake = tree.ntake;
                let mut newScore = tree.score;
                if piece == origPiece {
                    // pieceは自分の駒
                    newNtake += nextBoard.ntake;
                    newScore += nextBoard.score;
                } else {
                    // pieceは敵の駒
                    newNtake -= nextBoard.ntake;
                    newScore -= nextBoard.score;
                }

                let newDepth = depth - 1;
                if newDepth > 0 {
                    let newTree = SearchResult3 {
                        path: newPath,
                        board: nextBoard.board.clone(),
                        ntake: newNtake,
                        score: newScore
                    };
                    let childResults = nextBoard.board.genSearchTreeSub(
                        origPiece,
                        &Piece::getOpponent(piece),
                        newDepth,
                        &newTree
                    );
                    for childResult in &childResults {
                        results.push(childResult.clone());
                    }
                } else {
                    results.push(SearchResult3 {
                        path: newPath,
                        board: nextBoard.board.clone(),
                        ntake: newNtake,
                        score: newScore
                    })
                }
            }
        }

        return results;
    }

    // 最善の手を探す
    pub fn getBestMove(&self, piece: &Piece, depth: i32) -> Option<SearchResult3> {
        let mut bestMove = None;

        let allMoves: Vec<SearchResult3> = self.genSearchTree(piece, depth);
        println!("{} moves", allMoves.len());
        let mut bestScore = std::i32::MIN;
        for m in &allMoves {
            // let n = m.path.len();
            // for i in 0..n {
            //     print!("{}", Pos::toDesc(m.path[i].pos.x, m.path[i].pos.y));
            //     if i<n-1 {
            //         print!("-");
            //     }
            // }
            // println!(" score:{}", m.score);

            if m.score > bestScore {
                bestScore = m.score;
                bestMove = Some(m.clone());
            }
        }

        return bestMove;
    }

    // 白、黒が盤上に何個あるか数える
    pub fn getCount(&self) -> Count {
        let mut nWhitePieces = 0;
        let mut nBlackPieces = 0;

        for y in 1..=8 {
            for x in 1..=8 {
                if let Some(idx) = Pos::idx(x, y) {
                    let piece = &self.pieces[idx];
                    match piece {
                        Piece::White => {
                            nWhitePieces = nWhitePieces + 1;
                        },
                        Piece::Black => {
                            nBlackPieces = nBlackPieces + 1;
                        },
                        _ => {
                            // do nothing
                        }
                    };
                }
            }
        }

        Count { nWhitePieces, nBlackPieces }
    }

    pub fn printScore(&self) {
        let score = self.getCount();
        println!("●={}, ○={}", score.nWhitePieces, score.nBlackPieces);
    }
}
