extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session, Value, CallError};

use std::collections::HashMap;
use std::fmt::Debug;

mod utils;
use utils::*;
use utils::{Log, Logger, Pos};

mod args;
use args::Args;

#[derive(Debug, Clone)]
enum BPairs {
    Brack,
    Paren,
    Curly
}

impl BPairs{

    fn to_string_pair(&self) -> (String, String) {
        match &self {
            BPairs::Brack   =>  (String::from(r"\["), String::from(r"\]")),
            BPairs::Paren   =>  (String::from("("), String::from(")")),
            BPairs::Curly   =>  (String::from("{"), String::from("}")),
        }
    }

    fn to_simple_string_open(&self) -> String {
        match &self {
            BPairs::Brack   =>  String::from("["),
            BPairs::Paren   =>  String::from("("),
            BPairs::Curly   =>  String::from("{")
        }
    }

    fn to_simple_string_close(&self) -> String {
        match &self {
            BPairs::Brack   =>  String::from("]"),
            BPairs::Paren   =>  String::from(")"),
            BPairs::Curly   =>  String::from("}")
        }
    }


    fn array() -> [BPairs;3] {
        [BPairs::Brack, BPairs::Paren, BPairs::Curly]
    }
}

struct App {
    logger: Option<Log>
}

impl App {

    pub fn new() -> App {
        App { logger: None }
    }

    pub fn new_with_log() -> App {
        App { logger: Some(Log::new("/tmp/delinhere_app.log")) }
    }

    fn _search_bracket_pair_backwards_arg(bracket_pair: &BPairs) -> Vec<Value> {
        let (startb, endb) = bracket_pair.to_string_pair();
        vec![
            Value::from(startb),
            Value::from(""),
            Value::from(endb),
            Value::from("bnW"),
            Value::from(r#"(synIDattr(synID(line("."), col("."), 0), "name") =~? "string\\|comment")"#)
        ]
    }

    fn _search_bracket_pair_forwards_arg(bracket_pair: &BPairs) -> Vec<Value> {
        let (startb, endb) = bracket_pair.to_string_pair();
        vec![
            Value::from(startb),
            Value::from(""),
            Value::from(endb),
            Value::from("nW"),
            Value::from(r#"'(synIDattr(synID(line("."), col("."), 0), "name") =~? "string\\|comment")'"#)
        ]
    }

    fn _unwrap_array_result_into_tuple(o_array: Value) -> Option<(u64, u64)> {

        if let Some(array) = o_array.as_array() {
            let (v1, v2) = (array[0].clone(), array[1].clone());
            let (v1, v2) = (v1.as_u64(), v2.as_u64());
            if let (Some(o1),Some(o2)) = (v1, v2) {
                Some((o1, o2))
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    fn find_closest_bpair(&mut self, nvim: &mut Neovim) -> (BPairs, Pos) {

        let mut dual: (BPairs, Pos) = (BPairs::Brack, Pos::new(0,0));

        for bpair in BPairs::array().iter() {
            let args: Vec<Value> = App::_search_bracket_pair_backwards_arg(bpair);
            let search = nvim.call_function("searchpairpos", args);

            //TODO: We discard call errors for now. We should handle them
            let (line, col) = search.ok()
                .and_then(|array| Self::_unwrap_array_result_into_tuple(array))
                .unwrap_or((0u64,0u64));

            // TODO: Here we never check if the closest is a valid bracket, we could potentially
            // find unbalanced bpairs.
            dual = {
                let candidate = Pos::new(line, col);
                if { dual.1 < candidate } { (bpair.clone(), candidate) } else { dual }
            };
        }

        self.log(&format!("Closest {:?}\n", dual));

        dual

    }

    fn find_args(&mut self, nvim: &mut Neovim) -> Vec<String> {
        let (bpair, bpos) = self.find_closest_bpair(nvim);
        let searchpairpos_args: Vec<Value> = App::_search_bracket_pair_forwards_arg(&bpair);
        let search = nvim.call_function("searchpairpos", searchpairpos_args);
        self.log(&format!("search {:?}\n", search));
        let (line, col) = search.ok()
            .and_then(|array| Self::_unwrap_array_result_into_tuple(array))
            .unwrap_or((0u64,0u64));
        let epos = Pos::new(line, col);
        // let getline_args = vec![Value::from(pos.line()), Value::from(line)];
        self.log(&format!("line {} col {}\n", line, col));
        self.log(&format!("from {} to {}\n", bpos.line(), line));
        let getline_args = vec![Value::from(bpos.line()), Value::from(line)];
        let lines = nvim.call_function("getline", getline_args);

        let parse = Args::unwrap_raw_lines(lines.as_ref().unwrap());
        let all = Args::parse_args(&parse, bpos, epos);

        self.log(&format!("all {:?}\n", all));
        vec![String::new()]
    }

    fn test(&mut self, nvim: &mut Neovim, values: Vec<Value>) {
        self.log("Inside test method\n");
        self.find_args(nvim);
    }

    fn _test(&mut self, values: Vec<Value>) -> String {
        let map = values[0].as_map().unwrap();
        let mut hashmap: HashMap<String, Value> = HashMap::new();
        for (key, val) in map {
            if let Some(key_string) = key.as_str() {
                self.log(&format!("key_string {:?} -> val {:?}\n", key_string, val));
                hashmap.insert(key_string.to_string(), val.clone());
            }
        }
        return "dummy".to_string()
    }

    fn _verb_adverb_here(verb: &str, adverb: &str, here: &str) -> String {
        format!("{}{}{}", verb, adverb, here)
    }

    fn call_dih_w_normal(nvim: &mut Neovim, verb: &str, adverb: &str, here: &str) {
        let cmd = Self::_verb_adverb_here(verb, adverb, here);
        nvim.command(&format!("normal! {}", cmd));
    }

    fn call_dih_w_feedkeys(&mut self, nvim: &mut Neovim, verb: &str, adverb: &str, here: &str) {
        let cmd = Self::_verb_adverb_here(verb, adverb, here);
        let args = vec![Value::from(cmd), Value::from("n")];
        if let Err(err) = nvim.call_function("feedkeys", args) {
            self.log_err("call_dih_w_feedkeys ", err)
        }
    }

    fn delete_in_here(&mut self, nvim: &mut Neovim) {
        let (bpair, _pos) = self.find_closest_bpair(nvim);
        self.call_dih_w_feedkeys(nvim, "d", "i", &bpair.to_simple_string_open());
    }

    fn delete_around_here(&mut self, nvim: &mut Neovim) {
        let (bpair, _pos) = self.find_closest_bpair(nvim);
        self.call_dih_w_feedkeys(nvim, "d", "a", &bpair.to_simple_string_open());
    }

    fn change_in_here(&mut self, nvim: &mut Neovim) {
        let (bpair, _pos) = self.find_closest_bpair(nvim);
        self.call_dih_w_feedkeys(nvim, "c", "i", &bpair.to_simple_string_open());
    }

    fn change_around_here(&mut self, nvim: &mut Neovim) {
        let (bpair, _pos) = self.find_closest_bpair(nvim);
        self.call_dih_w_feedkeys(nvim, "c", "a", &bpair.to_simple_string_open());
    }

    fn select_in_here(&mut self, nvim: &mut Neovim) {
        let (bpair, _pos) = self.find_closest_bpair(nvim);
        self.call_dih_w_feedkeys(nvim, "v", "i", &bpair.to_simple_string_open());
    }

    fn select_around_here(&mut self, nvim: &mut Neovim) {
        let (bpair, _pos) = self.find_closest_bpair(nvim);
        self.call_dih_w_feedkeys(nvim, "v", "a", &bpair.to_simple_string_open());
    }

    fn yank_in_here(&mut self, nvim: &mut Neovim) {
        let (bpair, _pos) = self.find_closest_bpair(nvim);
        self.call_dih_w_feedkeys(nvim, "y", "i", &bpair.to_simple_string_open());
    }

    fn yank_around_here(&mut self, nvim: &mut Neovim) {
        let (bpair, _pos) = self.find_closest_bpair(nvim);
        self.call_dih_w_feedkeys(nvim, "y", "a", &bpair.to_simple_string_open());
    }

}

impl Logger for App {
    fn log(&mut self, string: &str) {
        if let Some(mut logger) = self.logger.take() {
            logger.log(string);
            self.logger = Some(logger);
        }
    }

    fn log_err<T: Debug>(&mut self, string: &str, err: T) {
        if let Some(mut logger) = self.logger.take() {
            logger.log(&format!("{} {:?}", string, err));
            self.logger = Some(logger);
        }
    }
}

enum Messages {
    DelInHere,
    DelArHere,
    ChaInHere,
    ChaArHere,
    SelInHere,
    SelArHere,
    YanInHere,
    YanArHere,
    Test,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "DelInHere" => Messages::DelInHere,
            "DelArHere" => Messages::DelArHere,
            "ChaInHere" => Messages::ChaInHere,
            "ChaArHere" => Messages::ChaArHere,
            "SelInHere" => Messages::SelInHere,
            "SelArHere" => Messages::SelArHere,
            "YanInHere" => Messages::YanInHere,
            "YanArHere" => Messages::YanArHere,
            "Test" => Messages::Test,
            _ => Messages::Unknown(event),
        }
    }
}

struct EventHandler<'a> {
    nvim: Neovim,
    app: App,
    logger: Option<&'a mut Log>
}

impl<'a> EventHandler<'a> {

    pub fn new() -> EventHandler<'a> {

        let mut session = Session::new_parent().unwrap();
        session.set_infinity_timeout();
        let nvim = Neovim::new(session);
        let app = App::new();
        let logger = None;
        EventHandler { nvim, app, logger }

    }

    pub fn new_with_log(logger: &'a mut Log) -> EventHandler<'a> {
        let mut session = Session::new_parent().unwrap();
        session.set_infinity_timeout();
        let nvim = Neovim::new(session);
        let app = App::new_with_log();

        EventHandler { nvim, app, logger: Some(logger) }
    }

    fn process(&mut self, event: String, values: Vec<Value>) {
        self.log(&format!("Processing message from event type {}\n", event));
        let message = Messages::from(event);
        match message {
            Messages::DelInHere => { self.app.delete_in_here(&mut self.nvim); },
            Messages::DelArHere => { self.app.delete_around_here(&mut self.nvim); },
            Messages::ChaInHere => { self.app.change_in_here(&mut self.nvim); },
            Messages::ChaArHere => { self.app.change_around_here(&mut self.nvim); },
            Messages::SelInHere => { self.app.select_in_here(&mut self.nvim); },
            Messages::SelArHere => { self.app.select_around_here(&mut self.nvim); },
            Messages::YanInHere => { self.app.yank_in_here(&mut self.nvim); },
            Messages::YanArHere => { self.app.yank_around_here(&mut self.nvim); },
            Messages::Test => {
                self.log("Inside Test branch\n");
                self.app.test(&mut self.nvim, values);
            },
            Messages::Unknown(s) => { self.log("Inside Unknown branch\n"); },
        }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();
        self.log("Opened receiver\n");
        // let (event, values) = receiver.recv().unwrap();
        for  (event, values) in receiver {
            self.log("Got some events\n");
            self.process(event, values);
        }
        self.log("After process.\n");
    }

}

impl<'a> Logger for EventHandler<'a> {
    fn log(&mut self, string: &str) {
        if let Some(logger) = self.logger.take() {
            logger.log(string);
            self.logger = Some(logger);
        }
    }
}

fn main() {
    let mut logger = Log::new("/tmp/delinhere.log");
    // let mut event_handler: EventHandler = EventHandler::new();
    let mut event_handler: EventHandler = EventHandler::new_with_log(&mut logger);
    event_handler.log("Initialized handler\n");
    loop {event_handler.recv();}
    event_handler.log("Finishing\n");
}
