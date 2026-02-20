use std::fmt::Display;

pub fn header(title: impl Display) {
    cliclack::intro(title).ok();
}

pub fn outro(msg: impl Display) {
    cliclack::outro(msg).ok();
}

pub fn outro_cancel(msg: impl Display) {
    cliclack::outro_cancel(msg).ok();
}
