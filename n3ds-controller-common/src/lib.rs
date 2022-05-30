//! Common types and functionality for the n3ds-controller project

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum InputMessage {
    Button {
        action: ButtonAction,
        button: Button,
    },
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum ButtonAction {
    Pressed,
    Released,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Button {
    A,
    B,
    X,
    Y,
    L,
    R,
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
}
