#![allow(non_snake_case)]

pub enum Piece {
    Space,
    White,
    Black
}

pub struct Pos {
    x: u8,  // 1..8
    y: u8   // 1..8
}

impl Pos {
    // Pos構造体からBoardのインデックスを返す
    //
    // (ex)
    pub fn toIdx(&self) -> usize {
        return ((self.y-1) * 8 + (self.x-1)).into();
    }

    pub fn idx(x: i32, y: i32) -> usize {
        return ((y-1) * 8 + (x-1)) as usize;
    }
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
                let idx = Pos::idx(x, y);
                let piece = &self.pieces[idx];
                let pstr = match piece {
                    Piece::White => "○",
                    Piece::Black => "●",
                    _ => "・"
                };
                print!("{}", pstr);
            }
            println!();
        }
    }

    // 初期状態にする
    pub fn init(&mut self) -> () {
        self.pieces[Pos::idx(4, 4)] = Piece::White;
        self.pieces[Pos::idx(5, 5)] = Piece::White;
        self.pieces[Pos::idx(4, 5)] = Piece::Black;
        self.pieces[Pos::idx(5, 4)] = Piece::Black;
    }

    pub fn getPiece(&self, p: Pos) -> &Piece {
        let ind = p.toIdx();  // 位置から（配列の）インデックスへ変換
        return &self.pieces[ind];
    }

    // pidが次に置ける場所を探す
    pub fn searchPos(&self, pid: Piece) -> Vec<Pos> {
        return Vec::new();
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