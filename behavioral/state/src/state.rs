use cursive::views::TextView;

use crate::player::Player;

pub struct StoppedState;

pub struct PausedState;

pub struct PlayingState;

/// There is a base `State` trait with methods `play` and `stop` which make
/// state transitions. There are also `next` and `prev` methods in a separate
/// `impl dyn State` block below, those are default implementations
/// that can not be override.
///
/// What is the `self: Box<Self>` notation? We use the state also follows:
///```rust
/// let prev_state = Box::new(PlayingState);
/// let next_state = prev_sate.play(&mut player);
///```
/// A method `play` receives a whole `Box<PlayingState>` object,
/// and not just `PlayingSate`. The previous state "disappears" in the method,
/// in turn, it returns a new `Box<PauseState>` state object.
pub trait State {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State>;
    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State>;
    fn render(&self, player: &Player, view: &mut TextView);
}

impl State for StoppedState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.play();

        // Stopped -> Playing.
        Box::new(PlayingState)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        self
    }

    fn render(&self, player: &Player, view: &mut TextView) {
        view.set_content("[Stopped] Press 'Play'")
    }
}

impl State for PausedState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.pause();

        // Paused -> Playing.
        Box::new(PlayingState)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.pause();
        player.rewind();

        // Paused -> Stopped.
        Box::new(StoppedState)
    }

    fn render(&self, player: &Player, view: &mut TextView) {
        view.set_content(format!(
            "[Paused] {} - {} sec",
            player.track().title,
            player.track().duration
        ))
    }
}

impl State for PlayingState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.pause();

        // Playing -> Paused
        Box::new(PausedState)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.pause();
        player.rewind();

        // Playing -> Stopped
        Box::new(StoppedState)
    }

    fn render(&self, player: &Player, view: &mut TextView) {
        view.set_content(format!(
            "[Playing] {} - {} sec",
            player.track().title,
            player.track().duration
        ))
    }
}

/// Default `next` and `prev` implementation for the trait
impl dyn State {
    pub fn next(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.next_track();
        self
    }

    pub fn prev(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.prev_track();

        // Change no state.
        self
    }
}