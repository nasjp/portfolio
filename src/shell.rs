use std::fmt;

#[derive(Default, Debug)]
pub struct Shell {
    histories: Vec<Line>,
    current: Line,
}

impl Shell {
    pub fn exec(&mut self) {
        self.histories.push(self.current.clone());

        let r = Line {
            now: Default::default(),
            work_dir: Default::default(),
            value: self.e(),
            typ: LineType::ExecResult,
        };
        self.histories.push(r);

        self.current = Default::default();
    }

    pub fn prompt(&self) -> String {
        format!("[{}] $ ", self.current.now)
    }

    pub fn update(&mut self, value: String) {
        self.current.value = value
    }

    pub fn histories(&self) -> Vec<Line> {
        self.histories.clone()
    }

    fn e(&mut self) -> String {
        let r = self.current.parse();
        match r {
            Ok(cmd) => cmd.run(),
            Err(e) => e,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    now: String,
    work_dir: WorkDir,
    value: String,
    typ: LineType,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            now: get_current_time(),
            work_dir: Default::default(),
            value: Default::default(),
            typ: LineType::Statement,
        }
    }
}

impl Line {
    fn parse(&self) -> Result<Cmd, String> {
        if self.value.starts_with("echo") {
            return Ok(Cmd::Echo(self.value.replacen("echo", "", 1)));
        }

        if self.value.starts_with("pwd") {
            return Ok(Cmd::Pwd(self.work_dir.dir.to_string()));
        }

        let vec: Vec<&str> = self.value.split_whitespace().collect();

        if vec.is_empty() {
            return Ok(Cmd::Return);
        }

        let cmd = vec[0].trim().to_string();
        Err(format!("shell: Unknown command: {}", cmd))
    }
}

fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    String::from(date.to_locale_time_string("en-US"))
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.typ {
            LineType::Statement => write!(f, "[{}] $ {}", self.now, self.value),
            LineType::ExecResult => write!(f, "{}", self.value),
        }
    }
}

#[derive(Debug, Clone)]
struct WorkDir {
    dir: String,
}

impl Default for WorkDir {
    fn default() -> Self {
        Self {
            dir: "/home/ec8-user".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum LineType {
    Statement,
    ExecResult,
}

enum Cmd {
    Echo(String),
    Pwd(String),
    Return,
}

impl Cmd {
    fn run(&self) -> String {
        match self {
            Self::Echo(v) => Self::echo(v.to_string()),
            Self::Pwd(v) => Self::echo(v.to_string()),
            Self::Return => "".to_string(),
        }
    }

    fn echo(value: String) -> String {
        value
    }
}
