use async_recursion::async_recursion;
use core::array::from_fn;
use std::{cmp::Ordering, ops::DerefMut};

//enum BoardTransformation {
//    None,
//    FlipSides { old_player_deltas: (isize, isize) },
//}

const SPACES_PER_PLAYER: usize = 6;
const MARBLES_PER_SPACE: usize = 4;
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Player {
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
#[derive(Clone, Debug)]
pub struct BoardSpace {
    pub player: Player,
    pub num: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct MancalaBoard {
    pub player_to_move: Player,
    pub spaces: [[usize; SPACES_PER_PLAYER]; 2],
    pub mancalas: [usize; 2],
}

#[derive(Debug)]
pub struct MancalaGameNode {
    pub turn: usize,
    pub terminal: bool,
    pub solved: bool,
    pub utility: Option<isize>,
    pub board: MancalaBoard,
    children: Option<Vec<Box<MancalaGameNode>>>,
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
    pub fn generate_legit_turns(&self) -> Vec<BoardSpace> {
        (0..SPACES_PER_PLAYER)
            .map(|space_num| BoardSpace {
                player: self.player_to_move,
                num: space_num,
            })
            .filter(|turn| self.turn_vibe_check(turn))
            .collect()
    }
    pub fn turn_vibe_check(&self, turn: &BoardSpace) -> bool {
        let space_num: usize = turn.num;

        turn.player == self.player_to_move
            && space_num < SPACES_PER_PLAYER
            && self.spaces[turn.player.to_u()][space_num] != 0
    }

    fn next_space(&self, mut space: BoardSpace) -> BoardSpace {
        match (space.num + 2).cmp(&SPACES_PER_PLAYER) {
            Ordering::Greater => {
                space.num = 0;
                space.player = space.player.next();
            }
            _ => {
                space.num += 1;
            }
        };

        space
    }

    pub fn render_simple(&self) {
        println!("{:?}", self)
    }
    pub fn render(&self) {
        let mut p1_str: String = format!("|{}|", self.mancalas[0]).to_string();
        let mut p2_str: String = format!("|{}|", self.mancalas[1].to_string().len()).to_string();

        let _ = self.spaces[0].iter().zip(self.spaces[1]).map(|(&p1, p2)| {
            let diff = p1 as isize - p2 as isize;
            if diff < 0 {
                p1_str.push_str(format!("{}|", p1).as_str());
                p2_str.push_str(format!("{}{}|", p2, " ".repeat(diff.unsigned_abs())).as_str());
            } else {
                p1_str.push_str(format!("{}{}|", p1, " ".repeat(diff.unsigned_abs())).as_str());
                p2_str.push_str(format!("{}|", p2).as_str());
            }
        });

        p1_str.push_str(format!("|{}|", self.mancalas[1]).as_str());
        p2_str.push_str(format!("|{}|", self.mancalas[0].to_string().len()).as_str());

        p2_str = p2_str.chars().rev().collect::<String>();

        println!("{p2_str}");
        println!("{p1_str}");
    }

    pub fn apply_turn_ip(&mut self, mut space: BoardSpace) {
        assert!(self.turn_vibe_check(&space));

        let mut marbles: usize = self.spaces[space.player.to_u()][space.num];
        self.spaces[space.player.to_u()][space.num] = 0;

        space = self.next_space(space);

        let mut dumped = false;
        while marbles >= 1 {
            if space.num == 0 && self.player_to_move != space.player && !dumped {
                self.mancalas[space.player.to_u()] += 1;
                dumped = true;
            } else {
                self.spaces[space.player.to_u()][space.num] += 1;
                dumped = false;
                space = self.next_space(space);
            }
            marbles -= 1;
        }

        if self.spaces[space.player.to_u()][space.num] == 1 {
            self.mancalas[space.player.to_u()] +=
                self.spaces[space.player.next().to_u()][SPACES_PER_PLAYER - 1 - space.num];
            self.spaces[space.player.next().to_u()][SPACES_PER_PLAYER - 1 - space.num] = 0;
        } else {
            self.player_to_move = self.player_to_move.next();
        }
    }

    pub fn apply_turn_cp(&self, turn: BoardSpace) -> Self {
        let mut out: MancalaBoard = self.clone();
        out.apply_turn_ip(turn);
        out
    }
}

impl MancalaGameNode {
    pub fn apply_turn_cp(&self, turn: BoardSpace) -> Self {
        MancalaGameNode {
            turn: self.turn + 1,
            solved: false,
            terminal: false,
            utility: None,
            board: self.board.apply_turn_cp(turn),
            children: None,
        }
    }
    pub fn default(board: MancalaBoard) -> Self {
        MancalaGameNode {
            turn: 1,
            terminal: false,
            solved: false,
            utility: None,
            board,
            children: None,
        }
    }

    pub fn origin(board: MancalaBoard) -> Self {
        MancalaGameNode {
            turn: 1,
            terminal: false,
            solved: false,
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
        //self.board.render_simple();

        if self.terminal && self.children.is_none() {
        } else {
            match self.children.as_mut() {
                Some(children) => {
                    for child in children {
                        child.deref_mut().build_trees(limit).await;
                    }
                }
                None => {}
            };
        }
    }

    //pub async fn build_trees_async(&mut self, limit: usize) {
    //    self.make_babies(limit);
    //
    //    if self.terminal && self.children.is_none() {
    //    } else {
    //        match &mut self.children {
    //            Some(children) => {
    //                for mut child in children {
    //                    spawn_blocking(move || child.get_mut().unwrap().build_trees(limit)).await;
    //                }
    //            }
    //            None => {}
    //        };
    //    }
    //}

    #[async_recursion]
    pub async fn evaluate_self_worth_from_children(&mut self) {
        //println!(
        //    "term:{},util:{:?},children.is_none:{}",
        //    self.terminal,
        //    self.utility,
        //    self.children.is_none()
        //);
        if self.utility == None && self.children.is_none() {
            self.utility = Some(self.board.mancalas[1] as isize - self.board.mancalas[0] as isize);
            if self.terminal {
                //println!("{:?}", self);
                self.solved = true;
            }
            //match self.board.player_to_move {
            //    Player::PlayerOne => ,
            //    Player::PlayerTwo => ,
            //}
        } else {
            //Update children utility
            match self.children.as_mut() {
                Some(children) => {
                    for child in children {
                        child.deref_mut().evaluate_self_worth_from_children().await
                    }
                }
                None => {}
            };

            //Propogate child utility to self
            let utilities = self
                .children
                .as_mut()
                .unwrap()
                .iter_mut()
                .map(|child| child.utility.unwrap());

            self.utility = match self.board.player_to_move {
                Player::PlayerOne => utilities.max(),
                Player::PlayerTwo => utilities.min(),
            };

            let solvedness: usize = self
                .children
                .as_ref()
                .unwrap()
                .iter()
                .map(|child| if child.solved { 0 } else { 1 })
                .sum();

            if solvedness == 0 {
                self.solved = true;
            }
        }
    }
}
