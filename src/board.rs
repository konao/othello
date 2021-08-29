#![allow(non_snake_case)]

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
    x: u8,  // 1..8
    y: u8   // 1..8
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaceInfo {
    pub x: i32,
    pub y: i32,
    pub score: i32
}

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
        for y in 1..=8 {
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

    // pidが次に置ける場所を探す
    pub fn searchPos(&self, piece: &Piece) -> Vec<PlaceInfo> {
        let mut result = Vec::new();
        for y in 1..=8 {
            for x in 1..=8 {
                if let Some(p) = self.getPiece(x, y) {
                    if *p == Piece::Space {
                        let score = self.searchPosSub(piece, x, y);
                        if score>0 {
                            let info = PlaceInfo {
                                x: x,
                                y: y,
                                score: score
                            };
                            result.push(info);
                        }
                    }
                }
            }
        }
        return result;
    }

    // (x, y)にpieceを置いたら何個相手の駒を取れるかを返す
    // @param x [i]
    // @param y [i]
    pub fn searchPosSub(&self, piece: &Piece, x: i32, y: i32) -> i32 {
        let opponent = Piece::getOpponent(piece);
        let mut score = 0;
        for dir in 0..8 {
            let (dx, dy) = Pos::getDxDy(dir);
            let mut x1 = x + dx;
            let mut y1 = y + dy;
            if let Some(p) = self.getPiece(x1, y1) {
                if *p == opponent {
                    // (dx, dy)に進めたら敵の駒があった
                    // さらに進めて、空白になる前に自分の駒があれば、(x, y)にpieceを置ける
                    let mut c = 1;
                    x1 = x1 + dx;
                    y1 = y1 + dy;
                    let mut proceeding = false;
                    let mut got = false;
                    if let Some(q) = self.getPiece(x1, y1) {
                        if *q == opponent {
                            proceeding = true;  // 続いている
                        }
                        else if *q == *piece {
                            got = true; // 囲んだ
                        }
                    }
                    while proceeding {
                        c = c + 1;
                        x1 = x1 + dx;
                        y1 = y1 + dy;
                        proceeding = false;
                        got = false;
                        if let Some(q) = self.getPiece(x1, y1) {
                            if *q == opponent {
                                proceeding = true;
                            }
                            else if *q == *piece {
                                got = true;
                            }
                        }
                    }
                    if got {
                        score += c;
                    }
                }
            }
        }

        return score;
    }

    // 駒pidを位置pに置く、
    // ひっくり返せる駒をひっくり返し、新しい盤を返す
    //
    // 返り値のu8はスコア(score)
    // pidにとっての得点．
    // score=1 --> pidが1個増える
    // pidにとって不利になる場合はマイナスの値になる（例：四隅の斜め隣りに置いた場合など）
    //
    // 返り値の[Pos]はひっくり返された駒の位置（アニメーション用）
    // pub fn put(&self, pid: PID, p: Pos) -> (Board, u8, [Pos]) {

    // }
}