use std::fmt;

#[derive(Default, Debug)]
pub struct Shell {
    histories: Vec<Line>,
    current: Line,
}

impl Shell {
    pub fn exec(&mut self) {
        self.histories.push(self.current.clone());
        self.current = Default::default();
    }

    pub fn prompt(&self) -> String {
        // format!("[{}] $ ", self.current.work_dir.base)
        format!("[{}] $ ", self.current.now)
    }

    pub fn update(&mut self, value: String) {
        self.current.value = value
    }

    pub fn histories(&self) -> Vec<Line> {
        self.histories.clone()
    }

    // fn e(&mut self) {
    //     self.current.value
    // }
}

#[derive(Debug, Clone)]
pub struct Line {
    now: String,
    work_dir: WorkDir,
    value: String,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            now: get_current_time(),
            work_dir: Default::default(),
            value: Default::default(),
        }
    }
}

fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    String::from(date.to_locale_time_string("en-US"))
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "[{}] $ {}", self.work_dir.base, self.value)
        write!(f, "[{}] $ {}", self.now, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct WorkDir {
    base: String,
}

impl Default for WorkDir {
    fn default() -> Self {
        Self {
            base: "/home/ec8-user".to_string(),
        }
    }
}
