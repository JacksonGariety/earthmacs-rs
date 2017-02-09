use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::cmp::{min, max};
use ncurses::*;

pub struct Drawer {
    pub prompt: String,
    pub value: String,
    pub lines: Vec<String>,
    pub active_line_index: i32,
}

static COLOR_PAIR_DEFAULT: i16 = 1;

impl Drawer {
    pub fn new_find_files(path: &PathBuf) -> Drawer {
        let dir;
        if path.is_dir() { dir = path.to_owned(); }
        else { dir = path.parent().unwrap().to_path_buf(); }
        let paths: Vec<String> = fs::read_dir(dir.clone()).unwrap().map(|res| res.unwrap().file_name().to_string_lossy().into_owned()).collect();
        let index = paths.len();
        
        Drawer{
            prompt: "Find files: ".to_string(),
            value: dir.to_str().unwrap().to_string() + "/",
            lines: paths,
            active_line_index: (index - 1) as i32,
        }
    }

    pub fn draw(&self, max_x: i32, max_y: i32) {
        let mut y = max_y - 1;
        let top_border = (0..max_x).map(|_| "-").collect::<String>();

        mv(y, 0);
        clrtoeol();
        let ln = format!("{}{}", self.prompt, self.value);
        addstr(ln.as_str());
        y -= 1;

        mv(y, 0);
        attron(COLOR_PAIR(COLOR_PAIR_DEFAULT));
        addstr(top_border.as_str());
        attroff(COLOR_PAIR(COLOR_PAIR_DEFAULT));
        y -= 1;

        for (index, line) in self.lines.iter().enumerate() {
            mv(y, 0);
            clrtoeol();
            if index == self.active_line_index as usize {attron(COLOR_PAIR(50));}
            addstr(line.as_str());
            if index == self.active_line_index as usize {attroff(COLOR_PAIR(50));}
            y -= 1;
        }
        mv(y, 0);
        attron(COLOR_PAIR(COLOR_PAIR_DEFAULT));
        addstr(top_border.as_str());
        attroff(COLOR_PAIR(COLOR_PAIR_DEFAULT));
        mv(max_y - 1, ln.len() as i32);
    }

    pub fn next_item(&mut self) {
        if self.active_line_index == 0 {
            self.active_line_index = self.lines.len() as i32 - 1;
        } else {
            self.active_line_index = max(0, self.active_line_index - 1);
        }
    }

    pub fn prev_item(&mut self) {
        if self.active_line_index == self.lines.len() as i32 - 1 {
            self.active_line_index = 0;
        } else {
            self.active_line_index = min((self.lines.len() - 1) as i32, self.active_line_index + 1);
        }
    }

    pub fn update_list(&mut self) {
        let v = &self.value;
        let pb = PathBuf::from(v);
        let mut dir = pb.clone();
        if pb.to_string_lossy().to_owned().chars().last().unwrap() != '/' {
            dir = pb.parent().unwrap().to_owned();
        }
        let file = Path::new(v).file_name().unwrap();
        let paths: Vec<String> = fs::read_dir(dir).unwrap().map(|res| res.unwrap().file_name().to_string_lossy().into_owned()).collect();
        self.lines = paths.iter().filter(|path| compare(path, file.to_str().unwrap(), min(path.len(), file.to_str().unwrap().len())) > 0.0).cloned().collect();
        self.active_line_index = self.lines.len() as i32 - 1;
    }

    pub fn handle_key(&mut self, key: &str) {
        match key {
            "<Backspace>" | "<DEL>" => {
                if self.value.len() != 0 {
                    self.value = self.value[..(self.value.len() - 1)].to_string();
                }
                self.update_list();
            },
            "<Tab>" => {
                let p = PathBuf::from(&self.value).join(&self.lines[self.active_line_index as usize]);
                if p.is_dir() {
                    self.value = p.to_str().unwrap().to_string() + "/";
                    self.update_list();
                }
            },
            "<C-l>" => {
                let old = self.value.clone();
                let mut p = PathBuf::from(&old);
                p.pop();
                self.value = p.to_str().unwrap().to_string() + "/";
                self.update_list();
            },
            _ => {
                self.value = self.value.clone() + key;
                self.update_list();
            }
        }
    }
}


pub fn compare(a: &str, b: &str, size: usize) -> f64 {
    if a == b {
        return 1.0;
    }

    //loop through first string add unique ngrams to vec
    let mut ngrams = Vec::new();
    for ngram in compute_ngram_tokens(a, size) {
        if !ngrams.contains(&ngram) {
            ngrams.push(ngram);
        }
    }

    //loop through second string
    let mut intersection = 0;
    let mut difference = 0;
    for ngram in compute_ngram_tokens(b, size) {
        if ngrams.contains(&ngram) {
            intersection += 1
        } else {
            difference += 1;
        }
    }

    intersection as f64 / ((ngrams.len() as i32 + difference) as f64)
}

pub fn compute_ngram_tokens(s: &str, size: usize) -> Vec<String> {
    let mut tokens = Vec::new();

    if s.len() < size {
        tokens.push(format!("{s:<width$}", s=s, width=size));
    } else {
        for i in 0..(s.len() - size + 1) {
            unsafe {
                tokens.push(s.slice_unchecked(i, i + size).to_string());
            }
        }
    }

    tokens
}
