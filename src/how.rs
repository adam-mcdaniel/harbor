
use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    Comment(char),

    Plus(u32),
    Minus(u32),
    Left(u32),
    Right(u32),
    Loop,
    End,

    Put,
    Get,
    Putnum,
    Getnum,

    Refer,
    Deref,

    Alloc,
    Free
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Op::Comment(c) => write!(f, "{}", c),

            Op::Plus(n) if n > 1 => write!(f, "+({})", n),
            Op::Plus(n) if n == 1 => write!(f, "+"),
            Op::Plus(_) => {Ok(())},

            Op::Minus(n) if n > 1 => write!(f, "-({})", n),
            Op::Minus(n) if n == 1 => write!(f, "-"),
            Op::Minus(_) => {Ok(())},

            Op::Left(n) if n > 1 => write!(f, "<({})", n),
            Op::Left(_) => write!(f, "<"),

            Op::Right(n) if n > 1 => write!(f, ">({})", n),
            Op::Right(_) => write!(f, ">"),

            Op::Loop => write!(f, "["),
            Op::End => write!(f, "]"),
            
            Op::Get => write!(f, ","),
            Op::Put => write!(f, "."),
            Op::Getnum => write!(f, "#"),
            Op::Putnum => write!(f, "$"),
            
            Op::Refer => write!(f, "&"),
            Op::Deref => write!(f, "*"),
            
            Op::Alloc => write!(f, "?"),
            Op::Free => write!(f, "!"),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Program(pub Vec<Op>);

impl Program {

    pub fn comment(&mut self, c: &str) {
        self.0.push(Op::Comment('\n'));
        for ch in c.chars() {
            self.0.push(Op::Comment(ch));
        }
        self.0.push(Op::Comment('\n'));
    }

    pub fn plus(&mut self, n: u32) {
        self.0.push(Op::Plus(n));
    }

    pub fn minus(&mut self, n: u32) {
        self.0.push(Op::Minus(n));
    }

    pub fn shift(&mut self, n: i32) {
        if n > 0 {
            self.0.push(Op::Right(n as u32));
        } else if n < 0 {
            self.0.push(Op::Left(-n as u32));
        }
    }

    pub fn left(&mut self, n: u32) {
        self.0.push(Op::Left(n));
    }

    pub fn right(&mut self, n: u32) {
        self.0.push(Op::Right(n));
    }

    pub fn begin_loop(&mut self) {
        self.0.push(Op::Loop);
    }

    pub fn end_loop(&mut self) {
        self.0.push(Op::End);
    }

    pub fn refer(&mut self) {
        self.0.push(Op::Refer);
    }

    pub fn deref(&mut self) {
        self.0.push(Op::Deref);
    }

    pub fn alloc(&mut self) {
        self.0.push(Op::Alloc);
    }

    pub fn free(&mut self) {
        self.0.push(Op::Free);
    }

    pub fn put(&mut self) {
        self.0.push(Op::Put);
    }

    pub fn putnum(&mut self) {
        self.0.push(Op::Putnum);
    }

    pub fn get(&mut self) {
        self.0.push(Op::Get);
    }

    pub fn getnum(&mut self) {
        self.0.push(Op::Getnum);
    }

    pub fn assemble(&self) -> String {
        let mut result = String::new();
        for op in &self.0 {
            match *op {
                Op::Comment(c) => result.push(c),
                Op::Plus(n) => result += &"+".repeat(n as usize),
                Op::Minus(n) => result += &"-".repeat(n as usize),
                Op::Left(n) => result += &"<".repeat(n as usize),
                Op::Right(n) => result += &">".repeat(n as usize),
                Op::Loop => result += "[",
                Op::End => result += "]",

                Op::Get => result += ",",
                Op::Put => result += ".",
                Op::Getnum => result += "#",
                Op::Putnum => result += "$",

                Op::Refer => result += "&",
                Op::Deref => result += "*",

                Op::Alloc => result += "?",
                Op::Free => result += "!",
            }
        }
        result
    }
}

impl From<&str> for Program {
    fn from(s: &str) -> Self {
        let mut result = vec![];
        for ch in s.chars() {
            match ch {
                '+' => result.push(Op::Plus(1)),
                '-' => result.push(Op::Minus(1)),
                '<' => result.push(Op::Left(1)),
                '>' => result.push(Op::Right(1)),
                '[' => result.push(Op::Loop),
                ']' => result.push(Op::End),
                ',' => result.push(Op::Get),
                '.' => result.push(Op::Put),
                '#' => result.push(Op::Getnum),
                '$' => result.push(Op::Putnum),
                '*' => result.push(Op::Deref),
                '&' => result.push(Op::Refer),
                '?' => result.push(Op::Alloc),
                '!' => result.push(Op::Free),
                _ => {}
            }
        }

        Self(result)
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.assemble())
    }
}