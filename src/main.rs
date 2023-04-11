mod game;
mod ai_player;

fn main() {
    println!("Hello, world!");
    let s0 = game::State::new(1, 2);
    println!("{:?}", s0)
}
