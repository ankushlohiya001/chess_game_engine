use chess_game_engine::{game::Game, moves::Moving};

fn main() {
    let mut game = Game::new();
    game.start();
    game.show_board();

    let pawn = game.pick('c', 2).unwrap();

    pawn.place_at(&mut game, 'c', 3).unwrap();

    game.change_side().unwrap();
    let pawn = game.pick('g', 7).unwrap();

    pawn.place_at(&mut game, 'g', 5).unwrap();

    game.change_side().unwrap();

    let bishop = game.pick('c', 1).unwrap();

    println!("{:?}", bishop.possible_moves());
    bishop.place_back(&mut game);

    let knight = game.pick('b', 1).unwrap();
    println!("{:?}", knight.possible_moves());
    knight.place_at(&mut game, 'a', 3);

    game.show_board();
}
