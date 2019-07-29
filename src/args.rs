use crate::utils::Pos;
use crate::utils::BPairs;

use crate::utils::{Log, Logger};

use neovim_lib::{Value};
use unicode_segmentation::UnicodeSegmentation;


/// .0         1         2
/// .012345678901234567890123456789
/// ..
/// 77      v---- cols[0] = 7
/// 78  fn (arg1, arg2, arg3,                     --> lines[0] = 78 \
/// 79      arg4, arg5, arg6, arg7, arg8, arg9,                      > lines[1] - lines[0] = len(cols) - 1
/// 80      ^            arg10, arg11)            --> lines[1] = 80 /
/// 81      |            ^------- cols[2] = 20
/// ..      ---- cols[1] = 7
///
/// lines: ...
/// pos_vec: [(78, 3), (79, 7), (80, 20)]
/// args: [arg1, arg2, arg3,| arg4, arg5, arg6, arg7, arg8, arg9,| arg10, arg11]
///         0      1     2      0     1     2     3     4     5      0      1
///         0      1     2      3     4     5     6     7     8      9      10
///                     N=3                                 N=9     
///     0 .. 3
///     3 .. 3+6
///     9 .. 9+2
/// counts: [3, 6, 2]
/// beg_pos: (78, 6)
/// end_pos: (80, 32)

/// lines: Vec<Vec<String>> -> each string is actually a unicode char. The outer vec are the lines
/// pos_vec: Vec<Pos>, -> the Pos of the first non-whitespace char in each line
/// args: Vec<String>, -> a curated list of args
/// counts: Vec<usize>, -> the number of arguments per line.
/// beg_pos: Pos, -> the starting point of the bpair
/// end_pos: Pos, -> the ending point of the bpair
/// logger: Option<Log>,
#[derive(Debug)]
pub struct Args {
    lines: Vec<Vec<String>>,
    pos_vec: Vec<Pos>,
    args: Vec<String>,
    counts: Vec<usize>,
    beg_pos: Pos,
    end_pos: Pos,
    debug: Vec<usize>,
    logger: Option<Log>,
}

impl Logger for Args {
    fn log(&mut self, string: &str) {
        if let Some(mut logger) = self.logger.take() {
            logger.log(string);
            self.logger = Some(logger);
        }
    }

    fn log_err<T: std::fmt::Debug>(&mut self, string: &str, err: T) {
        if let Some(mut logger) = self.logger.take() {
            logger.log(&format!("{} {:?}", string, err));
            self.logger = Some(logger);
        }
    }
}

impl Args {

    pub fn new(raw_lines: Value, beg_pos: Pos, end_pos: Pos, ext_logger: &mut Option<Log>) -> Args {

        let lines_vec = Args::unwrap_raw_lines(&raw_lines);
        let lines: Vec<Vec<String>>  = Args::parse_lines(&lines_vec);
        let pos_vec = Args::find_pos(&lines, beg_pos);
        // let args = Args::parse_args(&lines, beg_pos, end_pos).split(", ").map(|s| s.to_string()).collect();
        if let Some(logger) = ext_logger { logger.log("Inside Args::new\n"); }
        let (args, counts) = Args::parse_args(&lines, beg_pos,
                                              end_pos, ext_logger);

        // assert_eq![pos_vec.len(), lines_vec.len()];
        // assert_eq![lines_vec.len(), lines.len()];
        // assert_eq![lines.len(), counts.len()];

        Args {
            debug: vec![pos_vec.len(), lines_vec.len(), lines.len(), counts.len()],
            lines,
            pos_vec,
            args,
            counts,
            beg_pos,
            end_pos,
            logger: Some(Log::new("/tmp/delinhere_arg.log")),
        }

    }

    fn debug_1(&self) -> String {
        let print_str = self.pos_vec.iter()
            .enumerate()
            .map(|(i, pos)|
                 format!("{} {} | {:?}\n", pos.line(), pos.col(), &self.lines[i]))
            .collect::<String>();
        print_str
    }

    pub fn unwrap_raw_lines(array_value: &Value) -> Vec<String> {
        if let Some(array) = array_value.as_array() {
            array.iter().map(
                |v| v.as_str().unwrap_or("").to_string()
            ).collect()
        }
        else {
            vec![String::new()]
        }
    }

    fn first_not_whitespace(utf8_vec: &Vec<String>) -> Option<usize> {
        for (i, s) in utf8_vec.iter().enumerate() {
            if s != " " { return Some(i) }
        }
        None
    }

    pub fn parse_lines(lines_vec: &Vec<String>) -> Vec<Vec<String>> {
        // split_every_line:
        let sel: Vec<Vec<String>> = lines_vec.iter()
            .map(|s|
                 UnicodeSegmentation::graphemes(&s[..], true).map(|s| s.to_string()).collect()
            ).collect();
        sel
    }

    /// Find first non-whitespace character position of every line and then return the Pos vector.
    fn find_pos(parsed_lines: &Vec<Vec<String>>, beg_pos: Pos) -> Vec<Pos> {

        let cols = parsed_lines.iter().map(|vs| {
            if let Some(number) = Args::first_not_whitespace(vs) {
                number
            }
            else {
                1
            }
        });

        cols.enumerate().map(|(i, n)| Pos::new(i as u64 + beg_pos.line(), n as u64 +1)).collect()
    }

    fn reprocessed_args(arg_chars: &Vec<String>, ext_logger: &mut Option<Log>) -> (Vec<String>, Vec<usize>) {
        // arg_chars contains every character (as a String) from the starting bracket pair to the
        // end bracket pair.
        if let Some(logger) = ext_logger { logger.log("Inside Args::reprocessed_args\n"); }

        let N = arg_chars.len();

        fn ch_is_open_bpair(ch: &String) -> Option<BPairs> {
            for bpair in &BPairs::array() {
                if *ch == bpair.to_simple_string_open() {
                    return Some(bpair.clone())
                }
            }
            None
        }

        fn ch_is_close_bpair(ch: &String, bpair: &BPairs) -> bool {
            bpair.to_simple_string_close() == *ch
        }

        fn add_char_2_last_buf(bufs: &mut Vec<(BPairs, String)>, chars: &str) {
            let mut last = bufs.pop().unwrap();
            last.1.push_str(chars);
            bufs.push(last);
        }

        fn add_char_2_arg(arg: &mut String, chars: &str) {
            arg.push_str(chars)
        }

        fn add_buf_2_last_buf(bufs: &mut Vec<(BPairs, String)>, chars: &str) {
            add_char_2_last_buf(bufs, chars)
        }

        fn add_buf_2_arg(arg: &mut String, chars: &str) {
            add_char_2_arg(arg, chars)
        }


        fn new_buf(bufs: &mut Vec<(BPairs, String)>, bpair: &BPairs, ch: &str) {
            bufs.push((bpair.clone(), ch.to_string()));
        }

        fn close_buf(bufs: &mut Vec<(BPairs, String)>, arg: &mut String, ch: &str, ext_logger: &mut Option<Log>) {

            if let Some(logger) = ext_logger { logger.log(&format!("Inside close_buf {:?}\n", 1)); }

            // reduce buf level
            let (_, mut chars) = bufs.pop().unwrap();
            chars.push_str(&ch);

            if bufs.len() == 0 {
                // I got the last one, so we should add to the arg
                add_buf_2_arg(arg, &chars)
            }
            else {
                // There are more bufs in the queue, we should append the srtring to the last one-
                add_buf_2_last_buf(bufs, &chars)
            }

        }

        let mut parsed_args: Vec<String> = Vec::new();
        let mut curr_arg: String = String::new();
        let mut bufs: Vec<(BPairs, String)> = Vec::new();
        let mut count_args: Vec<usize> = Vec::new();
        let mut count: usize = 1;

        for i in 0..N {
            if let Some(logger) = ext_logger { logger.log(&format!("i: {}\n", i)); }
            let ch = &arg_chars[i];
            if let Some(bpair) = ch_is_open_bpair(&ch) {
                if let Some(logger) = ext_logger { logger.log(&format!("Found open bpair {:?}\n", bpair)); }
                new_buf(&mut bufs, &bpair, ch);
            }
            else {
                if bufs.len() == 0 {
                    if let Some(logger) = ext_logger { logger.log("bufs len == 0\n"); }

                    // We can add directly to the arg
                    if *ch == "," {
                        // new arg
                        if let Some(logger) = ext_logger { logger.log("New Arg\n"); }
                        parsed_args.push(curr_arg);
                        count += 1;
                        curr_arg = String::new();
                    }
                    else if *ch == "\n" {
                        if let Some(logger) = ext_logger { logger.log("Found newline!\n"); }
                        if curr_arg.trim() == "" {
                            count -= 1;
                        }
                        count_args.push(count);
                        count = 1;
                    }
                    else {
                        if let Some(logger) = ext_logger { logger.log(&format!("Adding char {}!\n", ch)); }
                        add_char_2_arg(&mut curr_arg, ch)
                    }

                }
                else {

                    if let Some(logger) = ext_logger { logger.log(&format!("bufs len == {}\n", bufs.len())); }
                    let (bpair, _) = &bufs[bufs.len()-1];
                    if ch_is_close_bpair(&ch, bpair) {
                        if let Some(logger) = ext_logger { logger.log(&format!("Found closing bpair {:?}\n", bpair)); }
                        close_buf(&mut bufs, &mut curr_arg, ch, ext_logger)
                    }
                    else {
                        if let Some(logger) = ext_logger { logger.log("No bpair\n"); }
                        // We need to append to last buf
                        add_char_2_last_buf(&mut bufs, ch)
                    }

                }
            }
        }

        // Collecting residual bufs.
        for buf in bufs.iter() {
            let (_, chars) = buf;
            curr_arg.push_str(&chars)
        }

        // add last arg
        if curr_arg != "" {
            parsed_args.push(curr_arg);
        }

        // adding last count
        if arg_chars[N-1] != "\n" {
            count_args.push(count);
        }

        parsed_args = parsed_args.iter().map(|s| s.trim().to_string()).collect();

        if let Some(logger) = ext_logger { logger.log(&format!("Finally! {:?}\n", parsed_args)); }
        (parsed_args, count_args)
    }

    pub fn parse_args(lines: &Vec<Vec<String>>, beg_pos: Pos, end_pos: Pos, ext_logger: &mut Option<Log>) -> (Vec<String>, Vec<usize>) {
        if let Some(logger) = ext_logger { logger.log("Inside Args::parse_args\n"); }
        let n_lines = lines.len();
        let (_bl, bc) = beg_pos.get();
        let (_el, ec) = end_pos.get();

        let mut print_str = String::new();
        let mut only_args = Vec::new();

        for (i, vs) in lines.iter().enumerate() {
            let mut slice = &vs[..];

            if n_lines == 1 {
                slice = &vs[bc as usize .. ec as usize -1]
            }
            else if i == 0 {
                slice = &vs[bc as usize ..]
            }
            else if i == n_lines -1 {
                slice = &vs[.. ec as usize -1]
            }

            // only_args.push(slice.to_vec().join(""));
            for stuff in slice {
                only_args.push(stuff.clone())
            }
            only_args.push("\n".to_string());

            print_str.push_str(
                &format!("{:?}\n", slice)
            )

        }

        let reprocessed_args = Args::reprocessed_args(&only_args, ext_logger);
        reprocessed_args

    }

    pub fn cycle_args(&mut self) {
    }

    fn first_line(&self) -> String {

        let n_args: usize = self.counts[0];
        let first_pos = 0;
        let last_pos = first_pos + self.counts[0];

        let args = self.args[first_pos..last_pos].join(", ");
        args

    }

    fn last_line(&self) -> String {

        let n = self.lines.len();

        let n_args: usize = self.counts[n-1];
        let first_pos = self.counts[0..n].iter().sum::<usize>();
        let last_pos = first_pos + self.counts[n-1];

        let args = self.args[first_pos..last_pos].join(", ");
        args

    }

    fn reconstruct_line(&self, line_number: usize) -> String {
        let whitepsace_slots: u64 = self.pos_vec[line_number].col();
        let first_pos = self.counts[0..line_number].iter().sum::<usize>();
        let last_pos = first_pos + self.counts[line_number];

        let args = &self.args[first_pos..last_pos].join(", ");

        let mut line = String::new();

        for _i in 0 .. (whitepsace_slots-1) { line.push_str(" ") }

        line.push_str(args);

        line

    }

    pub fn reconstruct_args(&mut self) -> String {

        let mut stringified_args = Vec::new();
        if self.first_line() != "" {
            stringified_args.push(self.first_line());
        }
        if self.lines.len() >= 2 {
            for i in 1 .. self.lines.len() {
                let recon = self.reconstruct_line(i);
                self.log(&format!("line {} {}\n",i , recon));
                stringified_args.push(recon)
            }
        }
        stringified_args.join(",\n")
    }


}
