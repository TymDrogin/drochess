use crate::gamestate::*;

pub struct History {
    list: Vec<Gamestate>
}
impl History {
    pub fn new() -> Self {
        Self {
            list: Vec::new()
        }
    }
    pub fn push(&mut self, game:Gamestate) {
        self.list.push(game);
    }
    pub fn pop(&mut self) {
        self.list.pop();
    }
    pub fn get_last(&self) -> &Gamestate {
        &self.list[self.list.len() - 1]
    }

}