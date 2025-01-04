wit_bindgen::generate!({
    world: "task",
    path: "../wit",
});

struct MyTask;

impl Guest for MyTask {
    fn run() {
        log("there we are!");
    }
}

export!(MyTask);
