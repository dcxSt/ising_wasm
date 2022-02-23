#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
pub struct Cellule {
    pub state: State,
}

impl Cellule {
    pub fn new_dead() -> Self {
        Self { state: State::Dead }
    }

    pub fn set_alive(&mut self) {
        self.state = State::Alive;
    }

    pub fn set_dead(&mut self) {
        self.state = State::Dead;
    }

    pub fn is_alive(self) -> bool {
        self.state == State::Alive
    }

    pub fn toggle(&mut self) {
        if self.is_alive() {
            self.set_dead()
        } else {
            self.set_alive()
        }
    }
}
