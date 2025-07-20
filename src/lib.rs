#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
extern crate console_error_panic_hook;
mod agent;
mod game;

use crate::game::{Game, State};
use std::panic;
use std::sync::Mutex;
use wasm_bindgen::__rt::LazyLock;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/helper.js")]
extern "C" {
    fn print(str: &str);
    fn getWindowWidth() -> i16;
    fn getWindowHeight() -> i16;
	fn rand() -> f64;
    fn drawRect(x: i16, y: i16, w: i16, h: i16, r: u8, g: u8, b: u8, t: i16);
    fn fillRect(x: i16, y: i16, w: i16, h: i16, r: u8, g: u8, b: u8);
    fn fill3DRect(x: i16, y: i16, w: i16, h: i16, r: u8, g: u8, b: u8, t: i16, raised: bool);
    fn drawCross(x: i16, y: i16, s: i16, r: u8, g: u8, b: u8, t: i16);
    fn drawCircle(x: i16, y: i16, s: i16, r: u8, g: u8, b: u8, t: i16);
    fn getConnectionRequest();
    fn getConnectionResponse();
    fn setRemoteDesc();
    fn sendData(str: &str);
    fn setStatus(str: &str);
    fn callAI();
}

static BOX_SIZE: i16 = 45;
static BOX_BORDER: i16 = 1;
static GRID_SIZE: i16 = 15;

static DIR4: [(i16, i16); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
static DIR8: [(i16, i16); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

static OppGameStart: LazyLock<Mutex<i8>> = LazyLock::new(|| Mutex::new(-1));
static PlayerGameStart: LazyLock<Mutex<i8>> = LazyLock::new(|| Mutex::new(-1));
static OffsetX: LazyLock<Mutex<i16>> = LazyLock::new(|| Mutex::new(0));
static OffsetY: LazyLock<Mutex<i16>> = LazyLock::new(|| Mutex::new(0));
static Player: LazyLock<Mutex<i8>> = LazyLock::new(|| Mutex::new(-1));

static AI: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

#[wasm_bindgen]
pub fn setHook() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

static MAIN_GAME: LazyLock<Mutex<Game>> = LazyLock::new(|| Mutex::new(Game::new()));

fn resetState() {
    *PlayerGameStart.lock().unwrap() = -1;
    *OppGameStart.lock().unwrap() = -1;
    *OffsetX.lock().unwrap() = 0;
    *OffsetY.lock().unwrap() = 0;

    if (*Player.lock().unwrap() == 1) {
        setStatus("New Game Started, Your turn to Place");
    } else {
		if(*AI.lock().unwrap()){
			setStatus("New Game Started, AI's turn to Place");
		}else{
			setStatus("New Game Started, Opponent's turn to Place");
		}
    }

    MAIN_GAME.lock().unwrap().resetState();
}

#[wasm_bindgen]
pub fn render() {
    let width = getWindowWidth();
    let height = getWindowHeight();
    fillRect(0, 0, width, height, 32, 32, 48);
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = i - 7 + *OffsetX.lock().unwrap();
            let y = 7 - j + *OffsetY.lock().unwrap();
            match MAIN_GAME.lock().unwrap().getState(x, y) {
                State::Inactive => {
                    fill3DRect(
                        BOX_SIZE + i * BOX_SIZE,
                        BOX_SIZE + j * BOX_SIZE,
                        BOX_SIZE,
                        BOX_SIZE,
                        96,
                        96,
                        96,
                        BOX_BORDER,
                        true,
                    );
                }
                State::Activatable => {
                    fill3DRect(
                        BOX_SIZE + i * BOX_SIZE,
                        BOX_SIZE + j * BOX_SIZE,
                        BOX_SIZE,
                        BOX_SIZE,
                        128,
                        192,
                        64,
                        BOX_BORDER,
                        false,
                    );
                }
                State::Active(_) => {
                    fill3DRect(
                        BOX_SIZE + i * BOX_SIZE,
                        BOX_SIZE + j * BOX_SIZE,
                        BOX_SIZE,
                        BOX_SIZE,
                        0,
                        160,
                        224,
                        BOX_BORDER,
                        false,
                    );
                }
                State::Cross(_) => {
                    fill3DRect(
                        BOX_SIZE + i * BOX_SIZE,
                        BOX_SIZE + j * BOX_SIZE,
                        BOX_SIZE,
                        BOX_SIZE,
                        0,
                        160,
                        224,
                        BOX_BORDER,
                        false,
                    );
                    drawCross(
                        BOX_SIZE + i * BOX_SIZE + BOX_SIZE / 2 + 1,
                        BOX_SIZE + j * BOX_SIZE + BOX_SIZE / 2 + 1,
                        (BOX_SIZE - BOX_BORDER * 16) / 2,
                        255,
                        128,
                        32,
                        BOX_BORDER * 4,
                    );
                }
                State::Circle(_) => {
                    fill3DRect(
                        BOX_SIZE + i * BOX_SIZE,
                        BOX_SIZE + j * BOX_SIZE,
                        BOX_SIZE,
                        BOX_SIZE,
                        0,
                        160,
                        224,
                        BOX_BORDER,
                        false,
                    );
                    drawCircle(
                        BOX_SIZE + i * BOX_SIZE + BOX_SIZE / 2 + 1,
                        BOX_SIZE + j * BOX_SIZE + BOX_SIZE / 2 + 1,
                        (BOX_SIZE - BOX_BORDER * 16) / 2,
                        255,
                        128,
                        32,
                        BOX_BORDER * 4,
                    );
                }
            }
        }
    }
}

fn reset() {
    resetState();
    render();
}

#[wasm_bindgen]
pub fn handleKeyDown(key: &str) {
    match key {
        "ArrowUp" => {
            *OffsetY.lock().unwrap() += 1;
        }
        "ArrowRight" => {
            *OffsetX.lock().unwrap() += 1;
        }
        "ArrowDown" => {
            *OffsetY.lock().unwrap() -= 1;
        }
        "ArrowLeft" => {
            *OffsetX.lock().unwrap() -= 1;
        }
        " " => {
            *OffsetX.lock().unwrap() = 0;
            *OffsetY.lock().unwrap() = 0;
        }
        "Enter" => {
            if (MAIN_GAME.lock().unwrap().Move == -1) {
                if (*OppGameStart.lock().unwrap() == 0) {
                    *PlayerGameStart.lock().unwrap() = 1;
                    setStatus("Waiting for Opponent to Start New Game");
                    sendData("Start:");
                } else if (*OppGameStart.lock().unwrap() == 1) {
                    MAIN_GAME.lock().unwrap().Move = 0;
                    if (*AI.lock().unwrap()) {
                        callAI();
                    } else {
                        sendData("Start:");
                    }
                    reset();
                }
            }
        }
        _ => {}
    }
    render();
}

#[wasm_bindgen]
pub fn handleAIMove() {
	print("Its AI time");
	let currPlayer = *Player.lock().unwrap();
	if (*AI.lock().unwrap() && (MAIN_GAME.lock().unwrap().Move / 2) + 1 == 3 - currPlayer) {
		let bestMove = agent::NegaMax(
            &mut *MAIN_GAME.lock().unwrap(),
            agent::DEPTH,
            -agent::INFINITY,
            agent::INFINITY,
        );
        MAIN_GAME
            .lock()
            .unwrap()
            .doPlayerClick(bestMove.1, bestMove.2, 3 - currPlayer);
        print(format!("AIScore:{}", bestMove.0).as_str());
        print(format!("AIMove:{},{}", bestMove.1, bestMove.2).as_str());
        let win = MAIN_GAME.lock().unwrap().checkWin(bestMove.1, bestMove.2);
        if (win == 3 - currPlayer) {
            setStatus("Your Lost, Press Enter to Start a New Game");
            *Player.lock().unwrap() = 3 - currPlayer;
            MAIN_GAME.lock().unwrap().Move = -1;
            *OppGameStart.lock().unwrap() = 1;
            *PlayerGameStart.lock().unwrap() = 0;
        } else {
            MAIN_GAME
                .lock()
                .unwrap()
                .doPlayerClick(bestMove.3, bestMove.4, 3 - currPlayer);
            print(format!("AIMove:{},{}", bestMove.3, bestMove.4).as_str());
            setStatus("Your turn to place");
        }
        render();
    }
}

#[wasm_bindgen]
pub fn handleMouseClick(mouseX: i16, mouseY: i16) {
    let gridX = (mouseX - BOX_SIZE) / BOX_SIZE;
    let gridY = (mouseY - BOX_SIZE) / BOX_SIZE;
    if (gridX >= 0 && gridY >= 0 && gridX < GRID_SIZE && gridY < GRID_SIZE) {
        let x = gridX - 7 + *OffsetX.lock().unwrap();
        let y = 7 - gridY + *OffsetY.lock().unwrap();
        let currPlayer = *Player.lock().unwrap();

        let validClick = MAIN_GAME.lock().unwrap().doPlayerClick(x, y, currPlayer);
        print(format!("Move:{},{}", x, y).as_str());

        if (validClick) {
            let currMove = MAIN_GAME.lock().unwrap().Move;
            match currMove {
                1 | 3 => {
                    setStatus("Your turn to expand");
                }
                0 | 2 => {
					if(*AI.lock().unwrap()) {
						setStatus("AI's turn to place");
					}else{
						setStatus("Opponent's turn to place");
					}
				}
                _ => {}
            }

            render();
            if (!*AI.lock().unwrap()) {
                sendData(format!("Move:{},{}", x, y).as_str());
            }
            let win = MAIN_GAME.lock().unwrap().checkWin(x, y);
            if (win == currPlayer) {
                if (!*AI.lock().unwrap()) {
                    sendData(format!("Win:{},{}", x, y).as_str());
                }
                setStatus("Your Won, Press Enter to Start a New Game");
                *Player.lock().unwrap() = 3 - currPlayer;

                MAIN_GAME.lock().unwrap().Move = -1;
                if (*AI.lock().unwrap()) {
                    *OppGameStart.lock().unwrap() = 1;
                } else {
                    *OppGameStart.lock().unwrap() = 0;
                }
                *PlayerGameStart.lock().unwrap() = 0;
            }
			if (*AI.lock().unwrap() && (MAIN_GAME.lock().unwrap().Move / 2) + 1 == 3 - currPlayer) {
                callAI();
            }
        }
    }
}

#[wasm_bindgen]
pub fn handleDataIn(data: &str) {
    if (data.starts_with("Join:")) {
        if (data.replace("Join:", "").parse::<i8>().is_err()) {
            return;
        }

        let oppPlayer = data.replace("Join:", "").parse::<i8>().unwrap();
        *Player.lock().unwrap() = 3 - oppPlayer;
        reset();
    } else if (data.starts_with("Move:")) {
        let currPlayer = *Player.lock().unwrap();
        let tuple = data.replace("Move:", "");
        if (tuple.split(",").count() != 2 || tuple.split(",").next().unwrap().parse::<i16>().is_err() || tuple.split(",").last().unwrap().parse::<i16>().is_err()) {
            return;
        }
        let x = tuple.split(",").next().unwrap().parse::<i16>().unwrap();
        let y = tuple.split(",").last().unwrap().parse::<i16>().unwrap();
        let validClick = MAIN_GAME
            .lock()
            .unwrap()
            .doPlayerClick(x, y, 3 - currPlayer);
        if (validClick) {
            let currMove = MAIN_GAME.lock().unwrap().Move;
			match currMove {
                1 | 3 => {
                    setStatus("Opponent's turn to expand");
                }
                0 | 2 => {
                    setStatus("Your turn to place");
                }
                _ => {}
            }
			render();
		}
	} else if (data.starts_with("Win:")) {
		let tuple = data.replace("Win:", "");
		if (tuple.split(",").count() != 2 || tuple.split(",").next().unwrap().parse::<i16>().is_err() || tuple.split(",").last().unwrap().parse::<i16>().is_err()) {
			return;
		}

        let x = tuple.split(",").next().unwrap().parse::<i16>().unwrap();
        let y = tuple.split(",").last().unwrap().parse::<i16>().unwrap();
        let currPlayer = *Player.lock().unwrap();
        let win = MAIN_GAME.lock().unwrap().checkWin(x, y);
        if (win == 3 - currPlayer) {
            setStatus("You Lost, Press Enter to Start a New Game");
            *Player.lock().unwrap() = 3 - currPlayer;
            MAIN_GAME.lock().unwrap().Move = -1;
            *OppGameStart.lock().unwrap() = 0;
            *PlayerGameStart.lock().unwrap() = 0;
        }
    } else if (data.starts_with("Start:")) {
        if (MAIN_GAME.lock().unwrap().Move == -1) {
            if (*PlayerGameStart.lock().unwrap() == 0) {
                *OppGameStart.lock().unwrap() = 1;
                setStatus("Opponent is waiting for you to Start New Game, Press Enter to Start a New Game");
            } else if (*PlayerGameStart.lock().unwrap() == 1) {
                MAIN_GAME.lock().unwrap().Move = 0;
                print("reset?");
                reset();
            }
        }
    }
}

#[wasm_bindgen]
pub fn createRequest() {
    if (!*AI.lock().unwrap()) {
        getConnectionRequest();
    }
}
#[wasm_bindgen]
pub fn createResponse() {
    if (!*AI.lock().unwrap()) {
        getConnectionResponse();
    }
}
#[wasm_bindgen]
pub fn beginConnection() {
    if (!*AI.lock().unwrap()) {
        setRemoteDesc();
    }
}

#[wasm_bindgen]
pub fn enableAI() {
    *AI.lock().unwrap() = true;
    *Player.lock().unwrap() = 1;
    reset();
}
