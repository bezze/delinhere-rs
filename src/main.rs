extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session, Value, CallError};

use std::collections::HashMap;
use std::fmt::Debug;

mod utils;
use utils::*;
use utils::{Log, Logger, Pos};
use utils::BPairs;

mod args;
use args::Args;

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
                if o1 != 0u64 && o2 != 0u64 {
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
        else {
            None
        }
    }

    fn searchpairpos(nvim: &mut Neovim, args: Vec<Value>) -> Option<Pos> {

        let search = nvim.call_function("searchpairpos", args);
        match search {
            Ok(array) => {
                if let Some((line, col)) = Self::_unwrap_array_result_into_tuple(array) {
                    Some(Pos::new(line,col))
                }
                else {
                    None
                }
            },
            //TODO: We discard call errors for now. We should handle them
            Err(_error) => {
                None
            },
        }

    }

    fn find_closest_bpair(&mut self, nvim: &mut Neovim) -> Option<(BPairs, Pos)> {

        let mut dual: Option<(BPairs, Pos)> = None;

        for bpair in BPairs::array().iter() {
            let args: Vec<Value> = App::_search_bracket_pair_backwards_arg(bpair);
            // TODO: Here we never check if the closest is a valid bracket, we could potentially
            // find unbalanced bpairs.
            let search = App::searchpairpos(nvim, args);
            if let Some(pos) = search {
                dual = if let Some((old_bpair, old_pos)) = dual {
                    if pos > old_pos {
                        Some((bpair.clone(), pos))
                    }
                    else {
                        Some((old_bpair, old_pos))
                    }
                }
                else {
                    Some((bpair.clone(), pos))
                }
            }
        }

        self.log(&format!("Closest {:?}\n", dual));
        dual

    }

    fn find_args(&mut self, nvim: &mut Neovim) -> Option<args::Args> {

        if let Some((bpair, bpos)) = self.find_closest_bpair(nvim) {
            let searchpairpos_args: Vec<Value> = App::_search_bracket_pair_forwards_arg(&bpair);
            let search = App::searchpairpos(nvim, searchpairpos_args);
            self.log(&format!("search {:?}\n", search));

            if let Some(epos) = search {
                let (line, col) = epos.get();
                self.log(&format!("line {} col {}\n", line, col));
                self.log(&format!("from {} to {}\n", bpos.line(), line));
                let getline_args = vec![Value::from(bpos.line()), Value::from(line)];
                let lines = nvim.call_function("getline", getline_args);
                let mut args = Args::new(lines.unwrap(), bpos, epos, &mut self.logger);
                self.log(&format!("all {:?}\n", args));
                Some(args)
            } else { None }
        } else { None }

    }

    fn test(&mut self, nvim: &mut Neovim, values: Vec<Value>) {

        let args = self.find_args(nvim);
        self.log("After find_args\n");
        let string = args.map_or(String::new(), |mut a|{
            self.log("Before fuckup\n");
            a.reconstruct_args()
        });
        // let string = String::from("hola");

        self.log(&format!("reconstruct := {:?}\n", &string));

        if let Some((bpair, _pos)) = self.find_closest_bpair(nvim) {

            let cmd = Self::_verb_adverb_here("d", "i", &bpair.to_simple_string_open());
            let dihargs = Value::from(vec!(
                    Value::from(cmd),
                    Value::from("n"),
                    Value::from(false)
            ));

            let call_dih = Value::from(vec![
                                       Value::from("nvim_feedkeys"), dihargs
            ]);

            let setregargs = Value::from(vec!(
                Value::from("-"),
                Value::from(string),
            ));

            let call_wrapper = Value::from(vec![Value::from("setreg"), setregargs]);

            let call_setreg = Value::from(vec!(
                    Value::from("nvim_call_function"), call_wrapper

            ));

            let feedargs_2 = Value::from(vec!(
                    Value::from("\"-P"),
                    Value::from("n"),
                    Value::from(false)
            ));


            let call_write = Value::from(vec!(
                    Value::from("nvim_feedkeys"), feedargs_2
            ));

            let atomic_call = vec![call_dih, call_setreg, call_write];

            self.log(&format!("atomic_call := {:?}\n", &atomic_call));

            let result = nvim.call_atomic(atomic_call);

            self.log(&format!("result := {:?}\n", &result));

        }

    }

    fn test_3(&mut self, nvim: &mut Neovim, values: Vec<Value>) {

        let string = String::from("This is a test string");

        if let Some((bpair, _pos)) = self.find_closest_bpair(nvim) {

            let cmd = Self::_verb_adverb_here("d", "i", &bpair.to_simple_string_open());
            let dihargs = Value::from(vec!(
                    Value::from(cmd),
                    Value::from("n"),
                    Value::from(false)
            ));

            let call_dih = Value::from(vec![
                                       Value::from("nvim_feedkeys"), dihargs
            ]);

            let setregargs = Value::from(vec!(
                Value::from("\"j"),
                Value::from(string),
            ));

            let call_wrapper = Value::from(vec![Value::from("setreg"), setregargs]);

            let call_setreg = Value::from(vec!(
                    Value::from("nvim_call_function"), call_wrapper

            ));

            let feedargs_2 = Value::from(vec!(
                    Value::from("\"jP"),
                    Value::from("n"),
                    Value::from(false)
            ));


            let call_write = Value::from(vec!(
                    Value::from("nvim_feedkeys"), feedargs_2
            ));

            let atomic_call = vec![call_setreg, call_dih, call_write];

            self.log(&format!("atomic_call := {:?}\n", &atomic_call));

            let result = nvim.call_atomic(atomic_call);

            self.log(&format!("result := {:?}\n", &result));

        }

    }

    fn test_2(&mut self, nvim: &mut Neovim, values: Vec<Value>) {
        self.log("Inside test method\n");

        let args = self.find_args(nvim);
        if let Some(mut a) = args {
            let arg_string = a.reconstruct_args();
            self.delete_in_here(nvim);
            self.write_in_here(nvim, arg_string);
        }

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

    fn _dih_write(&mut self, nvim: &mut Neovim, verb: &str, adverb: &str) {
        if let Some((bpair, _pos)) = self.find_closest_bpair(nvim) {
            let cmd = Self::_verb_adverb_here(verb, adverb, &bpair.to_simple_string_open());
            let args = vec![Value::from(cmd),
            Value::from("n")];
            if let Err(err) = nvim.call_function("feedkeys", args) {
                self.log_err("call_dih_w_feedkeys ", err)
            }
        }
    }

    fn write_in_here(&mut self, nvim: &mut Neovim, string: String) {

        let setregargs = vec![
            Value::from("\"\""),
            Value::from(string),
            Value::from("n")
        ];

        if let Err(err) = nvim.call_function("setreg", setregargs) {
            self.log_err("setreg ", err)
        }

        let feedargs = vec![
            Value::from("\"\"P"),
            Value::from("n")
        ];

        if let Err(err) = nvim.call_function("feedkeys", feedargs) {
            self.log_err("feedkeys ", err)
        }


    }

    fn _verb_adverb_here(verb: &str, adverb: &str, here: &str) -> String {
        format!("{}{}{}", verb, adverb, here)
    }

    fn call_dih_w_feedkeys(&mut self, nvim: &mut Neovim, verb: &str, adverb: &str) {
        if let Some((bpair, _pos)) = self.find_closest_bpair(nvim) {
            let cmd = Self::_verb_adverb_here(verb, adverb, &bpair.to_simple_string_open());
            let args = vec![Value::from(cmd), Value::from("n")];
            if let Err(err) = nvim.call_function("feedkeys", args) {
                self.log_err("call_dih_w_feedkeys ", err)
            }
        }
    }

    fn delete_in_here(&mut self, nvim: &mut Neovim) {
        self.call_dih_w_feedkeys(nvim, "d", "i");
    }

    fn delete_around_here(&mut self, nvim: &mut Neovim) {
        self.call_dih_w_feedkeys(nvim, "d", "a");
    }

    fn change_in_here(&mut self, nvim: &mut Neovim) {
        self.call_dih_w_feedkeys(nvim, "c", "i");
    }

    fn change_around_here(&mut self, nvim: &mut Neovim) {
        self.call_dih_w_feedkeys(nvim, "c", "a");
    }

    fn select_in_here(&mut self, nvim: &mut Neovim) {
        self.call_dih_w_feedkeys(nvim, "v", "i");
    }

    fn select_around_here(&mut self, nvim: &mut Neovim) {
        self.call_dih_w_feedkeys(nvim, "v", "a");
    }

    fn yank_in_here(&mut self, nvim: &mut Neovim) {
        self.call_dih_w_feedkeys(nvim, "y", "i");
    }

    fn yank_around_here(&mut self, nvim: &mut Neovim) {
        self.call_dih_w_feedkeys(nvim, "y", "a");
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
