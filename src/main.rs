use crate::game::NO_PLAYER;
use crate::player::Player;
use crate::player_ai::AIPlayer;
use crate::player_human::HumanPlayer;

mod game;
mod player;
mod player_ai;
mod player_human;

fn main() {
    let human = HumanPlayer::new(1);
    let ai = AIPlayer::new_with_strategy(2, AIPlayer::random_move_strategy());

    let players: [&dyn Player; 2] = [&human, &ai];

    let mut state = game::State::new(human.id(), ai.id());

    loop {
        for p in players {
            println!("{:?}: ", state.board());
            let (winner, is_finished) = state.is_finished();
            if is_finished {
                if winner == NO_PLAYER {
                    println!("Game finished - it's a draw!");
                } else {
                    println!("Game finished - player {} won!", winner);
                }
                return;
            }

            match p.next_move(state.board()) {
                Ok(coordinate) => {
                    match state.make_move(p.id(), coordinate) {
                        Ok(newState) => {
                            state = newState;
                        }

                        Err(err) => {
                            println!("Player {} error: {}", p.id(), err)
                        }
                    }
                }

                Err(err) => {
                    println!("Next move error: {}", err)
                }
            }
        }
    }
}
