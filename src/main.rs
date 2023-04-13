mod game;
mod player;
mod player_ai;

fn main() {
    println!("Hello, world!");
    let s0 = game::State::new(1, 2);
    println!("{:?}", s0)
}
