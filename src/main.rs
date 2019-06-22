extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session, Value};

use std::fs::File;
use std::io::prelude::*;

struct Log {
    fd: File,
}

impl Log {
    pub fn new() -> Log {
        Log { fd: File::create("/tmp/delinhere.log").unwrap() }
    }

    pub fn log(&mut self, string: &str) {
        self.fd.write(string.as_bytes()).unwrap();
    }

}

trait Logger {
    fn log(&mut self, string: &str) { }
}

struct App { }

impl App {
    pub fn new() -> Self {
        Self {  }
    }

    fn add(&self, values: Vec<Value>, logger: &mut Log ) -> String {
        logger.log("Inside add method\n");
        logger.log(&format!("values {:?}\n", values));
        let val = values.iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect::<String>();
        logger.log(&format!("Outputting val {}\n", val));
        return val
    }
}

enum Messages {
    Add,
    Multiply,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "add" => Messages::Add,
            "multiply" => Messages::Multiply,
            _ => Messages::Unknown(event),
        }
    }
}

struct EventHandler {
    nvim: Neovim,
    app: App,
    logger: Log
}

impl EventHandler {
    pub fn new() -> EventHandler {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let app = App::new();
        let logger = Log::new();

        EventHandler { nvim, app, logger }
    }

    fn process(&mut self, event: String, values: Vec<Value>) {
        self.log(&format!("Processing message from event type {}\n", event));
        let message = Messages::from(event);
        match message {
            Messages::Add => {
                self.log("Inside Add branch\n");
                let response = self.app.add(values, &mut self.logger);
                self.log(&format!("response {:?}\n", response));
                let command = self.nvim
                    .command(&format!("echo \"Sum: {}\"", response));
                self.log(&format!("command {:?}\n", command));
            },
            Messages::Multiply => {
                self.log("Inside Multiply branch\n");
            },
            Messages::Unknown(s) => {
                self.log("Inside Unknown branch\n");
            },
        }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();
        self.log("Opened receiver\n");

        for (event, values) in receiver {
            self.process(event, values);
        }
    }

}

impl Logger for EventHandler {
    fn log(&mut self, string: &str) {
        self.logger.log(string)
    }
}

fn main() {
    let mut event_handler: EventHandler = EventHandler::new();
    event_handler.log("Initialized handler\n");
    event_handler.recv();
}
