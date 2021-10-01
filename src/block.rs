#[derive(Debug, Clone, Copy)]
pub enum Block {
    Air,
    Block,
    Ghost
}

impl Block {
    pub fn show(self) -> String {
        match self {
            Block::Air => String::from("・"),
            Block::Block => String::from("■"),
            Block::Ghost => String::from("□"),
        }
    }
}
