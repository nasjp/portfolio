use std::fmt;

#[derive(Debug)]
pub struct Shell {
    pub histories: Vec<Line>,
    pub current: Line,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub now: String,
    pub work_dir: WorkDir,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct WorkDir {
    pub base: String,
}

pub fn init_shell() -> Shell {
    Shell {
        histories: vec![],
        current: init_line(),
    }
}

fn init_line() -> Line {
    Line {
        now: get_current_time(),
        work_dir: WorkDir {
            base: "/home/ec8-user".to_string(),
        },
        value: "".to_string(),
    }
}

impl Shell {
    pub fn exec(&mut self) {
        self.histories.push(self.current.clone());
        self.current = init_line();
    }

    pub fn prompt(&self) -> String {
        // format!("[{}] $ ", self.current.work_dir.base)
        format!("[{}] $ ", self.current.now)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "[{}] $ {}", self.work_dir.base, self.value)
        write!(f, "[{}] $ {}", self.now, self.value)
    }
}

impl Line {
    pub fn update(&mut self, value: String) {
        self.value = value
    }
}

fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    String::from(date.to_locale_time_string("en-US"))
}
