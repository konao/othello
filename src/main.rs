use othello::board;

fn main() {
    let mut board = board::Board::new();
    board.init();
    board.print();

    let result = board.searchPos(&board::Piece::White);
    for info in result.iter() {
        println!("({}, {}) = {}", info.x, info.y, info.score);
    }
}
    
