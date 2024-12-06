use async_recursion::async_recursion;
use core::array::from_fn;
use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

//enum BoardTransformation {
//    None,
//    FlipSides { old_player_deltas: (isize, isize) },
//}

const SPACES_PER_PLAYER: usize = 6;
const MARBLES_PER_SPACE: usize = 4;
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Player {
    PlayerOne,
    PlayerTwo,
}
impl Player {
    fn next(self) -> Self {
        match self {
            Self::PlayerOne => Self::PlayerTwo,
            Self::PlayerTwo => Self::PlayerOne,
        }
    }
    fn to_u(&self) -> usize {
        match self {
            Self::PlayerOne => 0,
            Self::PlayerTwo => 1,
        }
    }

    fn from_u(i: usize) -> Result<Self, &'static str> {
        match i {
            0 => Ok(Player::PlayerOne),
            1 => Ok(Player::PlayerTwo),
            _ => Err("Not a Valid Turn"),
        }
    }
}
#[derive(Clone)]
pub struct Turn {
    player: Player,
    space_num: usize,
}

#[derive(Copy, Clone)]
pub struct MancalaBoard {
    player_to_move: Player,
    spaces: [[usize; SPACES_PER_PLAYER]; 2],
    mancalas: [usize; 2],
}

pub struct MancalaSearchBoard {
    turn: usize,
    origin: bool,
    terminal: bool,
    utility: Option<isize>,
    board: MancalaBoard,
    children: Option<Vec<Box<MancalaSearchBoard>>>,
    //parents: Vec<(Arc<Mutex<MancalaSearchBoard>>, BoardTransformation)>,
}

impl MancalaBoard {
    pub fn starting_board() -> Self {
        MancalaBoard {
            player_to_move: Player::PlayerOne,
            spaces: from_fn(|_| from_fn(|_| MARBLES_PER_SPACE)),
            mancalas: from_fn(|_| 0),
        }
    }
    pub fn generate_legit_turns(&self) -> Vec<Turn> {
        (1..SPACES_PER_PLAYER - 1)
            .map(|space_num| Turn {
                player: self.player_to_move,
                space_num,
            })
            .filter(|turn| self.turn_vibe_check(turn))
            .collect()
    }
    pub fn turn_vibe_check(&self, turn: &Turn) -> bool {
        let space_num: usize = turn.space_num;

        turn.player == self.player_to_move
            && space_num < SPACES_PER_PLAYER
            && self.spaces[turn.player.to_u()][space_num] != 0
    }

    pub fn apply_turn_ip(&mut self, turn: Turn) {
        assert!(self.turn_vibe_check(&turn));

        let mut space_num: usize = turn.space_num;
        let mut marbles: usize = self.spaces[turn.player.to_u()][turn.space_num];
        let mut space_player: Player = turn.player;

        while marbles > 1 {
            match (space_num + 1).cmp(&SPACES_PER_PLAYER) {
                Ordering::Greater => {
                    space_num = 0;
                    if space_player == turn.player {
                        self.mancalas[space_player.to_u()] += 1;
                    }
                    space_player = space_player.next()
                }
                _ => {
                    self.spaces[space_player.to_u()][space_num] += 1;
                    space_num += 1;
                    marbles -= 1;
                }
            };
        }
    }

    pub fn apply_turn_cp(&self, turn: Turn) -> Self {
        let mut out: MancalaBoard = self.clone();
        out.apply_turn_ip(turn);
        out
    }
}

impl MancalaSearchBoard {
    pub fn apply_turn_cp(&self, turn: Turn) -> Self {
        MancalaSearchBoard {
            turn: self.turn + 1,
            origin: false,
            terminal: false,
            utility: None,
            board: self.board.apply_turn_cp(turn),
            children: None,
        }
    }
    pub fn default(board: MancalaBoard) -> Self {
        MancalaSearchBoard {
            turn: 1,
            origin: false,
            terminal: false,
            utility: None,
            board,
            children: None,
        }
    }

    pub fn origin(board: MancalaBoard) -> Self {
        MancalaSearchBoard {
            turn: 1,
            origin: true,
            terminal: false,
            utility: None,
            board,
            children: None,
        }
    }

    pub fn make_babies(&mut self, limit: usize) {
        let turns = self.board.generate_legit_turns();
        if turns.len() == 0 {
            self.terminal = true;
            self.children = None;
        } else if self.turn == limit {
        } else {
            self.children = Some(
                turns
                    .into_iter()
                    .map(|turn| Box::new(self.apply_turn_cp(turn)))
                    .collect(),
            );
        }
    }

    #[async_recursion]
    pub async fn build_trees(&mut self, limit: usize) {
        self.make_babies(limit);

        if self.terminal && self.children.is_none() {
        } else {
            match self.children.as_mut() {
                Some(children) => {
                    for child in children {
                        child.deref_mut().build_trees(limit).await
                    }
                }
                None => {}
            };
        }
    }

    #[async_recursion]
    pub async fn build_trees_async(&mut self, limit: usize) {
        self.make_babies(limit);

        if self.terminal && self.children.is_none() {
        } else {
            match self.children.as_mut() {
                Some(children) => {
                    for child in children {
                        child.deref_mut().build_trees(limit).await
                    }
                }
                None => {}
            };
        }
    }

    //fn evaluate_self_worth_from_children(&mut self) {
    //    self.children.unwrap().into_iter().map(|child| child.deref().make_babies());
    //
    //    if self.terminal && self.utility == None && !self.children.is_none() {
    //        match self.board.player_to_move {
    //            Player::PlayerOne => ,
    //            Player::PlayerTwo => ,
    //        }
    //    } else {
    //        self.utility = None;
    //    }
    //}
}
