#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
mod utils;

use std::cmp::PartialEq;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::__rt::LazyLock;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/helper.js")]
extern "C" {
    fn print(str: &str);
    fn getWindowWidth() -> i32;
    fn getWindowHeight() -> i32;
    fn randRange(x: i32, y: i32) -> i32;
    fn drawRect(x: i32, y: i32, w: i32, h: i32, r: u8, g: u8, b: u8, t: i32);
    fn fillRect(x: i32, y: i32, w: i32, h: i32, r: u8, g: u8, b: u8);
    fn fill3DRect(x: i32, y: i32, w: i32, h: i32, r: u8, g: u8, b: u8, t: i32, raised: bool);
    fn drawCross(x: i32, y: i32, s: i32, r: u8, g: u8, b: u8, t: i32);
    fn drawCircle(x: i32, y: i32, s: i32, r: u8, g: u8, b: u8, t: i32);
    fn getConnectionRequest();
    fn getConnectionResponse();
    fn setRemoteDesc();
    fn sendData(str: &str);
    fn setStatus(str: &str);
}

static BOX_SIZE: i32 = 45;
static BOX_BORDER: i32 = 1;
static GRID_SIZE: i32 = 15;

static Move: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

static OppGameStart: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-1));
static PlayerGameStart: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-1));
static OffsetX: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
static OffsetY: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
static Player: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-1));

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    Inactive,
    Activatable,
    Active,
    Cross,
    Circle,
}

static gameState: LazyLock<Mutex<HashMap<(i32, i32), State>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn getState(x: i32, y: i32) -> State {
    for (key, val) in gameState.lock().unwrap().iter() {
        if (key.0 == x && key.1 == y) {
            return val.clone();
        }
    }
    State::Inactive
}

fn setState(x: i32, y: i32, s: State) {
    for (key, val) in gameState.lock().unwrap().iter_mut() {
        if (key.0 == x && key.1 == y) {
            *val = s;
            break;
        }
    }
    gameState.lock().unwrap().insert((x, y), s);
}

fn resetState() {
    *Move.lock().unwrap() = 0;
    *PlayerGameStart.lock().unwrap() = -1;
    *OppGameStart.lock().unwrap() = -1;
    *OffsetX.lock().unwrap() = 0;
    *OffsetY.lock().unwrap() = 0;

    if (*Player.lock().unwrap() == 1) {
        setStatus("New Game Started, Your turn to Place");
    } else {
        setStatus("New Game Started, Opponent's turn to Place");
    }

    let mut currGameState = gameState.lock().unwrap();

    currGameState.clear();

    currGameState.insert((-2, -1), State::Activatable);
    currGameState.insert((-2, 0), State::Activatable);
    currGameState.insert((-2, 1), State::Activatable);

    currGameState.insert((-1, -2), State::Activatable);
    currGameState.insert((-1, -1), State::Active);
    currGameState.insert((-1, 0), State::Active);
    currGameState.insert((-1, 1), State::Active);
    currGameState.insert((-1, 2), State::Activatable);

    currGameState.insert((0, -2), State::Activatable);
    currGameState.insert((0, -1), State::Active);
    currGameState.insert((0, 0), State::Active);
    currGameState.insert((0, 1), State::Active);
    currGameState.insert((0, 2), State::Activatable);

    currGameState.insert((1, -2), State::Activatable);
    currGameState.insert((1, -1), State::Active);
    currGameState.insert((1, 0), State::Active);
    currGameState.insert((1, 1), State::Active);
    currGameState.insert((1, 2), State::Activatable);

    currGameState.insert((2, -1), State::Activatable);
    currGameState.insert((2, 0), State::Activatable);
    currGameState.insert((2, 1), State::Activatable);
}

#[wasm_bindgen]
pub fn render() {
    let width = getWindowWidth();
    let height = getWindowHeight();
    fillRect(0, 0, width, height, 32, 32, 48);
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = i - 7 + *OffsetX.lock().unwrap();
            let y = j - 7 + *OffsetY.lock().unwrap();
            match getState(x, y) {
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
                State::Active => {
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
                State::Cross => {
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
                State::Circle => {
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

fn checkWin(x: i32, y: i32) -> i32 {
    let clickedState = getState(x, y);
    let win;
    if (clickedState == State::Cross) {
        win = 1;
    } else if (clickedState == State::Circle) {
        win = -1;
    } else {
        return 0;
    }
    if (getState(x - 3, y) == clickedState
        && getState(x - 2, y) == clickedState
        && getState(x - 1, y) == clickedState)
    {
        return win;
    } else if (getState(x - 2, y) == clickedState
        && getState(x - 1, y) == clickedState
        && getState(x + 1, y) == clickedState)
    {
        return win;
    } else if (getState(x - 1, y) == clickedState
        && getState(x + 1, y) == clickedState
        && getState(x + 2, y) == clickedState)
    {
        return win;
    } else if (getState(x + 1, y) == clickedState
        && getState(x + 2, y) == clickedState
        && getState(x + 3, y) == clickedState)
    {
        return win;
    } else if (getState(x, y - 3) == clickedState
        && getState(x, y - 2) == clickedState
        && getState(x, y - 1) == clickedState)
    {
        return win;
    } else if (getState(x, y - 2) == clickedState
        && getState(x, y - 1) == clickedState
        && getState(x, y + 1) == clickedState)
    {
        return win;
    } else if (getState(x, y - 1) == clickedState
        && getState(x, y + 1) == clickedState
        && getState(x, y + 2) == clickedState)
    {
        return win;
    } else if (getState(x, y + 1) == clickedState
        && getState(x, y + 2) == clickedState
        && getState(x, y + 3) == clickedState)
    {
        return win;
    } else if (getState(x - 3, y - 3) == clickedState
        && getState(x - 2, y - 2) == clickedState
        && getState(x - 1, y - 1) == clickedState)
    {
        return win;
    } else if (getState(x - 2, y - 2) == clickedState
        && getState(x - 1, y - 1) == clickedState
        && getState(x + 1, y + 1) == clickedState)
    {
        return win;
    } else if (getState(x - 1, y - 1) == clickedState
        && getState(x + 1, y + 1) == clickedState
        && getState(x + 2, y + 2) == clickedState)
    {
        return win;
    } else if (getState(x + 1, y + 1) == clickedState
        && getState(x + 2, y + 2) == clickedState
        && getState(x + 3, y + 3) == clickedState)
    {
        return win;
    } else if (getState(x - 3, y + 3) == clickedState
        && getState(x - 2, y + 2) == clickedState
        && getState(x - 1, y + 1) == clickedState)
    {
        return win;
    } else if (getState(x - 2, y + 2) == clickedState
        && getState(x - 1, y + 1) == clickedState
        && getState(x + 1, y - 1) == clickedState)
    {
        return win;
    } else if (getState(x - 1, y + 1) == clickedState
        && getState(x + 1, y - 1) == clickedState
        && getState(x + 2, y - 2) == clickedState)
    {
        return win;
    } else if (getState(x + 1, y - 1) == clickedState
        && getState(x + 2, y - 2) == clickedState
        && getState(x + 3, y - 3) == clickedState)
    {
        return win;
    }
    0
}

fn doPlayerClick(x: i32, y: i32, player: i32) -> bool {
    let currMove = *Move.lock().unwrap();
    match getState(x, y) {
        State::Activatable => {
            if (currMove == 1 && player == 1 || currMove == 3 && player == 2) {
                *Move.lock().unwrap() += 1;
                *Move.lock().unwrap() %= 4;
                if (player == *Player.lock().unwrap()) {
                    setStatus("Opponent's turn to Place");
                } else {
                    setStatus("Your turn to Place");
                }
                setState(x, y, State::Active);
                if (getState(x - 1, y) == State::Inactive) {
                    setState(x - 1, y, State::Activatable);
                } else if (getState(x - 1, y) == State::Activatable) {
                    if (getState(x - 2, y) != State::Inactive
                        && getState(x - 2, y) != State::Activatable
                        && getState(x - 1, y - 1) != State::Inactive
                        && getState(x - 1, y - 1) != State::Activatable
                        && getState(x - 1, y + 1) != State::Inactive
                        && getState(x - 1, y + 1) != State::Activatable)
                    {
                        setState(x - 1, y, State::Active);
                    }
                }
                if (getState(x + 1, y) == State::Inactive) {
                    setState(x + 1, y, State::Activatable);
                } else if (getState(x + 1, y) == State::Activatable) {
                    if (getState(x + 2, y) != State::Inactive
                        && getState(x + 2, y) != State::Activatable
                        && getState(x + 1, y - 1) != State::Inactive
                        && getState(x + 1, y - 1) != State::Activatable
                        && getState(x + 1, y + 1) != State::Inactive
                        && getState(x + 1, y + 1) != State::Activatable)
                    {
                        setState(x + 1, y, State::Active);
                    } else {
                        setState(x + 1, y, State::Activatable);
                    }
                }
                if (getState(x, y - 1) == State::Inactive) {
                    setState(x, y - 1, State::Activatable);
                } else if (getState(x, y - 1) == State::Activatable) {
                    if (getState(x, y - 2) != State::Inactive
                        && getState(x, y - 2) != State::Activatable
                        && getState(x - 1, y - 1) != State::Inactive
                        && getState(x - 1, y - 1) != State::Activatable
                        && getState(x + 1, y - 1) != State::Inactive
                        && getState(x + 1, y - 1) != State::Activatable)
                    {
                        setState(x, y - 1, State::Active);
                    } else {
                        setState(x, y - 1, State::Activatable);
                    }
                }
                if (getState(x, y + 1) == State::Inactive) {
                    setState(x, y + 1, State::Activatable);
                } else if (getState(x, y + 1) == State::Activatable) {
                    if (getState(x, y + 2) != State::Inactive
                        && getState(x, y + 2) != State::Activatable
                        && getState(x - 1, y + 1) != State::Inactive
                        && getState(x - 1, y + 1) != State::Activatable
                        && getState(x + 1, y + 1) != State::Inactive
                        && getState(x + 1, y + 1) != State::Activatable)
                    {
                        setState(x, y + 1, State::Active);
                    } else {
                        setState(x, y + 1, State::Activatable);
                    }
                }
                render();
                return true;
            }
        }
        State::Active => {
            if (currMove == 0 && player == 1 || currMove == 2 && player == 2) {
                *Move.lock().unwrap() += 1;
                *Move.lock().unwrap() %= 4;
                if (player == *Player.lock().unwrap()) {
                    setStatus("Your turn to Expand");
                } else {
                    setStatus("Opponent's turn to Expand");
                }
                if (player == 1) {
                    setState(x, y, State::Cross);
                } else {
                    setState(x, y, State::Circle);
                }
                render();
                return true;
            }
        }
        _ => {}
    }
    false
}


fn reset() {
    resetState();
    render();
}

#[wasm_bindgen]
pub fn handleKeyDown(key: &str) {
    match key {
        "ArrowUp" => {
            *OffsetY.lock().unwrap() -= 1;
        }
        "ArrowRight" => {
            *OffsetX.lock().unwrap() += 1;
        }
        "ArrowDown" => {
            *OffsetY.lock().unwrap() += 1;
        }
        "ArrowLeft" => {
            *OffsetX.lock().unwrap() -= 1;
        }
        " " => {
            *OffsetX.lock().unwrap() = 0;
            *OffsetY.lock().unwrap() = 0;
        }
        "Enter" => {
            if (*Move.lock().unwrap() == -1) {
                if (*OppGameStart.lock().unwrap() == 0) {
                    *PlayerGameStart.lock().unwrap() = 1;
                    setStatus("Waiting for Opponent to Start New Game");
                    sendData("Start:");
                } else if (*OppGameStart.lock().unwrap() == 1) {
                    *Move.lock().unwrap() = 0;
                    sendData("Start:");
                    reset();
                }
            }
        }
        _ => {}
    }
    render();
}

#[wasm_bindgen]
pub fn handleMouseClick(mouseX: i32, mouseY: i32) {
    let gridX = (mouseX - BOX_SIZE) / BOX_SIZE;
    let gridY = (mouseY - BOX_SIZE) / BOX_SIZE;
    if (gridX >= 0 && gridY >= 0 && gridX < GRID_SIZE && gridY < GRID_SIZE) {
        let x = gridX - 7 + *OffsetX.lock().unwrap();
        let y = gridY - 7 + *OffsetY.lock().unwrap();
        let currPlayer = *Player.lock().unwrap();

        let validClick = doPlayerClick(x, y, currPlayer);
        if (validClick) {
            sendData(format!("Move:{},{}", x, y).as_str());
            let win = checkWin(x, y);
            if (win == currPlayer) {
                sendData(format!("Win:{},{}", x, y).as_str());
                setStatus("Your Won, Press Enter to Start a New Game");
                *Player.lock().unwrap() = 3 - currPlayer;
                *Move.lock().unwrap() = -1;
                *OppGameStart.lock().unwrap() = 0;
                *PlayerGameStart.lock().unwrap() = 0;
            }
        }
    }
}

#[wasm_bindgen]
pub fn handleDataIn(data: &str) {
    if (data.starts_with("Join:")) {
        if (data.replace("Join:", "").parse::<i32>().is_err()) {
            return;
        }

        let oppPlayer = data.replace("Join:", "").parse::<i32>().unwrap();
        *Player.lock().unwrap() = 3 - oppPlayer;
        reset();
    } else if (data.starts_with("Move:")) {
        let currPlayer = *Player.lock().unwrap();
        let tuple = data.replace("Move:", "");
        if (tuple.split(",").count() != 2
            || tuple.split(",").next().unwrap().parse::<i32>().is_err()
            || tuple.split(",").last().unwrap().parse::<i32>().is_err())
        {
            return;
        }
        let x = tuple.split(",").next().unwrap().parse::<i32>().unwrap();
        let y = tuple.split(",").last().unwrap().parse::<i32>().unwrap();
        doPlayerClick(x, y, 3 - currPlayer);
    } else if (data.starts_with("Win:")) {
        let tuple = data.replace("Win:", "");
        if (tuple.split(",").count() != 2
            || tuple.split(",").next().unwrap().parse::<i32>().is_err()
            || tuple.split(",").last().unwrap().parse::<i32>().is_err())
        {
            return;
        }

        let x = tuple.split(",").next().unwrap().parse::<i32>().unwrap();
        let y = tuple.split(",").last().unwrap().parse::<i32>().unwrap();
        let currPlayer = *Player.lock().unwrap();
        let win = checkWin(x, y);
        if (win == 3 - currPlayer) {
            setStatus("You Lost, Press Enter to Start a New Game");
            *Player.lock().unwrap() = 3 - currPlayer;
            *Move.lock().unwrap() = -1;
            *OppGameStart.lock().unwrap() = 0;
            *PlayerGameStart.lock().unwrap() = 0;
        }
    } else if (data.starts_with("Start:")) {
        if (*Move.lock().unwrap() == -1) {
            if (*PlayerGameStart.lock().unwrap() == 0) {
                *OppGameStart.lock().unwrap() = 1;
                setStatus("Opponent is waiting for you to Start New Game, Press Enter to Start a New Game");
            } else if (*PlayerGameStart.lock().unwrap() == 1) {
                *Move.lock().unwrap() = 0;
                print("reset?");
                reset();
            }
        }
    }
}

#[wasm_bindgen]
pub fn createRequest() {
    getConnectionRequest();
}
#[wasm_bindgen]
pub fn createResponse() {
    getConnectionResponse();
}
#[wasm_bindgen]
pub fn beginConnection() {
    setRemoteDesc();
}
