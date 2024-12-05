use std::{
    cmp::Ordering,
    ops::Deref,
    sync::{Arc, Mutex},
};

//enum BoardTransformation {
//    None,
//    FlipSides { old_player_deltas: (isize, isize) },
//}

const SPACES_PER_PLAYER: usize = 6;
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
struct Turn {
    player: Player,
    space_num: usize,
}

#[derive(Copy, Clone)]
struct MancalaBoard {
    player_to_move: Player,
    spaces: [[usize; SPACES_PER_PLAYER]; 2],
    mancalas: [usize; 2],
}

struct MancalaSearchBoard {
    origin: bool,
    board: MancalaBoard,
    turn: Option<Turn>,
    children: Vec<Arc<Mutex<MancalaSearchBoard>>>,
    //parents: Vec<(Arc<Mutex<MancalaSearchBoard>>, BoardTransformation)>,
}

impl MancalaBoard {
    fn apply_turn_ip(&mut self, turn: Turn) {
        let mut space_num: usize = turn.space_num;
        let mut marbles: usize = self.spaces[turn.player.to_u()][turn.space_num];
        let mut space_player: Player = turn.player;

        assert!(space_num < SPACES_PER_PLAYER);

        while marbles > 1 {
            match space_num.cmp(&SPACES_PER_PLAYER) {
                Ordering::Greater => {
                    space_num = 0;
                    if space_player == turn.player {
                        self.mancalas[space_player.to_u()] += 1;
                    }
                    space_player = space_player.next()
                }
                _ => {
                    space_num += 1;
                    marbles -= 1;
                    self.spaces[space_player.to_u()][space_num] += 1;
                }
            };
        }
    }

    fn apply_turn_cp(&self, turn: Turn) -> Self {
        let mut out: MancalaBoard = self.clone();
        out.apply_turn_ip(turn);
        out
    }
}
impl MancalaSearchBoard {
    fn search(&self) {}
}
