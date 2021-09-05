#![allow(non_snake_case)]

// 駒
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Space,
    White,
    Black
}

impl Piece {
    pub fn getOpponent(piece: &Piece) -> Piece {
        return match *piece {
            Piece::Space => Piece::Space,
            Piece::White => Piece::Black,
            Piece::Black => Piece::White
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
}

// 探索結果を表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult1 {
    pub pos: Pos,   // 置ける位置
    pub scoreInfo: SearchResult1Sub    // posに置いたときに取れる駒の個数と方向
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult1Sub {
    pub score: i32,
    pub dirs: Vec<i32>  // 相手の駒を取れる方向
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult2 {
    pub pos: Pos,   // posに
    pub piece: Piece,   // pieceを置いたら
    pub board: Board,   // このボードになる
    pub score: i32,
    pub capturedPieceLocs: Vec<Pos>
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult3 {
    pub path: Vec<SearchResult3Sub>,  // 駒を置いたパス
    pub board: Board,   // pathに置いた結果
    pub score: i32
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult3Sub {
    pub pos: Pos,
    pub piece: Piece
}

// オセロ盤
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pieces: Vec<Piece>
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

        return Board {
            pieces: pieces
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
                        Piece::White => "○",
                        Piece::Black => "●",
                        _ => "・"
                    };
                    print!("{}", pstr);    
                }
            }
            println!();
        }
    }

    // 初期状態にする
    pub fn init(&mut self) -> () {
        self.setPiece(4, 4, Piece::White);
        self.setPiece(5, 5, Piece::White);
        self.setPiece(4, 5, Piece::Black);
        self.setPiece(5, 4, Piece::Black);
    }

    pub fn init2(&mut self) -> () {
        self.setPiece(4, 4, Piece::White);
        self.setPiece(5, 5, Piece::Black);
        self.setPiece(4, 5, Piece::Black);
        self.setPiece(5, 4, Piece::Black);
        self.setPiece(6, 5, Piece::Black);
        self.setPiece(6, 6, Piece::White);
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

    // pieceが次に置ける場所を探す
    pub fn searchPos(&self, piece: &Piece) -> Vec<SearchResult1> {
        let mut result = Vec::new();
        for y in 1..=8 {
            for x in 1..=8 {
                if let Some(p) = self.getPiece(x, y) {
                    if *p == Piece::Space {
                        if let Some(res) = self.searchPosSub(piece, &Pos{x, y}) {
                            if res.score>0 {
                                let info = SearchResult1 {
                                    pos: Pos { x, y },
                                    scoreInfo: res
                                };
                                result.push(info);
                            }
                        }
                    }
                }
            }
        }
        return result;
    }

    // (x, y)にpieceを置いたら何個相手の駒を取れるかを返す
    // @param piece [i]
    // @param pos [i] 置く位置
    //
    // @return posにpieceを置けない場合はNoneが返る
    pub fn searchPosSub(&self, piece: &Piece, pos: &Pos) -> Option<SearchResult1Sub> {
        let opponent = Piece::getOpponent(piece);
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
                            let mut c = 1;
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
                                c = c + 1;
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
                                score += c;
                                dirs.push(dir);
                            }
                        }
                    }
                }
            }
        }

        if score>0 {
            Some(SearchResult1Sub { score, dirs })
        } else {
            None
        }
    }

    // 駒pieceを置ける位置を探し、
    // そこに置いた場合の新しい盤のリストを返す
    //
    // 返り値のu8はスコア(score)
    // pieceにとっての得点．
    // score=1 --> pieceが1個増える
    //
    // scoreは負の値になることもある．
    // pieceにとって不利になる場合に置いた場合（例：四隅の斜め隣りに置いた場合など）
    //
    // 返り値の[Pos]はひっくり返された駒の位置（アニメーション用）
    pub fn genNextBoard(&self, piece: &Piece) -> Vec<SearchResult2> {

        // 現在のボードに、pieceを置ける場所を探す
        let places = self.searchPos(&piece);

        let mut results = vec!();
        if places.len() == 0 {
            // 1個も置けるところはなかった
            return results;
        }

        // 置けるところがあった
        // println!("possible next moves:");
        for pi in &places {
            let result = self.put(piece, &pi.pos);
            results.push(result.unwrap());
        }

        return results;
    }

    // 返り値：取った駒の位置
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
            score: res.score,
            capturedPieceLocs: capturedPieceLocs
        })
    }

    pub fn genSearchTree(&self, piece: &Piece, depth: i32) -> Vec<SearchResult3> {
        let root = SearchResult3 {
            path: vec!(),
            board: self.clone(),
            score: 0
        };
        return self.genSearchTreeSub(piece, piece, depth, &root);
    }

    pub fn genSearchTreeSub(&self, origPiece: &Piece, piece: &Piece, depth: i32, tree: &SearchResult3) -> Vec<SearchResult3> {
        let mut results = vec!();

        if depth > 0 {
            let nextBoards = self.genNextBoard(piece);
            for nextBoard in &nextBoards {
                let mut newPath = tree.path.clone();
                newPath.push(SearchResult3Sub {
                    pos: nextBoard.pos,
                    piece: *piece
                });

                let mut newScore = tree.score;
                if piece == origPiece {
                    // pieceは自分の駒
                    newScore += nextBoard.score;
                } else {
                    // pieceは敵の駒
                    newScore -= nextBoard.score;
                }

                let newDepth = depth - 1;
                if newDepth > 0 {
                    let newTree = SearchResult3 {
                        path: newPath,
                        board: nextBoard.board.clone(),
                        score: newScore
                    };
                    return nextBoard.board.genSearchTreeSub(
                        origPiece,
                        &Piece::getOpponent(piece),
                        newDepth,
                        &newTree
                    );
                } else {
                    results.push(SearchResult3 {
                        path: newPath,
                        board: nextBoard.board.clone(),
                        score: newScore
                    })
                }
            }
        }

        return results;
    }

    pub fn getBestMove(&self, piece: &Piece, depth: i32) -> Option<SearchResult3> {
        let mut bestMove = None;

        let allMoves = self.genSearchTree(piece, depth);
        let mut bestScore = -1;
        for m in &allMoves {
            if m.score > bestScore {
                bestScore = m.score;
                bestMove = Some(m.clone());
            }
        }

        return bestMove;
    }
}
