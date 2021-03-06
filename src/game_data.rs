use crate::{
    mino::*,
    block::*,
    mino_rotation::*
};

#[derive(Debug)]
pub struct GameData {
    pub minos: Vec<Mino>,
    pub field: Vec<Vec<Block>>,
    pub control_mino: Option<Mino>,
    pub mino_pos: (i32, i32),
    pub next_minos: Vec<Mino>,
    pub mino_rotation: MinoRotation,
    pub field_size: (usize, usize),
    pub show_ghost: bool,
    pub hold_mino: Option<Mino>,
    pub score: i64,
    pub game_speed: f32,
}

impl GameData {
    pub fn new(field_size: (usize, usize), show_ghost: bool) -> GameData {
        let minos: Vec<Mino> = vec![
            Mino {
                id: String::from("I"),
                shape: vec![
                            vec![false, false, false ,false],
                            vec![true, true, true ,true],
                            vec![false, false, false ,false],
                            vec![false, false, false ,false],
                        ]
            },
            Mino { 
                id: String::from("J"), 
                shape: vec![
                            vec![true, false, false],
                            vec![true, true, true],
                            vec![false, false, false],
                        ] 
            },
            Mino { 
                id: String::from("L"), 
                shape: vec![
                            vec![false ,false ,true],
                            vec![true ,true ,true],
                            vec![false ,false ,false],
                        ]
            },
            Mino { 
                id: String::from("S"), 
                shape: vec![
                            vec![false ,true ,true],
                            vec![true ,true ,false],
                            vec![false ,false ,false],
                        ]
            },
            Mino { 
                id: String::from("Z"), 
                shape: vec![
                            vec![true ,true ,false],
                            vec![false ,true ,true],
                            vec![false ,false ,false],
                        ]
            },
            Mino { 
                id: String::from("T"), 
                shape: vec![
                            vec![false ,true ,false],
                            vec![true ,true ,true],
                            vec![false ,false ,false],
                        ]
            },
            Mino { 
                id: String::from("O"), 
                shape: vec![
                            vec![true ,true],
                            vec![true ,true],
                        ]
            }
        ];

        let field: Vec<Vec<Block>> = vec![vec![Block::Air; field_size.0]; field_size.1];

        let next_minos = minos.clone();

        GameData { 
            minos, 
            field, 
            control_mino: None, 
            mino_pos: (0, 0),
            next_minos,
            mino_rotation: MinoRotation::Up,
            field_size,
            show_ghost,
            hold_mino: None,
            score: 0,
            game_speed: 0.5,
        }
    }
}
