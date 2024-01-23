use std::fmt::Display;
use std::io::Write;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Fmt {
    Default,
    Bold,
    Underline,
    UnderlineStop,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Col {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    DarkCyan,
    Grey,
    DarkGrey,
    BrightRed,
    Lime,
    BrightYellow,
    BrightBlue,
    Magenta,
    Cyan,
    White,
}

macro_rules! f {
    ($s:expr) => {
        format!("{}[{}m", 27 as char, $s).as_str()
    };
}

pub struct Printer {
    s: String,
    last_col: Col,
    last_bg: Col,
    last_fmt: Fmt,
}

impl Printer {
    pub fn start() -> Self {
        Printer {
            s: "".to_string(),
            last_col: Col::White,
            last_bg: Col::Black,
            last_fmt: Fmt::Default,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.s.push_str(text);
        self
    }

    pub fn text_ln(self, text: &str) -> Self {
        let f = self.last_fmt;
        let c = self.last_col;
        let g = self.last_bg;
        self.text(text).def().ln().col(c).bg(g).fmt(f)
    }

    pub fn ln(mut self) -> Self {
        let f = self.last_fmt;
        let c = self.last_col;
        let b = self.last_bg;
        self.s.push_str(f!(0));
        self.s.push(10 as char);
        self.col(c).bg(b).fmt(f)
    }

    pub fn fmt(mut self, fmt: Fmt) -> Self {
        match fmt {
            Fmt::Default => self.s.push_str(f!(0)),
            Fmt::Bold => self.s.push_str(f!(1)),
            Fmt::Underline => self.s.push_str(f!(4)),
            Fmt::UnderlineStop => self.s.push_str(f!(24)),
        }
        self.last_fmt = fmt;
        self
    }

    pub fn def(self) -> Self {
        self.fmt(Fmt::Default)
    }

    pub fn col(mut self, col: Col) -> Self {
        match col {
            Col::Black => self.s.push_str(f!(30)),
            Col::Red => self.s.push_str(f!(31)),
            Col::Green => self.s.push_str(f!(32)),
            Col::Yellow => self.s.push_str(f!(33)),
            Col::Blue => self.s.push_str(f!(34)),
            Col::Purple => self.s.push_str(f!(35)),
            Col::DarkCyan => self.s.push_str(f!(36)),
            Col::Grey => self.s.push_str(f!(37)),
            Col::DarkGrey => self.s.push_str(f!(90)),
            Col::BrightRed => self.s.push_str(f!(91)),
            Col::Lime => self.s.push_str(f!(92)),
            Col::BrightYellow => self.s.push_str(f!(93)),
            Col::BrightBlue => self.s.push_str(f!(94)),
            Col::Magenta => self.s.push_str(f!(95)),
            Col::Cyan => self.s.push_str(f!(96)),
            Col::White => self.s.push_str(f!(97)),
        }
        self.last_col = col;
        self
    }

    pub fn bg(mut self, col: Col) -> Self {
        match col {
            Col::Black => self.s.push_str(f!(40)),
            Col::Red => self.s.push_str(f!(41)),
            Col::Green => self.s.push_str(f!(42)),
            Col::Yellow => self.s.push_str(f!(43)),
            Col::Blue => self.s.push_str(f!(44)),
            Col::Purple => self.s.push_str(f!(45)),
            Col::DarkCyan => self.s.push_str(f!(46)),
            Col::Grey => self.s.push_str(f!(47)),
            Col::DarkGrey => self.s.push_str(f!(100)),
            Col::BrightRed => self.s.push_str(f!(101)),
            Col::Lime => self.s.push_str(f!(102)),
            Col::BrightYellow => self.s.push_str(f!(103)),
            Col::BrightBlue => self.s.push_str(f!(104)),
            Col::Magenta => self.s.push_str(f!(105)),
            Col::Cyan => self.s.push_str(f!(106)),
            Col::White => self.s.push_str(f!(107)),
        }
        self.last_bg = col;
        self
    }

    pub fn revert_styles(self, fmt: Fmt, col: Col, bg: Col) -> Self {
        self.col(col).bg(bg).fmt(fmt)
    }

    pub fn fmt_for(self, fmt: Fmt, text: &str) -> Self {
        let f = self.last_fmt;
        let c = self.last_col;
        let b = self.last_bg;
        self.fmt(fmt).text(text).revert_styles(f, c, b)
    }

    pub fn fmt_for_ln(self, fmt: Fmt, text: &str) -> Self {
        self.fmt_for(fmt, text).ln()
    }

    pub fn col_for(self, col: Col, text: &str) -> Self {
        let f = self.last_fmt;
        let c = self.last_col;
        let b = self.last_bg;
        self.col(col).text(text).revert_styles(f, c, b)
    }

    pub fn col_for_ln(self, col: Col, text: &str) -> Self {
        self.col_for(col, text).ln()
    }

    pub fn bg_for(self, col: Col, text: &str) -> Self {
        let f = self.last_fmt;
        let c = self.last_col;
        let b = self.last_bg;
        self.bg(col).text(text).revert_styles(f, c, b)
    }

    pub fn bg_for_ln(self, col: Col, text: &str) -> Self {
        self.bg_for(col, text).ln()
    }

    pub fn all_for(self, fmt: Fmt, col: Col, bg: Col, text: &str) -> Self {
        let f = self.last_fmt;
        let c = self.last_col;
        let b = self.last_bg;
        self.col(col)
            .bg(bg)
            .fmt(fmt)
            .text(text)
            .revert_styles(f, c, b)
    }

    pub fn all_for_ln(self, fmt: Fmt, col: Col, bg: Col, text: &str) -> Self {
        self.all_for(fmt, col, bg, text).ln()
    }

    pub fn flush(self) {
        print!("{}", self.def().s);
        std::io::stdout().flush().unwrap();
    }
}

impl Display for Printer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}
