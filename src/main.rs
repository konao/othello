use othello::board;

fn main() {
    let mut board = board::Board::new();
    board.init();
    board.print();
}
