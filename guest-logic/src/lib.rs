wit_bindgen::generate!({
    world: "task",
    path: "../wit",
});

use crate::example::task::logging::{log, Level};

struct MyTask;

impl Guest for MyTask {
    fn run() {
        log(Level::Debug, "there we are!");
    }
}

export!(MyTask);
