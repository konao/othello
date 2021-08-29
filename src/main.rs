use othello::board;

fn main() {
    let mut board = board::Board::new();
    board.init2();
    board.print();

    println!("** White **");
    let result = board.searchPos(&board::Piece::White);
    for info in result.iter() {
        println!("({}, {}) = {}", info.x, info.y, info.score);
    }

    println!("** Black **");
    let result = board.searchPos(&board::Piece::Black);
    for info in result.iter() {
        println!("({}, {}) = {}", info.x, info.y, info.score);
    }
}
    
