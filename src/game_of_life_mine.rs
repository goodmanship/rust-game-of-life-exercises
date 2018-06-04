pub trait GameOfLife {

    /// Return `Some(true)` if the cell is alive, `Some(false)` if it is dead, or `None` if `x`
    /// and/or `y` are out of bounds.
    fn is_cell_alive(&self, x: i32, y: i32) -> Option<bool>;

    /// Swap the given cell from alive to dead or dead to alive.
    ///
    /// If `x` or `y` is out of bounds, this method should do nothing.
    ///
    /// The origin is assumed to be at the top left, i.e. when `(x, y) == (0, 0)` then the top-left-most
    /// cell should be toggled.
    fn toggle_cell(&mut self, x: i32, y: i32);

    /// Execute one timestep; i.e. cause cells to live, be born, or die based on the amount of
    /// neighbors they have.
    fn tick(&mut self);

    /// Return the current width in cells of the game.
    fn width(&self) -> u32;

    /// Return the current height in cells of the game.
    fn height(&self) -> u32;
}

/// A blatantly-wrong implementation of GameOfLife, to show the syntax for implementing traits.
pub struct Game {
    cell_state: bool,
}

impl Game {
    pub fn new() -> Self {
        Game { cell_state: true }
    }
}

impl GameOfLife for Game {
    fn is_cell_alive(&self, _x: i32, _y: i32) -> Option<bool> {
        Some(self.cell_state)
    }

    fn toggle_cell(&mut self, _x: i32, _y: i32) {
        // Toggle the only cell we have
        self.cell_state = !self.cell_state;
    }

    fn tick(&mut self) {
        self.cell_state = !self.cell_state;

        println!(
            "Broken game tick completed - cell_state is now {}",
            self.cell_state
        );
    }

    fn width(&self) -> u32 {
        49 // broken implementation always lies about the width
    }

    fn height(&self) -> u32 {
        40 // broken implementation always lies about the height
    }
}

#[cfg(test)]
mod broken_game_test {
    use super::{BrokenGame, GameOfLife};

    #[test]
    fn broken_game_is_definitely_broken() {
        let mut game = BrokenGame::new();
        let cell_0_0_orig_val = game.is_cell_alive(0, 0);

        // change a totally different cell from 0,0
        game.toggle_cell(1, 1);

        let cell_0_0_new_val = game.is_cell_alive(0, 0);
        // now we expect cell 0,0's liveness to have changed because we know that BrokenGame
        // is a totally broken implementation. If the two values are equal, then something seriously
        // weird is going on.
        // Tip: `assert_ne!` means "assert not equal" - normally using `assert!` or `assert_eq!` is typical.
        assert_ne!(cell_0_0_orig_val, cell_0_0_new_val, "Uh oh, cell 0,0 failed to change from its \
        original value even though we tried to mutate another cell, so BrokenGame is may not be \
        broken anymore!?");
    }
}
