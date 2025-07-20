#![allow(non_snake_case)]
#![allow(unused_parens)]

use crate::game::{Game, State};
use crate::{rand, DIR8};
use std::collections::HashSet;

pub static DEPTH: i16 = 4;
pub static WIN_SCORE: f64 = (1 << 12) as f64;
pub static INFINITY: f64 = 2.0 * WIN_SCORE;
static DELAY_FACTOR: f64 = 0.9;
static ACTIVATABLE_COUNT: usize = 2;

fn CalculateScore(
    newGame: &mut Game,
    x1: i16,
    y1: i16,
    /*x2: i16, y2: i16,*/ player: i8,
) -> f64 {
    let mut score = rand();
    if (player == 1) {
        for dir in DIR8 {
            if (matches!(newGame.getState(x1 - dir.0, y1 - dir.1), State::Active(_))
                && matches!(newGame.getState(x1 + dir.0, y1 + dir.1), State::Cross(_))
                && matches!(
                    newGame.getState(x1 + 2 * dir.0, y1 + 2 * dir.1),
                    State::Cross(_)
                )
                && matches!(
                    newGame.getState(x1 + 3 * dir.0, y1 + 3 * dir.1),
                    State::Active(_)
                ))
            {
                score = 1.5;
            }
        }
        for dir in &DIR8[0..4] {
            if (matches!(
                newGame.getState(x1 - 2 * dir.0, y1 - 2 * dir.1),
                State::Active(_)
            ) && matches!(newGame.getState(x1 - dir.0, y1 - dir.1), State::Cross(_))
                && matches!(
                    newGame.getState(x1 + 2 * dir.0, y1 + 2 * dir.1),
                    State::Cross(_)
                )
                && matches!(
                    newGame.getState(x1 + 3 * dir.0, y1 + 3 * dir.1),
                    State::Active(_)
                ))
            {
                score = 1.0;
            }
        }
    } else {
        for dir in DIR8 {
            if (matches!(newGame.getState(x1 - dir.0, y1 - dir.1), State::Active(_))
                && matches!(newGame.getState(x1 + dir.0, y1 + dir.1), State::Circle(_))
                && matches!(
                    newGame.getState(x1 + 2 * dir.0, y1 + 2 * dir.1),
                    State::Circle(_)
                )
                && matches!(
                    newGame.getState(x1 + 3 * dir.0, y1 + 3 * dir.1),
                    State::Active(_)
                ))
            {
                score = 1.5;
            }
        }
        for dir in &DIR8[0..4] {
            if (matches!(
                newGame.getState(x1 - 2 * dir.0, y1 - 2 * dir.1),
                State::Active(_)
            ) && matches!(newGame.getState(x1 - dir.0, y1 - dir.1), State::Circle(_))
                && matches!(
                    newGame.getState(x1 + 2 * dir.0, y1 + 2 * dir.1),
                    State::Circle(_)
                )
                && matches!(
                    newGame.getState(x1 + 3 * dir.0, y1 + 3 * dir.1),
                    State::Active(_)
                ))
            {
                score = 1.0
            }
        }
    }
    //TODO implement better heuristics
    score
}

pub fn NegaMax(
    mainGame: &mut Game,
    depth: i16,
    alpha: f64,
    beta: f64,
) -> (f64, i16, i16, i16, i16) {
    let mut currAlpha = alpha;
    let player = (mainGame.Move / 2) + 1;
    let mut bestMove = (-INFINITY, 0, 0, 0, 0);
    let mut activeOptions = Vec::new();
    let mut activatableOptions = Vec::new();
    for (loc1, state1) in mainGame.GameState.iter() {
        match state1 {
            State::Active(_) => activeOptions.push(*loc1),
            State::Activatable => activatableOptions.push(*loc1),
            _ => {}
        }
    }
    for loc1 in activeOptions.iter() {
        let mut activatableOptionsExtra = HashSet::new();
        for dir in DIR8 {
            if (matches!(
                mainGame.getState(loc1.0 + dir.0, loc1.1 + dir.1),
                State::Activatable
            )) {
                activatableOptionsExtra.insert((loc1.0 + dir.0, loc1.1 + dir.1));
            }
        }
        for _ in 0..ACTIVATABLE_COUNT {
            activatableOptionsExtra
                .insert(activatableOptions[(rand() * activatableOptions.len() as f64) as usize]);
        }
        mainGame.doPlayerClick(loc1.0, loc1.1, player);
        for loc2 in activatableOptionsExtra.iter() {
            mainGame.doPlayerClick(loc2.0, loc2.1, player);

            let score: f64;
            let win = mainGame.checkWin(loc1.0, loc1.1);
            if (win == 0) {
                if (depth == 1) {
                    score =
                        CalculateScore(mainGame, loc1.0, loc1.1, /*loc2.0, loc2.1,*/ player);
                } else {
                    score = -DELAY_FACTOR * NegaMax(mainGame, depth - 1, -beta, -alpha).0;
                }
            } else {
                score = WIN_SCORE;
            }

            mainGame.undoPlayerClick(loc2.0, loc2.1, player);

            if (score > bestMove.0) {
                bestMove = (score, loc1.0, loc1.1, loc2.0, loc2.1);
            }
            if (bestMove.0 > currAlpha) {
                currAlpha = bestMove.0;
            }
            if (currAlpha >= beta) {
                break;
            }
        }
        mainGame.undoPlayerClick(loc1.0, loc1.1, player);
        if (currAlpha >= beta) {
            break;
        }
    }
    bestMove
}
