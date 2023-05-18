use std::io::Write;
use crate::print::Col::Black;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Fmt {
    Default,
    Bold,
    Underline,
    UnderlineStop
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
    White
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
    last_fmt: Fmt
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

    pub fn textln(mut self, text: &str) -> Self {
        let f = self.last_fmt;
        let c = self.last_col;
        let g = self.last_bg;
        self.text(text).def().ln().fmt(f).col(c).bg(g)
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
            Fmt::Default =>         {self.s.push_str(f!(0))}
            Fmt::Bold =>            {self.s.push_str(f!(1))}
            Fmt::Underline =>       {self.s.push_str(f!(4))}
            Fmt::UnderlineStop =>   {self.s.push_str(f!(24))}
        }
        self.last_fmt = fmt;
        self
    }

    pub fn def(mut self) -> Self {
        self.last_fmt = Fmt::Default;
        self.fmt(Fmt::Default)
    }

    pub fn col(mut self, col: Col) -> Self {
        match col {
            Col::Black =>           {self.s.push_str(f!(30))}
            Col::Red =>             {self.s.push_str(f!(31))}
            Col::Green =>           {self.s.push_str(f!(32))}
            Col::Yellow =>          {self.s.push_str(f!(33))}
            Col::Blue =>            {self.s.push_str(f!(34))}
            Col::Purple =>          {self.s.push_str(f!(35))}
            Col::DarkCyan =>        {self.s.push_str(f!(36))}
            Col::Grey =>            {self.s.push_str(f!(37))}
            Col::DarkGrey =>        {self.s.push_str(f!(90))}
            Col::BrightRed =>       {self.s.push_str(f!(91))}
            Col::Lime =>            {self.s.push_str(f!(92))}
            Col::BrightYellow =>    {self.s.push_str(f!(93))}
            Col::BrightBlue =>      {self.s.push_str(f!(94))}
            Col::Magenta =>         {self.s.push_str(f!(95))}
            Col::Cyan =>            {self.s.push_str(f!(96))}
            Col::White =>           {self.s.push_str(f!(97))}
        }
        self.last_col = col;
        self
    }

    pub fn bg(mut self, col: Col) -> Self {
        match col {
            Col::Black =>           {self.s.push_str(f!(40))}
            Col::Red =>             {self.s.push_str(f!(41))}
            Col::Green =>           {self.s.push_str(f!(42))}
            Col::Yellow =>          {self.s.push_str(f!(43))}
            Col::Blue =>            {self.s.push_str(f!(44))}
            Col::Purple =>          {self.s.push_str(f!(45))}
            Col::DarkCyan =>        {self.s.push_str(f!(46))}
            Col::Grey =>            {self.s.push_str(f!(47))}
            Col::DarkGrey =>        {self.s.push_str(f!(100))}
            Col::BrightRed =>       {self.s.push_str(f!(101))}
            Col::Lime =>            {self.s.push_str(f!(102))}
            Col::BrightYellow =>    {self.s.push_str(f!(103))}
            Col::BrightBlue =>      {self.s.push_str(f!(104))}
            Col::Magenta =>         {self.s.push_str(f!(105))}
            Col::Cyan =>            {self.s.push_str(f!(106))}
            Col::White =>           {self.s.push_str(f!(107))}
        }
        self.last_bg = col;
        self
    }

    pub fn flush(mut self) {
        print!("{}", self.def().s);
        std::io::stdout().flush();
        print!(" ");
    }
}

impl ToString for Printer {
    fn to_string(&self) -> String {
        self.s.to_string()
    }
}