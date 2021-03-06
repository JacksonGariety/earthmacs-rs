use syntect;
use syntect::easy::HighlightLines;
use std::cmp::{max};
// use std::fs::File;
// use std::io::Write;
use cell::Cell;
use std::path::PathBuf;
use util;
use transaction::Transaction;

pub struct Buffer {
    pub lines: Vec<Vec<Cell>>,
    pub path: PathBuf,
    pub highlighter: Option<syntect::parsing::SyntaxDefinition>,
    pub ts: Option<syntect::highlighting::ThemeSet>,
    pub transactions: Vec<Transaction>,
}

impl Buffer {
    pub fn new(path: PathBuf, highlighter: Option<syntect::parsing::SyntaxDefinition>, ts: Option<syntect::highlighting::ThemeSet>) -> Buffer {
        Buffer {
            lines: vec![],
            path: path,
            highlighter: highlighter,
            ts: ts,
            transactions: vec![],
        }
    }

    pub fn save(&self) {
        // match File::create(&self.path) {
        //     Ok(mut f) => {
        //         let mut lns = self.lines.join(&Cell::new('\n', 1));
        //         // lns.push('\n');
        //         // match f.write_all(lns) {
        //         //     Ok(_) => (),
        //         //     Err(e) => panic!(e)
        //         // };
        //     },
        //     Err(_) => ()
        // }
    }

    pub fn char_at(&mut self, x: i32, y: i32) -> Option<char> {
        let ref line = self.lines[y as usize];
        match line.iter().nth(x as usize) {
            Some(cell) => { return Some(cell.ch) },
            None => { return None },
        }
    }

    pub fn remove(&mut self, x: i32, y: i32, t: bool) {
        let mut line = self.lines[y as usize].clone();
        if t {
            match self.char_at(x, y) {
                Some(ch) => {
                    self.transactions.push(Transaction{
                        x: x,
                        y: y,
                        add: false,
                        text: ch.to_string(),
                    });
                },
                None => {}
            }
        }
        if x == -1 || line.len() == 0 {
            self.lines[(y - 1) as usize].append(&mut line);
            self.remove_line(y as usize);
        } else {
            let (a, b) = line.split_at(x as usize);
            let mut new = a.to_vec();
            new.append(&mut (&b[1..]).to_vec());
            self.lines[y as usize] = new;
        }
        self.highlight_line(y);
    }

    pub fn insert(&mut self, c: &str, x: i32, y: i32, t: bool) {
        let mut line = self.lines[y as usize].clone();
        let mut new = vec![];
        if line.len() == 0 {
            line.append(&mut c.chars().map(|ch| Cell::new(ch, 1)).collect());
            new = line;
        } else {
            let (a, b) = line.split_at(x as usize);
            new.append(&mut a.to_vec());
            new.append(&mut c.chars().map(|ch| Cell::new(ch, 1)).collect());
            new.append(&mut b.to_vec());
        }

        self.lines[y as usize] = new;
        self.highlight_line(y);
        if t {
            self.transactions.push(Transaction{
                x: x,
                y: y,
                add: true,
                text: c.to_string(),
            });
        }
    }

    pub fn highlight_line(&mut self, y: i32) {
        match self.highlighter {
            Some(ref highlighter) => {
                let mut h = HighlightLines::new(&highlighter, &self.ts.as_ref().unwrap().themes["base16-ocean.dark"]);
                let line_string: String = self.lines[y as usize].iter().cloned().map(|c| c.ch).collect();
                self.lines[y as usize] = vec![];
                let ranges = h.highlight(line_string.as_str());
                for (style, text) in ranges {
                    let color = util::rgb_to_short(format!("{0:02.x}{1:02.x}{2:02.x}", style.foreground.r, style.foreground.g, style.foreground.b).as_str());
                    for ch in text.chars() {
                        self.lines[y as usize].push(Cell::new(ch, color as i32));
                    }
                }
            },
            None => ()
        }
    }

    pub fn insert_newline(&mut self, x: i32, y: i32) {
        let line = self.lines[y as usize].clone();
        let (a, b) = line.split_at(x as usize);
        self.lines[y as usize] = a.to_vec();
        self.lines.insert((y + 1) as usize, b.to_vec());
    }

    pub fn remove_line(&mut self, index: usize) {
        self.lines.remove(index);
    }

    pub fn eol(&self, y: i32) -> i32 {
        max(0, (self.lines[y as usize].len() as i32) - 1)
    }

    pub fn eof(&self) -> i32 {
        self.lines.len() as i32
    }
}
