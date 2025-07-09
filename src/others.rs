use crate::game_color::GameColor;
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
    seq::IndexedRandom,
};

pub type PieceMatrix = [[Presence; 4]; 4];
pub type GameMap = [Vec<Presence>];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Presence {
    No,
    Yes(GameColor),
}

#[derive(Debug, Clone)]
pub enum PieceType {
    J,
    L,
    S,
    Z,
    T,
    I,
    O,
}

impl Distribution<PieceType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        use self::PieceType::*;
        [L, J, S, Z, T, I, O].choose(rng).unwrap().clone()
    }
}
