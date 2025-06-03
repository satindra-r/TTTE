#![allow(non_snake_case)]
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
}

static BoxSize: i32 = 45;
static BoxBorder: i32 = 1;
static GridSize: i32 = 15;

static Move: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
static offsetX: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
static offsetY: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

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
    for (key, val) in gameState.lock().expect("Game state should exist").iter() {
        if (key.0 == x && key.1 == y) {
            return val.clone();
        }
    }
    State::Inactive
}

fn setState(x: i32, y: i32, s: State) {
    for (key, val) in gameState
        .lock()
        .expect("Game state should exist")
        .iter_mut()
    {
        if (key.0 == x && key.1 == y) {
            *val = s;
            break;
        }
    }
    gameState
        .lock()
        .expect("Game state should exist")
        .insert((x, y), s);
}

fn resetState() {
    let mut currGameState = gameState.lock().expect("Game state should exist");
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

fn render() {
    let width = getWindowWidth();
    let height = getWindowHeight();
    fillRect(0, 0, width, height, 32, 32, 48);
    for i in 0..GridSize {
        for j in 0..GridSize {
            let x = i - 7 + *offsetX.lock().expect("offset is initialised");
            let y = j - 7 + *offsetY.lock().expect("offset is initialised");
            match getState(x, y) {
                State::Inactive => {
                    fill3DRect(
                        BoxSize + i * BoxSize,
                        BoxSize + j * BoxSize,
                        BoxSize,
                        BoxSize,
                        96,
                        96,
                        96,
                        BoxBorder,
                        true,
                    );
                }
                State::Activatable => {
                    fill3DRect(
                        BoxSize + i * BoxSize,
                        BoxSize + j * BoxSize,
                        BoxSize,
                        BoxSize,
                        128,
                        192,
                        64,
                        BoxBorder,
                        false,
                    );
                }
                State::Active => {
                    fill3DRect(
                        BoxSize + i * BoxSize,
                        BoxSize + j * BoxSize,
                        BoxSize,
                        BoxSize,
                        0,
                        160,
                        224,
                        BoxBorder,
                        false,
                    );
                }
                State::Cross => {
                    fill3DRect(
                        BoxSize + i * BoxSize,
                        BoxSize + j * BoxSize,
                        BoxSize,
                        BoxSize,
                        0,
                        160,
                        224,
                        BoxBorder,
                        false,
                    );
                    drawCross(
                        BoxSize + i * BoxSize + BoxSize / 2 + 1,
                        BoxSize + j * BoxSize + BoxSize / 2 + 1,
                        (BoxSize - BoxBorder * 16) / 2,
                        255,
                        128,
                        32,
                        BoxBorder * 4,
                    );
                }
                State::Circle => {
                    fill3DRect(
                        BoxSize + i * BoxSize,
                        BoxSize + j * BoxSize,
                        BoxSize,
                        BoxSize,
                        0,
                        160,
                        224,
                        BoxBorder,
                        false,
                    );
                    drawCircle(
                        BoxSize + i * BoxSize + BoxSize / 2 + 1,
                        BoxSize + j * BoxSize + BoxSize / 2 + 1,
                        (BoxSize - BoxBorder * 16) / 2,
                        255,
                        128,
                        32,
                        BoxBorder * 4,
                    );
                }
            }
        }
    }
}

/*impl PartialEq<Self> for state {
    fn eq(&self, other: &Self) -> bool {
        match self {
            state::Inactive => matches!(other, state::Inactive),
            state::Activatable => matches!(other, state::Activatable),
            state::Active => matches!(other, state::Active),
            state::Cross => matches!(other, state::Cross),
            state::Circle => matches!(other, state::Circle),
        }
    }
}*/

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
    return 0;
}

#[wasm_bindgen]
pub fn reset() {
    resetState();
    render();
}

#[wasm_bindgen]
pub fn handleKeyDown(key: &str) {
    match key {
        "ArrowUp" | "w" => {
            *offsetY.lock().expect("Offset is set") -= 1;
        }
        "ArrowRight" | "d" => {
            *offsetX.lock().expect("Offset is set") += 1;
        }
        "ArrowDown" | "s" => {
            *offsetY.lock().expect("Offset is set") += 1;
        }
        "ArrowLeft" | "a" => {
            *offsetX.lock().expect("Offset is set") -= 1;
        }
        _ => {}
    }
    render();
}

#[wasm_bindgen]
pub fn handleMouseClick(mouseX: i32, mouseY: i32) -> i32 {
    let gridX = (mouseX - BoxSize) / BoxSize;
    let gridY = (mouseY - BoxSize) / BoxSize;
    if (gridX >= 0 && gridY >= 0 && gridX < GridSize && gridY < GridSize) {
        let x = gridX - 7 + *offsetX.lock().expect("offset is set");
        let y = gridY - 7 + *offsetY.lock().expect("offset is set");
        match getState(x, y) {
            State::Activatable => {
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
            }
            State::Active => {
                if (randRange(0, 2) == 1) {
                    setState(x, y, State::Cross);
                } else {
                    setState(x, y, State::Circle);
                }
                render();
                return checkWin(x, y);
            }
            _ => {}
        }
    }
    0
}
