use cursive::Cursive;
use cursive::event::Key;
use cursive::view::Nameable;
use cursive::views::{Dialog, TextView};

use crate::player::Player;
use crate::state::{State, StoppedState};

mod state;
mod player;

// Application context: a music player and a state.
struct PlayerApplication {
    player: Player,
    state: Box<dyn State>,
}

fn main() {
    let mut app = cursive::default();

    app.set_user_data(PlayerApplication {
        player: Player::default(),
        state: Box::new(StoppedState),
    });

    app.add_layer(
        Dialog::around(TextView::new("Press Play").with_name("Play Status"))
            .title("Music Player")
            .button("Play", |s| execute(s, "Play"))
            .button("Stop", |s| execute(s, "Stop"))
            .button("Prev", |s| execute(s, "Prev"))
            .button("Next", |s| execute(s, "Next"))
    );

    app.add_global_callback(Key::Esc, |s| s.quit());
    app.run();
}

fn execute(s: &mut Cursive, button: &'static str) {
    let PlayerApplication {
        mut player,
        mut state,
    } = s.take_user_data().unwrap();

    let mut view = s.find_name::<TextView>("Player Status").unwrap();

    state = match button {
        "Play" => state.play(&mut player),
        "Stop" => state.stop(&mut player),
        "Prev" => state.prev(&mut player),
        "Next" => state.next(&mut player),
        _ => unreachable!(),
    };

    state.render(&player, &mut view);
    s.set_user_data(PlayerApplication { player, state })
}

