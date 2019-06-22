use crate::utils::Pos;
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

pub struct Args {
    lines: (u64, u64),
    cols: Vec<u64>,
    args: Vec<String>,
    beg_pos: Pos,
    end_pos: Pos,
}

impl Args {

    pub fn new(raw_lines: Value, beg_pos: Pos, end_pos: Pos) { //-> Args {

        let lines_vec = Args::unwrap_raw_lines(&raw_lines);
        // let args = Args::parse_args(&lines_vec);

        // let string_vec = Args::unwrap_raw_lines(raw_lines, beg_pos, end_pos);

        // Args {
        //     lines: (beg_pos.line(), end_pos.line()),
        //     cols:,
        //     args:,
        //     beg_pos,
        //     end_pos,
        // }

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

    pub fn parse_args(lines_vec: &Vec<String>, beg_pos: Pos, end_pos: Pos) -> String { //Vec<Vec<String>> {
        let n_lines = lines_vec.len();
        let (bl, bc) = beg_pos.get();
        let (el, ec) = end_pos.get();

        // let start = &lines_vec[0];
        // let end = &lines_vec[lines_vec.len()-1];
        // let N_lines = end_pos.line() - beg_pos.line() + 1;
        // let all = lines_vec.join("");

        // split_every_line:
        let mut sel: Vec<Vec<&str>> = lines_vec.iter()
            .map(|s|
                 UnicodeSegmentation::graphemes(&s[..], true).collect::<Vec<&str>>()
            ).collect();

        // sel[0] = sel[0][(bc-1) as usize .. ].to_vec();
        // sel[n_lines-1] = sel[n_lines-1][.. (ec-1) as usize].to_vec();

        let a: String = sel.iter().map(|v|
                                    v.join("")).map(|s| s.to_string()).collect();

        let o: Vec<Vec<String>> = sel.iter().map(|v|
                                                 v.iter()
                                                 .map(|s|
                                                      s.to_string()
                                                 ).collect()
        ).collect();

        let mut t: Vec<String> = o.iter().map(|v| v.join("")).collect();

        // t[0] = o[0][(bc-1) as usize ..].join("");
        // t[n_lines-1] = o[n_lines-1][.. (ec-1) as usize].join("");

        format!("t := {:?}", t)
        // format!("{} {:?}", bc, o[0][(bc-1) as usize ..].join(""))
        // format!("broken line {} {:?}", ec, o[n_lines-1][.. (ec-1) as usize].join("")) //[(ec-1) as usize])
        // format!("broken line {} {:?}", ec, sel[n_lines-1][(ec-1) as usize]) //[(ec-1) as usize])
        // format!("broken nums {} {} {:?}", ec, sel[n_lines-1].len(), sel[n_lines-1]) //[(ec-1) as usize])
        // characters.iter().map(|c| c.to_string()).collect::<String>()
        // String::new()
        // o
    }

}
