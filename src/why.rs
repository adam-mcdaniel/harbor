use alloc::collections::BTreeMap;
use core::fmt;
use super::{error, how::Program};

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub why_parser);


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(pub u32);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Literal(pub u32);

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Location {
    Address(Address),
    Deref(Box<Self>),
    Offset(Box<Self>, i32),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    MacroNotDefined(String),
    CannotGetRuntimeAddress(Location),
    
    ParseError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[91merror: \x1b[m\x1b[0m")?;
        match self {
            Error::MacroNotDefined(name) => write!(f, "macro '{}' not defined", name),
            Error::CannotGetRuntimeAddress(location) => write!(f, "cannot get runtime address of {}", location),
            Error::ParseError(msg) => write!(f, "\n{}", msg),
        }
    }
}

pub fn parse(code: impl ToString) -> Result<Op, Error> {
    let code = code.to_string();
    match why_parser::WhyParser::new().parse(&code) {
        Ok(parsed) => {
            Ok(parsed)
        },
        Err(e) => {
            Err(Error::ParseError(error::format_error(&code, e)))
        }
    }
}


pub fn function(mut args: Vec<(String, u32)>, ret_size: u32, code: Vec<Op>) -> Op {
    let mut offset = 0;
    let mut args_size = 0;
    let mut result = Op::Do(code);
    for (_, size) in &args {
        args_size += size;
    }
    
    while let Some((name, size)) = args.pop() {
        offset += size;
        result = Op::Let(name, vec![
            Op::LoadFrom(FP, 1),
            Op::Increment(SP.deref(), args_size - offset),
        ], vec![result]);
    }
    
    Op::Frame(args_size, ret_size, vec![result])
}

impl Location {
    pub fn get_address(&self) -> Result<Address, Error> {
        match self {
            Self::Address(address) => Ok(*address),
            Self::Offset(inner, offset) => {
                match inner.get_address() {
                    Ok(address) => Ok(Address((address.0 as i32 + offset) as u32)),
                    Err(error) => Err(error),
                }
            }
            Self::Deref(_) => Err(Error::CannotGetRuntimeAddress(self.clone())),
        }
    }

    pub fn deref(&self) -> Self {
        Self::Deref(Box::new(self.clone()))
    }

    pub fn offset(&self, count: i32) -> Self {
        Self::Offset(Box::new(self.clone()), count)
    }

    pub fn to(&self, program: &mut Program) {
        match self {
            Self::Deref(value) => {
                value.to(program);
                program.deref();
            },

            Self::Address(Address(address)) => {
                program.right(*address);
            }

            Self::Offset(value, count) => {
                value.to(program);
                program.shift(*count);
            }
        }
    }

    pub fn from(&self, program: &mut Program) {
        match self {
            Self::Deref(value) => {
                program.refer();
                value.from(program);
            },

            Self::Address(Address(address)) => {
                program.left(*address);
            }

            Self::Offset(value, count) => {
                program.shift(-*count);
                value.from(program);
            }
        }
    }

    pub fn zero(&self, program: &mut Program) {
        self.to(program);
        program.begin_loop();
        program.minus(1);
        program.end_loop();
        self.from(program);
    }

    pub fn plus(&self, n: u32, program: &mut Program) {
        self.to(program);
        program.plus(n);
        self.from(program);
    }

    pub fn inc(&self, program: &mut Program) {
        self.to(program);
        program.plus(1);
        self.from(program);
    }

    pub fn minus(&self, n: u32, program: &mut Program) {
        self.to(program);
        program.minus(n);
        self.from(program);
    }

    pub fn dec(&self, program: &mut Program) {
        self.to(program);
        program.minus(1);
        self.from(program);
    }

    pub fn set(&self, value: u32, program: &mut Program) {
        self.zero(program);
        self.plus(value, program);
    }

    pub fn alloc(&self, program: &mut Program) {
        self.to(program);
        program.alloc();
        self.from(program);
    }

    pub fn free(&self, program: &mut Program) {
        self.to(program);
        program.free();
        program.zero();
        self.from(program);
    }

    pub fn put(&self, program: &mut Program) {
        self.to(program);
        program.put();
        self.from(program);
    }

    pub fn get(&self, program: &mut Program) {
        self.to(program);
        program.get();
        self.from(program);
    }

    pub fn putnum(&self, program: &mut Program) {
        self.to(program);
        program.putnum();
        self.from(program);
    }

    pub fn getnum(&self, program: &mut Program) {
        self.to(program);
        program.getnum();
        self.from(program);
    }


    pub fn begin_loop(&self, program: &mut Program) {
        self.to(program);
        program.begin_loop();
        self.from(program);
    }
    
    pub fn end_loop(&self, program: &mut Program) {
        self.to(program);
        program.end_loop();
        self.from(program);
    }

    pub fn push(&self, program: &mut Program) {
        SP.inc(program);
        copy_cell(
            SP.deref(),
            self.clone(),
            program
        );
    }

    pub fn pop_into(&self, program: &mut Program) {
        copy_cell(
            self.clone(),
            SP.deref(),
            program
        );
        // SP.deref().zero(program);
        SP.dec(program);
    }

    pub fn load_ptr(&self, size: u32, program: &mut Program) {
        for i in 0..size {
            self.offset(i as i32).push(program);
        }
    }

    pub fn store_ptr(&self, size: u32, program: &mut Program) {
        for i in 0..size {
            self.offset((size - i - 1) as i32).pop_into(program);
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Address(loc) => if self == &SP {
                write!(f, "SP")
            } else if self == &FP {
                write!(f, "FP")

            } else if self == &TMP0 {
                write!(f, "TMP0")
            } else if self == &TMP1 {
                write!(f, "TMP1")
            } else if self == &TMP2 {
                write!(f, "TMP2")
            } else if self == &TMP3 {
                write!(f, "TMP3")
            } else if self == &TMP4 {
                write!(f, "TMP4")
            } else if self == &TMP5 {
                write!(f, "TMP5")
                
            } else if self == &R0 {
                write!(f, "R0")
            } else if self == &R1 {
                write!(f, "R1")
            } else if self == &R2 {
                write!(f, "R2")
            } else if self == &R3 {
                write!(f, "R3")
            } else if self == &R4 {
                write!(f, "R4")
            } else if self == &R5 {
                write!(f, "R5")

            } else {
                write!(f, "{}", loc.0)
            },
            Self::Offset(inner, offset) => write!(f, "{} {} +", inner, offset),
            Self::Deref(inner) => write!(f, "{}@", inner),
        }
    }
}

pub const TOTAL_REGISTERS: u32 = 14;


/// Stack pointer
pub const SP: Location = Location::Address(Address(0));

// Frame pointer
pub const FP: Location = Location::Address(Address(3));

/// Builtin registers for assembler use only
pub const TMP0: Location = Location::Address(Address(1));
pub const TMP1: Location = Location::Address(Address(2));
pub const TMP2: Location = Location::Address(Address(4));
pub const TMP3: Location = Location::Address(Address(5));
pub const TMP4: Location = Location::Address(Address(6));
pub const TMP5: Location = Location::Address(Address(7));

/// General purpose registers
pub const R0: Location = Location::Address(Address(8));
pub const R1: Location = Location::Address(Address(9));
pub const R2: Location = Location::Address(Address(10));
pub const R3: Location = Location::Address(Address(11));
pub const R4: Location = Location::Address(Address(12));
pub const R5: Location = Location::Address(Address(13));

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    Let(String, Vec<Self>, Vec<Self>),
    Macro(String),

    Frame(u32, u32, Vec<Self>),
    Do(Vec<Self>),


    /// Pop an address and store a value at that address
    Set(Literal),

    /// Store a number of cells at an address
    StoreAt(Location, u32),
    
    /// Load a number of cells from an address
    LoadFrom(Location, u32),

    /// Pop an address and store a number of cells at that address
    Store(u32),
    /// Pop an address and load cells at that address
    Load(u32),

    /// Pop a size off the stack, and allocate that number of cells
    Alloc,
    /// Pop an address off the stack, and free that number of cells
    Free,
    /// Duplicate the top cell on the stack
    Duplicate,

    /// Pop a cell and print it
    Putchar,

    /// Get a character from STDIN and push it
    Getchar,

    /// Pop a cell and print it as a number
    Putnum,
    
    /// Get a number from STDIN and push it
    Getnum,

    /// While loop
    While(Vec<Self>, Vec<Self>),

    /// If statement
    If(Vec<Self>, Vec<Self>),

    Increment(Location, u32),
    Decrement(Location, u32),

    /// Pop a value and push its logical not
    Not,

    /// Pop two cells and push their logical or
    Or,

    /// Pop two cells and push their logical and
    And,

    /// Pop two cells and push their sum
    Add,

    /// Pop two cells and push their difference
    Sub,

    /// Pop two cells and push their product
    Mul,

    /// Pop two cells and push their quotient
    Div,

    /// Pop two cells and push their equality
    Eq,

    /// Pop two cells and push their inequality
    Neq,

    /// Pop a value from the stack into an address
    Pop(Location),

    /// Push a literal onto the stack
    PushLiteral(Literal),
    /// Push an address onto the stack
    PushAddress(Address),
    /// Push a value onto the stack
    Push(Location),

    /// Allocate a number of cells on the stack
    Stalloc(u32),
    /// Pop a number of cells off the stack
    Stfree(u32),
}

pub fn copy_cell(x: Location, y: Location, program: &mut Program) {
    TMP0.zero(program);
    x.zero(program);
    
    y.begin_loop(program);
    x.inc(program);
    TMP0.inc(program);
    y.dec(program);
    y.end_loop(program);
    
    TMP0.begin_loop(program);    
    y.inc(program);
    TMP0.dec(program);
    TMP0.end_loop(program);
}

impl Op {
    pub fn assemble(&self, program: &mut Program) -> Result<(), Error> {
        self.assemble_with_scope(&BTreeMap::new(), program)
    }

    pub fn assemble_with_scope(&self, scope: &BTreeMap<String, Vec<Self>>, program: &mut Program) -> Result<(), Error> {
        match self {
            Self::Do(code) => {
                for op in code {
                    op.assemble_with_scope(&scope, program)?;
                }
            }
            Self::Let(name, val, ret) => {
                let mut new_scope = scope.clone();
                new_scope.insert(name.clone(), val.clone());
                
                for op in ret {
                    op.assemble_with_scope(&new_scope, program)?;
                }
            }

            Self::Macro(name) => {
                if let Some(code) = scope.get(name) {
                    let mut new_scope = scope.clone();
                    new_scope.remove(name);
                    for op in code {
                        op.assemble_with_scope(&new_scope, program)?;
                    }
                } else {
                    return Err(Error::MacroNotDefined(name.clone()));
                }
            }

            Self::Frame(args_size, ret_size, code) => {
                // Allocate some space on the heap and temporarily spill the arguments there

                if *args_size > 0 {
                    Self::PushLiteral(Literal(*args_size)).assemble_with_scope(scope, program)?;
                    Self::Alloc.assemble_with_scope(scope, program)?;
                    Self::Duplicate.assemble_with_scope(scope, program)?;
                    TMP5.pop_into(program);
                    Self::Store(*args_size).assemble_with_scope(scope, program)?;
                }


                // push old frame pointer
                FP.push(program);
                copy_cell(
                    FP,
                    SP,
                    program
                );
                // Increment it so it points to the first argument
                FP.inc(program);

                if *args_size > 0 {
                    // Load the arguments from the heap
                    TMP5.push(program);
                    Self::Load(*args_size).assemble_with_scope(scope, program)?;
                    // Free the memory we allocated to store them temporarily
                    TMP5.push(program);
                    Self::Free.assemble_with_scope(scope, program)?;
                }
                // Run the code in the new frame
                for op in code {
                    op.assemble_with_scope(scope, program)?;
                }

                if *ret_size > 0 {
                    // Spill the return value to some memory on the heap
                    Self::PushLiteral(Literal(*ret_size)).assemble_with_scope(scope, program)?;
                    Self::Alloc.assemble_with_scope(scope, program)?;
                    Self::Duplicate.assemble_with_scope(scope, program)?;
                    TMP5.pop_into(program);
                    Self::Store(*ret_size).assemble_with_scope(scope, program)?;
                }
                if *args_size > 0 {
                    // Remove the arguments from the stack
                    Self::Stfree(*args_size).assemble_with_scope(scope, program)?;
                }
                // Restore the frame pointer
                FP.pop_into(program);

                if *ret_size > 0 {
                    // Load the return value from the heap, and free the memory
                    TMP5.push(program);
                    Self::Load(*ret_size).assemble_with_scope(scope, program)?;
                    TMP5.push(program);
                    Self::Free.assemble_with_scope(scope, program)?;
                }
                TMP5.zero(program);
            }

            Self::Pop(loc) => {
                loc.pop_into(program);
            },

            Self::PushLiteral(Literal(n)) | Self::PushAddress(Address(n)) => {
                SP.inc(program);
                SP.deref().set(*n, program);
            },

            Self::Push(loc) => {
                loc.push(program);
            },

            Self::Load(size) => {
                TMP2.pop_into(program);
                TMP2.deref().load_ptr(*size, program);
            },

            Self::Store(size) => {
                TMP2.pop_into(program);
                TMP2.deref().store_ptr(*size, program);
            },

            Self::LoadFrom(loc, size) => {
                loc.load_ptr(*size, program);
            },

            Self::StoreAt(loc, size) => {
                loc.store_ptr(*size, program);
            },

            Self::Stalloc(size) => {
                if *size > 0 {
                    SP.plus(*size, program);
                }
            }

            Self::Stfree(size) => {
                if *size > 0 {
                    SP.minus(*size, program);
                }
            }

            Self::Alloc => {
                SP.deref().alloc(program);
            }

            Self::Free => {
                SP.deref().free(program);
                SP.dec(program);
            }

            Self::Duplicate => {
                TMP2.pop_into(program);
                TMP2.push(program);
                TMP2.push(program);
            }

            Self::Decrement(loc, n) => {
                loc.minus(*n, program)
            }

            Self::Increment(loc, n) => {
                loc.plus(*n, program)
            }

            Self::Not => {
                let x = TMP2;
                x.pop_into(program);

                TMP0.zero(program);
                x.begin_loop(program);
                TMP0.inc(program);
                x.zero(program);
                x.end_loop(program);
                x.inc(program);

                TMP0.begin_loop(program);
                x.dec(program);
                TMP0.dec(program);
                TMP0.end_loop(program);

                x.push(program);
            }

            Self::Add | Self::Or => {
                let x = TMP3;
                let y = TMP2;
                y.pop_into(program);
                x.pop_into(program);
                
                // do addition
                TMP0.zero(program);
                y.begin_loop(program);
                x.inc(program);
                TMP0.inc(program);
                y.dec(program);
                y.end_loop(program);

                TMP0.begin_loop(program);
                y.inc(program);
                TMP0.dec(program);
                TMP0.end_loop(program);

                x.push(program);
            }

            Self::Sub => {
                let x = TMP2;
                let y = TMP3;
                y.pop_into(program);
                x.pop_into(program);
                
                // do subtraction
                TMP0.zero(program);
                y.begin_loop(program);
                x.dec(program);
                TMP0.inc(program);
                y.dec(program);
                y.end_loop(program);

                TMP0.begin_loop(program);
                y.inc(program);
                TMP0.dec(program);
                TMP0.end_loop(program);

                x.push(program);
            }

            Self::Mul | Self::And => {
                let x = TMP3;
                let y = TMP2;
                y.pop_into(program);
                x.pop_into(program);
                
                // do multiplication
                TMP0.zero(program);
                TMP1.zero(program);
                
                x.begin_loop(program);
                TMP1.inc(program);
                x.dec(program);
                x.end_loop(program);

                TMP1.begin_loop(program);
                y.begin_loop(program);
                x.inc(program);
                TMP0.inc(program);
                y.dec(program);
                y.end_loop(program);

                TMP0.begin_loop(program);
                y.inc(program);
                TMP0.dec(program);
                TMP0.end_loop(program);

                TMP1.dec(program);
                TMP1.end_loop(program);

                x.push(program);
            }

            Self::Div => {
                let x = TMP4;
                let y = TMP5;
                y.pop_into(program);
                x.pop_into(program);

                // do division
                TMP0.zero(program);
                TMP1.zero(program);
                TMP2.zero(program);
                TMP3.zero(program);


                x.begin_loop(program);
                TMP0.inc(program);
                x.dec(program);
                x.end_loop(program);

                TMP0.begin_loop(program);

                y.begin_loop(program);
                TMP1.inc(program);
                TMP2.inc(program);
                y.dec(program);
                y.end_loop(program);

                TMP2.begin_loop(program);
                y.inc(program);
                TMP2.dec(program);
                TMP2.end_loop(program);

                TMP1.begin_loop(program);

                TMP2.inc(program);
                TMP0.dec(program);
                TMP0.begin_loop(program);
                TMP2.zero(program);
                TMP3.inc(program);
                TMP0.dec(program);
                TMP0.end_loop(program);
                
                TMP3.begin_loop(program);
                TMP0.inc(program);
                TMP3.dec(program);
                TMP3.end_loop(program);

                TMP2.begin_loop(program);
                TMP1.dec(program);
                TMP1.begin_loop(program);
                x.dec(program);
                TMP1.zero(program);
                TMP1.end_loop(program);
                TMP1.inc(program);

                TMP2.dec(program);
                TMP2.end_loop(program);
                TMP1.dec(program);

                TMP1.end_loop(program);
                x.inc(program);
                TMP0.end_loop(program);

                x.push(program);
            }

            Self::If(cond, body) => {
                let x = TMP2;
                for op in cond {
                    op.assemble_with_scope(scope, program)?;
                }
                x.pop_into(program);
                x.begin_loop(program);
                for op in body {
                    op.assemble_with_scope(scope, program)?;
                }
                x.zero(program);
                x.end_loop(program);
            }

            Self::While(cond, body) => {
                for op in cond {
                    op.assemble_with_scope(scope, program)?;
                }
                TMP2.pop_into(program);
                TMP2.begin_loop(program);
                for op in body {
                    op.assemble_with_scope(scope, program)?;
                }
                for op in cond {
                    op.assemble_with_scope(scope, program)?;
                }
                TMP2.pop_into(program);
                TMP2.end_loop(program);
            }

            Self::Eq => {
                let x = TMP3;
                let y = TMP2;
                y.pop_into(program);
                x.pop_into(program);

                TMP0.zero(program);
                TMP1.zero(program);

                x.begin_loop(program);
                TMP1.inc(program);
                x.dec(program);
                x.end_loop(program);
                x.inc(program);

                y.begin_loop(program);
                TMP1.dec(program);
                TMP0.inc(program);
                y.dec(program);
                y.end_loop(program);

                TMP0.begin_loop(program);
                y.inc(program);
                TMP0.dec(program);
                TMP0.end_loop(program);

                TMP1.begin_loop(program);
                x.dec(program);
                TMP1.zero(program);
                TMP1.end_loop(program);

                x.push(program);
            }
            
            Self::Neq => {
                Self::Eq.assemble_with_scope(scope, program)?;
                Self::Not.assemble_with_scope(scope, program)?;
                // Self::Sub.assemble_with_scope(scope, program)?;
            }

            Self::Putnum => {
                SP.deref().putnum(program);
                // SP.deref().zero(program);
                SP.dec(program);
            }
            Self::Getnum => {
                // TMP2.getnum(program);
                // TMP2.push(program);
                SP.deref().offset(1).getnum(program);
                SP.inc(program);
            }

            Self::Putchar => {
                // SP.deref().zero(program);
                SP.deref().put(program);
                SP.dec(program);
            }
            Self::Getchar => {
                // TMP2.get(program);
                // TMP2.push(program);
                SP.deref().offset(1).get(program);
                SP.inc(program);
            }

            _ => unimplemented!()
        }
        Ok(())
    }
}