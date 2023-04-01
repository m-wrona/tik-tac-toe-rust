mod game;

fn main() {
    println!("Hello, world!");
    let s0 = game::game::State::new(1, 2);
    println!("{:?}", s0)
}
