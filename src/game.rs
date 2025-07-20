#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::cmp::PartialEq;
use std::collections::HashMap;

pub static DIR4: [(i16, i16); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
pub static DIR8: [(i16, i16); 8] = [
	(-1, -1),
	(0, -1),
	(1, -1),
	(-1, 0),
	(1, 0),
	(-1, 1),
	(0, 1),
	(1, 1),
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
	Inactive,
	Activatable,
	Active(bool),
	Cross(bool),
	Circle(bool),
}

#[derive(Clone, Debug)]
pub struct Game {
	pub GameState: HashMap<(i16, i16), State>,
	pub Move: i8,
}

impl Game {
	pub fn new() -> Game {
		let newGame = Game {
			GameState: HashMap::new(),
			Move: -1,
		};
		
		newGame
	}
	
	pub fn resetState(&mut self) {
		self.Move=0;
		self.GameState.clear();
		
		self.setState(-2, -1, State::Activatable);
		self.setState(-2, 0, State::Activatable);
		self.setState(-2, 1, State::Activatable);

		self.setState(-1, -2, State::Activatable);
		self.setState(-1, -1, State::Active(true));
		self.setState(-1, 0, State::Active(true));
		self.setState(-1, 1, State::Active(true));
		self.setState(-1, 2, State::Activatable);

		self.setState(0, -2, State::Activatable);
		self.setState(0, -1, State::Active(true));
		self.setState(0, 0, State::Active(true));
		self.setState(0, 1, State::Active(true));
		self.setState(0, 2, State::Activatable);

		self.setState(1, -2, State::Activatable);
		self.setState(1, -1, State::Active(true));
		self.setState(1, 0, State::Active(true));
		self.setState(1, 1, State::Active(true));
		self.setState(1, 2, State::Activatable);

		self.setState(2, -1, State::Activatable);
		self.setState(2, 0, State::Activatable);
		self.setState(2, 1, State::Activatable);
	}
	/*fn coordTransform(x: i16, y: i16) -> (usize, usize) {
		let xVec;
		let yVec;
		if (x >= 0) {
			xVec = 2 * x;
		} else {
			xVec = (-x * 2) - 1;
		}
		if (y >= 0) {
			yVec = 2 * y;
		} else {
			yVec = (-y * 2) - 1;
		}
		(xVec as usize, yVec as usize)
	}
	pub fn revCoordTransform(xVec: usize, yVec: usize) -> (i16, i16) {
		let x;
		let y;
		if (xVec % 2 == 0) {
			x = xVec as i16 / 2;
		} else {
			x = 1 - (xVec as i16 / 2);
		}
		if (yVec % 2 == 0) {
			y = yVec as i16 / 2;
		} else {
			y = 1 - (yVec as i16 / 2);
		}
		(x, y)
	}*/
	pub fn getState(&self, x: i16, y: i16) -> State {
		/*let (xVec, yVec) = Self::coordTransform(x, y);
		if (self.GameState.len() <= xVec || self.GameState[xVec].len() <= yVec) {
			return State::Inactive;
		}
		self.GameState[xVec][yVec]*/

		*self.GameState.get(&(x, y)).unwrap_or(&State::Inactive)
	}
	fn setState(&mut self, x: i16, y: i16, s: State) {
		/*let (xVec, yVec) = Self::coordTransform(x, y);
		if (self.GameState.len() <= xVec) {
			self.GameState.resize(xVec + 1, Vec::new());
		}
		if (self.GameState[xVec].len() <= yVec) {
			self.GameState[xVec].resize(yVec + 1, State::Inactive);
		}
		self.GameState[xVec][yVec] = s;
		*/
		if (s != State::Inactive) {
			self.GameState.insert((x, y), s);
		} else {
			self.GameState.remove(&(x, y));
		}
	}
	pub fn checkWin(&self, x: i16, y: i16) -> i8 {
		let clickedState = self.getState(x, y);
		let win;
		match clickedState {
			State::Cross(_) => {
				win = 1;
			}
			State::Circle(_) => {
				win = 2;
			}
			_ => {
				return 0;
			}
		}

		for dir in DIR8.iter() {
			let mut won1 = true;
			let mut won2 = true;
			for offset in 0..4 {
				if (won1 && self.getState(x + offset * dir.0, y + offset * dir.1) != clickedState) {
					won1 = false;
				}
				if (won2
					&& self.getState(x + (offset - 1) * dir.0, y + (offset - 1) * dir.1)
					!= clickedState)
				{
					won2 = false;
				}
			}
			if (won1 || won2) {
				return win;
			}
		}
		0
	}
	pub fn doPlayerClick(&mut self, x: i16, y: i16, player: i8) -> bool {
		let currMove = self.Move;
		match self.getState(x, y) {
			State::Activatable => {
				if (currMove == 1 && player == 1 || currMove == 3 && player == 2) {
					self.Move += 1;
					self.Move %= 4;
					self.setState(x, y, State::Active(true));

					for dir in DIR4.iter() {
						match self.getState(x + dir.0, y + dir.1) {
							State::Inactive => {
								self.setState(x + dir.0, y + dir.1, State::Activatable);
							}
							State::Activatable => {
								let mut neighbourActivate = true;
								for neighbourDir in DIR4.iter() {
									match self.getState(
										x + dir.0 + neighbourDir.0,
										y + dir.1 + neighbourDir.1,
									) {
										State::Inactive | State::Activatable => {
											neighbourActivate = false;
											break;
										}
										_ => {}
									}
								}
								if (neighbourActivate) {
									self.setState(x + dir.0, y + dir.1, State::Active(false));
								}
							}
							_ => {}
						}
					}
					return true;
				}
			}
			State::Active(clicked) => {
				if (currMove == 0 && player == 1 || currMove == 2 && player == 2) {
					self.Move += 1;
					self.Move %= 4;
					if (player == 1) {
						self.setState(x, y, State::Cross(clicked));
					} else {
						self.setState(x, y, State::Circle(clicked));
					}
					return true;
				}
			}
			_ => {}
		}
		false
	}
	pub fn undoPlayerClick(&mut self, x: i16, y: i16, player: i8) -> bool {
		let prevMove = (self.Move + 3) % 4;
		match (self.getState(x, y)) {
			State::Active(true) => {
				if (prevMove == 1 && player == 1 || prevMove == 3 && player == 2) {
					self.Move = prevMove;
					self.setState(x, y, State::Activatable);

					for dir in DIR4.iter() {
						match (self.getState(x + dir.0, y + dir.1)) {
							State::Active(false) => {
								self.setState(x + dir.0, y + dir.1, State::Activatable);
							}
							State::Activatable => {
								let mut neighbourInactivate = true;
								for neighbourDir in DIR4.iter() {
									match (self.getState(
										x + dir.0 + neighbourDir.0,
										y + dir.1 + neighbourDir.1,
									)) {
										State::Active(_) | State::Cross(_) | State::Circle(_) => {
											neighbourInactivate = false;
											break;
										}
										_ => {}
									}
								}
								if (neighbourInactivate) {
									self.setState(x + dir.0, y + dir.1, State::Inactive);
								}
							}
							_ => {}
						}
					}
					return true;
				}
			}
			State::Cross(clicked) => {
				if (prevMove == 0 && player == 1) {
					self.Move = prevMove;
					self.setState(x, y, State::Active(clicked));
					return true;
				}
			}
			State::Circle(clicked) => {
				if (prevMove == 2 && player == 2) {
					self.Move = prevMove;
					self.setState(x, y, State::Active(clicked));
					return true;
				}
			}
			_ => {}
		}
		false
	}
}