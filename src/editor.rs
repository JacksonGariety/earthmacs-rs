use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use ncurses::*;

use buffer::Buffer;

static COLOR_PAIR_DEFAULT: i16 = 1;

pub struct Editor {
    pub buffers: Vec<Buffer>,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            buffers: vec![],
        }
    }

    pub fn open(&mut self, path: String) {
        match File::open(&path) {
            Ok(f) => {
                let mut buf = Buffer::new(path);
                let reader = BufReader::new(f);
                for line in reader.lines() {
                    buf.append_line(line.unwrap());
                }
                self.buffers.push(buf);
            },
            Err(_) => ()
        }

        let new_win = self.buffers[0].windows[0].split_horizontally();
        self.buffers[0].windows.push(new_win);
    }

    pub fn draw(&mut self) {
        for buffer in &mut self.buffers {
            for (id, window) in buffer.windows.iter_mut().enumerate() {
                refresh();
                init_pair(COLOR_PAIR_DEFAULT, 3, -1);

                let lines = buffer.lines.iter().skip(window.scroll_y as usize).take(window.height as usize);

                for (index, line) in lines.enumerate() {
                    wmove(window.pane, (index + 1) as i32, 0);
                    wclrtoeol(window.pane);
                    waddstr(window.pane, format!(" {}", line).as_str());
                }

                // update cursor
                wmove(window.pane, (buffer.cursor_y - window.scroll_y) + 1, buffer.cursor_x + 1);
                wresize(window.pane, window.real_height(), window.real_width());
                mvwin(window.pane, window.real_y(), window.real_x());
                if id == buffer.active_window as usize {
                    wattron(window.pane, COLOR_PAIR(COLOR_PAIR_DEFAULT));
                }
                box_(window.pane, 0, 0);
                wattroff(window.pane, COLOR_PAIR(COLOR_PAIR_DEFAULT));
                refresh();
                wrefresh(window.pane);
            }
        }
    }

    pub fn height() -> i32 {
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        max_y
    }
}
