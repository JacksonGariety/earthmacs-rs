use buffer::Buffer;
use window::Window;
use ncurses::*;

static COLOR_PAIR_DEFAULT: i16 = 1;

// const NORTH: usize = 1;
// const SOUTH: usize = 2;
// const EAST: usize = 3;
const WEST: usize = 4;

#[derive(Clone)]
pub struct WindowTree {
    pub branches: Vec<WindowTree>,
    pub leaf: Window,
    pub parent: Option<*mut WindowTree>,
}

impl WindowTree {
    pub fn new(parent: Option<*mut WindowTree>) -> WindowTree {
        WindowTree {
            branches: vec![],
            leaf: Window::new(),
            parent: parent,
        }
    }

    pub fn find_active_window(&mut self) -> Option<&mut Window> {
        if self.leaf.active {
            return Some(&mut self.leaf);
        } else {
            for branch in &mut self.branches {
                match branch.find_active_window() {
                    Some(leaf) => { return Some(leaf); },
                    None => ()
                }
            }
        }
        None
    }

    pub fn find_active_window_tree(&mut self) -> Option<&mut WindowTree> {
        if self.leaf.active {
            return Some(self);
        } else {
            for branch in &mut self.branches {
                match branch.find_active_window_tree() {
                    Some(tree) => { return Some(tree); },
                    None => ()
                }
            }
        }
        None
    }

    pub fn focus(&mut self, direction: usize) {
        let active_tree = self.find_active_window_tree().unwrap();
        let mut is_right_index = 1;
        let mut is_left_index = 0;
        let mut is_right = false;
        let mut is_left = false;
        let mut current = active_tree as *mut WindowTree;

        if direction == WEST {
            is_right_index = 0;
            is_left_index = 1;
        }

        unsafe {
            while is_right == false {
                match (*current).parent {
                    Some(parent) => {
                        current = parent;
                    },
                    None => return
                };
                match (*current).branches[is_right_index].find_active_window_tree() {
                    None => {
                        is_right = true;
                        current = &mut (*current).branches[is_right_index];
                    },
                    _ => ()
                }
            }
            while is_left == false {
                if (*current).branches.len() == 0 {
                    is_left = true;
                } else {
                    current = &mut (*current).branches[is_left_index];
                }
            }
            active_tree.leaf.active = false;
            (*current).leaf.active = true;
        }
    }

    pub fn draw(&mut self, buffers: &Vec<Buffer>, width: i32, height: i32, x: i32, y: i32) {
        let n = self.branches.len() as i32;
        if n > 0 {
            let mut extra_width = 0;
            for (i, branch) in &mut self.branches.iter_mut().enumerate() {
                if i == (n - 1) as usize { extra_width = width % n; }
                branch.draw(buffers, (width / n) + extra_width, height, x + ((width / n) * (i as i32)), y);
            }
        } else {
            let ref buffer = buffers[self.leaf.buffer_index as usize];
            let mut lines = buffer.lines.iter().skip(self.leaf.scroll_y as usize).take(height as usize);

            for index in 0..height {
                wmove(self.leaf.pane, (index + 1) as i32, 0);
                wclrtoeol(self.leaf.pane);
                waddstr(self.leaf.pane, " ");
                match lines.next() {
                    Some(line) => {
                        for ch in line {
                            wattron(self.leaf.pane, COLOR_PAIR(ch.fg as i16));
                            waddstr(self.leaf.pane, ch.ch.to_string().as_str());
                            wattroff(self.leaf.pane, COLOR_PAIR(ch.fg as i16));
                        }
                    },
                    None => ()
                }
            }

            // position/size
            wresize(self.leaf.pane, height, width);
            mvwin(self.leaf.pane, y, x);

            // border
            if self.leaf.active {
                wattron(self.leaf.pane, COLOR_PAIR(COLOR_PAIR_DEFAULT));
            }
            box_(self.leaf.pane, 0, 0);

            // name label
            let name = buffer.path.file_name().unwrap().to_str().unwrap();
            if width >= name.len() as i32 + 4 {
                wmove(self.leaf.pane, height - 1, 4);
                waddstr(self.leaf.pane, name);
            }

            // (x,y) label
            let xy_label = format!("({},{})", self.leaf.cursor_y, self.leaf.cursor_x);
            if width >= (4 + name.len() + 4) as i32 + xy_label.len() as i32 {
                wmove(self.leaf.pane, height - 1, (4 + name.len() + 4) as i32);
                waddstr(self.leaf.pane, xy_label.as_str());
            }

            if width >= self.leaf.mode.len() as i32 + 4 {
                // mode label
                wmove(self.leaf.pane, height - 1, width - 4 - self.leaf.mode.len() as i32);
                waddstr(self.leaf.pane, self.leaf.mode.as_str());
            }

            wattroff(self.leaf.pane, COLOR_PAIR(COLOR_PAIR_DEFAULT));

            // refresh
            wnoutrefresh(self.leaf.pane);
        }
    }

    pub fn split_horizontally(&mut self) {
        let mut left = WindowTree::new(Some(self));
        left.leaf = self.leaf.clone();
        left.leaf.active = true;
        let mut right = WindowTree::new(Some(self));
        right.leaf = self.leaf.clone();
        right.leaf.pane = newwin(1,1,0,0);
        right.leaf.active = false;
        self.branches.push(left);
        self.branches.push(right);
        self.leaf = Window::new();
    }

    pub fn find_leaf(&mut self) -> Option<&mut Window> {
        let len = self.branches.len();
        if self.branches.len() > 0 {
            match self.branches[len - 1].find_leaf() {
                Some(leaf) => { return Some(leaf) },
                None => ()
            }
        } else {
            return Some(&mut self.leaf)
        }
        None
    }

    pub fn reparent_branches(&mut self) {
        let parent = self as *mut WindowTree;
        for branch in &mut self.branches {
            branch.parent = Some(parent);
            branch.reparent_branches();
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            match self.parent {
                Some(tree) => {
                    let ref mut parent = *tree;
                    for branch in &parent.branches {
                        if branch.leaf.active == false {
                            if branch.branches.len() > 0 {
                                (*self.parent.unwrap()).branches = branch.branches.clone();
                            } else {
                                (*self.parent.unwrap()).leaf = branch.leaf.clone();
                                (*self.parent.unwrap()).branches = vec![];
                            }
                        }
                    }
                    (*self.parent.unwrap()).reparent_branches();
                    (*self.parent.unwrap()).find_leaf().unwrap().active = true;
                },
                None => ()
            }
        }
    }
}
