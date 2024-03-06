// use chess_game_engine;
//

trait Jk {
    fn pok(&self) {
        println!("joker");
    }
}

trait Lk {
    fn pok(&self) {
        println!("Loker");
    }
}
struct Pk {}

impl Pk {
    fn pok(&self) {
        println!("poker");
    }
}

fn main() {
    let pk = Pk {};
    pk.pok();
}
