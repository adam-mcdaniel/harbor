use alloc::collections::BTreeMap;
use super::{error, why, why::*};
use core::fmt;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub what_parser);


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    VariableNotInScope(String),
    CallNonFunction(String),
    MismatchedTypes(Expr, Type, Type),
    SizeOfFunction(Type),
    DerefNonPointer(Expr, Type),

    ParseError(String),
    WhyError(why::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::VariableNotInScope(name) => write!(f, "\x1b[91merror: \x1b[m\x1b[0mvariable `{}` is used, but not in scope", name),
            Self::CallNonFunction(name) => write!(f, "\x1b[91merror: \x1b[m\x1b[0mcalled non-function `{}`", name),
            Self::MismatchedTypes(expr, expected, found) => write!(f, "\x1b[91merror: \x1b[m\x1b[0mmismatched types: expected `{}` but found `{}` in expression `{}`", expected, found, expr),
            Self::SizeOfFunction(t) => write!(f, "\x1b[91merror: \x1b[m\x1b[0mattempted to get the size of a function with signature `{}`: are you trying to assign functions to a value?", t),
            Self::DerefNonPointer(expr, t) => write!(f, "\x1b[91merror: \x1b[m\x1b[0mdereferenced non-pointer type `{}` in expression `{}`", t, expr),

            Self::ParseError(e) => write!(f, "\x1b[91merror: \x1b[m\x1b[0m\n{}", e),
            Self::WhyError(e) => write!(f, "{}", e)
        }
    }
}

pub fn parse(code: String) -> Result<Expr, Error> {
    let code = match comment::c::strip(&code) {
        Ok(s) => s,
        Err(_) => code
    };
    
    match what_parser::WhatParser::new().parse(&code) {
        Ok(parsed) => {
            Ok(parsed)
        },
        Err(e) => {
            Err(Error::ParseError(error::format_error(&code, e)))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Integer,
    Bool,
    Character,
    Void,

    Pointer(Box<Self>),

    // Tuple(Vec<Self>),
    Function(Vec<Self>, Box<Self>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer => write!(f, "int"),
            Self::Bool => write!(f, "bool"),
            Self::Character => write!(f, "char"),
            Self::Void => write!(f, "void"),
            Self::Pointer(inner) => write!(f, "&{}", inner),
            Self::Function(args, ret) => {
                write!(f, "(")?;
                for arg in args {
                    write!(f, "{}, ", arg)?;
                }
                write!(f, ") -> {}", ret)
            },
        }
    }
}

impl Type {
    fn get_size(&self) -> Result<u32, Error> {
        Ok(match self {
            Type::Integer
            | Type::Bool
            | Type::Character => 1,
            Type::Void => 0,
            Type::Pointer(_) => 1,
            Type::Function(_, _) => {
                return Err(Error::SizeOfFunction(self.clone()))
            }
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Integer(u32),
    Bool(bool),
    Character(char),

    Function(Vec<(String, Type)>, Type, Box<Self>),
    Let(String, Type, Box<Self>, Box<Self>),
    Assign(String, Box<Self>),
    
    Call(String, Vec<Self>),
    Variable(String),
    // Reference(String),
    // Dereference(String),
    // Tuple(Vec<Self>),
    // Nth(Box<Self>, usize),
    Refer(String),
    Deref(Box<Self>),
    DerefAssign(Box<Self>, Box<Self>),

    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),

    Or(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Not(Box<Self>),

    Getchar,
    Getnum,
    Putchar(Box<Self>),
    Putnum(Box<Self>),

    Free(Box<Self>),
    Alloc(Box<Self>, Type),

    Block(Vec<Self>),

    While(Box<Self>, Box<Self>),
    If(Box<Self>, Box<Self>),

    Eq(Box<Self>, Box<Self>),
    Neq(Box<Self>, Box<Self>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Bool(x) => write!(f, "{}", x),
            Self::Character(ch) => write!(f, "{:?}", ch),

            Self::Function(args, ret, body) => {
                write!(f, "fn(")?;
                for (name, t) in args {
                    write!(f, "{}: {}, ", name, t)?;
                }
                write!(f, ") -> {} = {}", ret, body)
            }

            Self::Let(name, t, val, ret) => {
                write!(f, "let {}: {} = {} in {}", name, t, val, ret)
            }

            Self::Assign(name, val) => 
                write!(f, "{} = {}", name, val),

            Self::Call(name, args) => {
                write!(f, "{}(", name)?;
                for arg in args {
                    write!(f, "{}, ", arg)?;
                }
                write!(f, ")")
            }

            Self::Variable(name) => write!(f, "{}", name),
            Self::Refer(name) => write!(f, "&{}", name),
            Self::Deref(val) => write!(f, "*{}", val),
            Self::DerefAssign(val, new_val) => write!(f, "*{} = {}", val, new_val),

            Self::Add(lhs, rhs) => write!(f, "{} + {}", lhs, rhs),
            Self::Sub(lhs, rhs) => write!(f, "{} - {}", lhs, rhs),
            Self::Mul(lhs, rhs) => write!(f, "({} * {})", lhs, rhs),
            Self::Div(lhs, rhs) => write!(f, "({} / {})", lhs, rhs),

            Self::And(lhs, rhs) => write!(f, "{} && {}", lhs, rhs),
            Self::Or(lhs, rhs) => write!(f, "{} || {}", lhs, rhs),
            Self::Not(x) => write!(f, "!{}", x),

            Self::Eq(lhs, rhs) => write!(f, "({} == {})", lhs, rhs),
            Self::Neq(lhs, rhs) => write!(f, "({} != {})", lhs, rhs),

            Self::Putchar(x) => write!(f, "putchar({})", x),
            Self::Putnum(x) => write!(f, "putnum({})", x),
            Self::Getchar => write!(f, "getchar()"),
            Self::Getnum => write!(f, "getnum()"),

            Self::Alloc(n, t) => write!(f, "alloc({}, {})", n, t),
            Self::Free(x) => write!(f, "free({})", x),

            Self::Block(block) => {
                write!(f, "do ")?;
                for expr in block {
                    write!(f, "{}; ", expr)?;
                }
                write!(f, "end")
            }
            Self::While(cond, code) => {
                write!(f, "while ({}) {}", cond, code)
            }
            Self::If(cond, code) => {
                write!(f, "if ({}) {}", cond, code)
            }
        }
    }
}

impl Expr {
    pub fn compile(&self, scope: &BTreeMap<String, Type>, offset: &mut u32) -> Result<Op, Error> {
        self.type_check(scope)?;
        Ok(match self {
            Self::Refer(name) => {
                Op::Macro(name.clone())
            }
            Self::Deref(value) => {
                let ptr_type = value.get_type(scope)?;
                if let Type::Pointer(t) = ptr_type {
                    Op::Do(vec![
                        value.compile(scope, offset)?,
                        Op::Load(t.get_size()?)
                    ])
                } else {
                    return Err(Error::DerefNonPointer(self.clone(), ptr_type));
                }
            }
            Self::DerefAssign(addr, value) => {
                Op::Do(vec![
                    value.compile(scope, offset)?,
                    addr.compile(scope, offset)?,
                    Op::Store(value.get_type(scope)?.get_size()?)
                ])
            }


            Self::Block(items) => {
                let mut ops = vec![];
                for (i, value) in items.iter().enumerate() {
                    let size = value.get_type(scope)?.get_size()?;
                    ops.push(value.compile(scope, offset)?);
                    if size > 0 && i < items.len() - 1 {
                        ops.push(Op::Stfree(size));
                    }
                }
                Op::Do(ops)
            }

            Self::Assign(name, expr) => {
                Op::Do(vec![
                    expr.compile(scope, offset)?,
                    Op::Macro(name.clone()),
                    Op::Store(scope.get(name).ok_or(Error::VariableNotInScope(name.clone()))?.get_size()?)
                ])
            }

            Self::If(item, expr) => {
                Op::If(vec![item.compile(scope, offset)?], vec![expr.compile(scope, offset)?])
            }

            Self::While(item, expr) => {
                Op::While(vec![item.compile(scope, offset)?], vec![expr.compile(scope, offset)?])
            }

            Self::Integer(i) => Op::PushLiteral(Literal(*i)),
            Self::Bool(b) => Op::PushLiteral(Literal(*b as u32)),
            Self::Character(ch) => Op::PushLiteral(Literal(*ch as u8 as u32)),

            Self::Putchar(x) => Op::Do(vec![
                x.compile(scope, offset)?,
                Op::Putchar
            ]),
            Self::Putnum(x) => Op::Do(vec![
                x.compile(scope, offset)?,
                Op::Putnum
            ]),
            Self::Alloc(x, t) => Op::Do(vec![
                x.compile(scope, offset)?,
                Op::PushLiteral(Literal(t.get_size()?)),
                Op::Mul,
                Op::Alloc
            ]),
            Self::Free(x) => Op::Do(vec![
                x.compile(scope, offset)?,
                Op::Free
            ]),
            Self::Getchar => Op::Getchar,
            Self::Getnum => Op::Getnum,

            Self::Function(args, ret, body) => {
                let mut scope = scope.clone();
                for (name, t) in args {
                    scope.insert(name.clone(), t.clone());
                }
                function(
                    args.iter().map(|(name, t)| Ok((name.clone(), t.get_size()?)))
                        .collect::<Result<Vec<_>, Error>>()?,
                    ret.get_size()?,
                    vec![body.compile(&scope, offset)?]
                )
            },

            Self::Add(a, b) => {
                Op::Do(vec![
                    a.compile(scope, offset)?,
                    b.compile(scope, offset)?,
                    Op::Add
                ])
            }

            Self::Sub(a, b) => {
                Op::Do(vec![
                    a.compile(scope, offset)?,
                    b.compile(scope, offset)?,
                    Op::Sub
                ])
            }

            Self::Mul(a, b) => {
                Op::Do(vec![
                    a.compile(scope, offset)?,
                    b.compile(scope, offset)?,
                    Op::Mul
                ])
            }

            Self::Div(a, b) => {
                Op::Do(vec![
                    a.compile(scope, offset)?,
                    b.compile(scope, offset)?,
                    Op::Div
                ])
            }

            Self::And(a, b) => {
                Op::Do(vec![
                    a.compile(scope, offset)?,
                    b.compile(scope, offset)?,
                    Op::And
                ])
            }

            Self::Or(a, b) => {
                Op::Do(vec![
                    a.compile(scope, offset)?,
                    b.compile(scope, offset)?,
                    Op::Or
                ])
            }

            Self::Not(x) => {
                Op::Do(vec![
                    x.compile(scope, offset)?,
                    Op::Not
                ])
            }

            Self::Eq(a, b) => {
                Op::Do(vec![
                    a.compile(scope, offset)?,
                    b.compile(scope, offset)?,
                    Op::Eq
                ])
            }

            Self::Neq(a, b) => {
                Op::Do(vec![
                    a.compile(scope, offset)?,
                    b.compile(scope, offset)?,
                    Op::Neq
                ])
            }

            Self::Variable(name) => {
                Op::Do(vec![
                    Op::Macro(name.clone()),
                    Op::Load(scope.get(name).ok_or(Error::VariableNotInScope(name.clone()))?.get_size()?)
                ])

            }

            Self::Let(name, t, expr, body) => {
                let mut scope = scope.clone();
                scope.insert(name.clone(), t.clone());
                if let Type::Function(args, _) = t {
                    let mut args_size = 0;
                    for arg in args {
                        args_size += arg.get_size()?;
                    }
                    *offset += args_size;
                    let expr_result = expr.compile(&scope, offset)?;
                    *offset -= args_size;

                    let result = Op::Let(name.clone(), vec![
                        expr_result
                    ], vec![
                        body.compile(&scope, offset)?,
                    ]);
                    result
                } else {
                    let this_offset = *offset;
                    let size = t.get_size()?;
                    *offset += size;
                    let expr_result = expr.compile(&scope, offset)?;
                    let body_result = body.compile(&scope, offset)?;
                    *offset -= size;
    
                    let result_type = body.get_type(&scope)?;
                    let result_size = result_type.get_size()?;
                    let result = Op::Let(name.clone(), vec![
                        Op::LoadFrom(FP, 1),
                        Op::PushLiteral(Literal(this_offset)),
                        Op::Add
                    ], vec![
                        // Allocate space on the stack to
                        // store the value
                        Op::Stalloc(size),
                        Op::Pop(TMP2),
                        // Store the value
                        expr_result,
                        Op::Macro(name.clone()),
                        Op::Store(size),

                        body_result,

                        Op::PushLiteral(Literal(result_size)),
                        Op::Alloc,
                        Op::Duplicate,
                        Op::StoreAt(R0, 1),
                        Op::Store(result_size),
                        Op::Stfree(size),

                        Op::LoadFrom(R0, 1),
                        Op::Load(result_size),
                        Op::LoadFrom(R0, 1),
                        Op::Free,
                        // Op::Decrement(SP)
                    ]);
                    result
                }

            }

            Self::Call(name, args) => {
                let mut result = vec![];
                for arg in args {
                    result.push(arg.compile(scope, offset)?);
                }
                result.push(Op::Macro(name.clone()));
                Op::Do(result)
            }
        })
    }

    fn type_check(&self, scope: &BTreeMap<String, Type>) -> Result<(), Error> {
        self.get_type(scope)?;
        Ok(())
    }

    fn get_type(&self, scope: &BTreeMap<String, Type>) -> Result<Type, Error> {
        Ok(match self {
            Self::Alloc(n, t) => {
                let count_type = n.get_type(scope)?;
                if count_type != Type::Integer {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Integer, count_type));
                }
                Type::Pointer(Box::new(t.clone()))
            }

            Self::Free(x) => {
                let x_type = x.get_type(scope)?;
                if let Type::Pointer(t) = x_type {
                    t.get_size()?;
                    Type::Void
                } else {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Pointer(Box::new(Type::Void)), x_type));
                }
            }

            Self::Refer(name) => {
                let var_type = scope.get(name).ok_or(Error::VariableNotInScope(name.clone()))?;
                var_type.get_size()?;

                Type::Pointer(Box::new(var_type.clone()))
            }
            Self::Deref(value) => {
                let ptr_type = value.get_type(scope)?;
                if let Type::Pointer(t) = ptr_type {
                    t.get_size()?;
                    *t.clone()
                } else {
                    return Err(Error::DerefNonPointer(self.clone(), ptr_type));
                }
            }
            Self::DerefAssign(addr, value) => {
                Self::Deref(addr.clone()).get_type(scope)?;
                let value_type = value.get_type(scope)?;
                let ptr_type = addr.get_type(scope)?;
                if let Type::Pointer(t) = ptr_type {
                    t.get_size()?;
                    if *t.clone() != value_type {
                        return Err(Error::MismatchedTypes(self.clone(), *t.clone(), value_type));
                    }
                    Type::Void
                } else {
                    return Err(Error::DerefNonPointer(self.clone(), ptr_type));
                }
            }

            Self::Block(items) => {
                let mut result = Type::Void; 
                for item in items {
                    result = item.get_type(scope)?
                }
                result
            }

            Self::If(cond, body) => {
                let cond_type = cond.get_type(scope)?;
                let body_type = body.get_type(scope)?;
                if cond_type != Type::Bool {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Bool, cond_type))
                } else if body_type != Type::Void {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Void, body_type))
                } else {
                    Type::Void
                }
            }

            Self::While(cond, body) => {
                let cond_type = cond.get_type(scope)?;
                let body_type = body.get_type(scope)?;
                if cond_type != Type::Bool {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Bool, cond_type))
                } else if body_type != Type::Void {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Void, body_type))
                } else {
                    Type::Void
                }
            }

            Self::Integer(_) => Type::Integer,
            Self::Bool(_) => Type::Bool,
            Self::Character(_) => Type::Character,
            
            Self::Function(args, ret, expr) => {
                let mut test_scope = scope.clone();
                for (name, t) in args {
                    test_scope.insert(name.clone(), t.clone());
                }
                let expr_type = expr.get_type(&test_scope)?;
                if expr_type != *ret {
                    return Err(Error::MismatchedTypes(
                        self.clone(),
                        ret.clone(),
                        expr_type,
                    ));
                }

                Type::Function(
                    args.iter().map(|(_, t)| t.clone()).collect::<Vec<_>>(),
                    Box::new(ret.clone())
                )
            },
            Self::Variable(name) => {
                if let Some(t) = scope.get(name) {
                    t.clone()
                } else {
                    return Err(Error::VariableNotInScope(name.clone()));
                }
            },
            Self::Call(name, args) => {
                if let Some(t) = scope.get(name) {
                    match t {
                        Type::Function(params, ret) => {
                            if params.len() != args.len() {
                                return Err(Error::MismatchedTypes(self.clone(), t.clone(), Type::Void));
                            }
                            for (a, b) in params.iter().zip(args.iter()) {
                                let b_type = b.get_type(scope)?;
                                if a != &b_type {
                                    return Err(Error::MismatchedTypes(self.clone(), a.clone(), b_type));
                                }
                            }

                            *ret.clone()
                        },
                        _ => return Err(Error::CallNonFunction(name.clone())),
                    }
                } else {
                    return Err(Error::VariableNotInScope(name.clone()));
                }
            },
            Self::Let(name, t, val, expr) => {
                let mut scope = scope.clone();
                scope.insert(name.clone(), t.clone());
                let val_type = val.get_type(&scope)?;
                if &val_type != t {
                    return Err(Error::MismatchedTypes(self.clone(), t.clone(), val_type));
                }
                expr.get_type(&scope)?
            }
            Self::Assign(name, expr) => {
                let var_type = scope.get(name).ok_or(Error::VariableNotInScope(name.clone()))?;
                // let mut scope = scope.clone();
                // scope.insert(name.clone(), t.clone());
                let expr_type = expr.get_type(&scope)?;
                if var_type != &expr_type {
                    return Err(Error::MismatchedTypes(self.clone(), var_type.clone(), expr_type));
                }
                Type::Void
            }

            Self::Add(a, b) => {
                match (a.get_type(scope)?, b.get_type(scope)?) {
                    (Type::Integer, Type::Integer) => Type::Integer,
                    (Type::Pointer(x), Type::Integer) => Type::Pointer(x),
                    (a, b) => return Err(Error::MismatchedTypes(self.clone(), a, b)),
                }
            }

            Self::Sub(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b) => {
                let a_type = a.get_type(scope)?;
                let b_type = b.get_type(scope)?;
                if a_type != Type::Integer {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Integer, a_type));
                }
                if b_type != Type::Integer {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Integer, b_type));
                }
                Type::Integer
            }

            Self::And(a, b)
            | Self::Or(a, b) => {
                let a_type = a.get_type(scope)?;
                let b_type = b.get_type(scope)?;
                if a_type != Type::Bool {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Bool, a_type));
                }
                if b_type != Type::Bool {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Bool, b_type));
                }
                Type::Bool
            }

            Self::Not(x) => {
                let x_type = x.get_type(scope)?;
                if x_type != Type::Bool {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Bool, x_type));
                }
                Type::Bool
            }

            Self::Putchar(x) => {
                let x_type = x.get_type(scope)?;
                if x_type != Type::Character {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Character, x_type));
                }
                Type::Void
            }

            Self::Putnum(x) => {
                let x_type = x.get_type(scope)?;
                if x_type != Type::Integer {
                    return Err(Error::MismatchedTypes(self.clone(), Type::Integer, x_type));
                }
                Type::Void
            }

            Self::Getnum => Type::Integer,
            Self::Getchar => Type::Character,

            Self::Eq(a, b) | Self::Neq(a, b) => {
                let a_type = a.get_type(scope)?;
                let b_type = b.get_type(scope)?;
                if a_type != b_type {
                    return Err(Error::MismatchedTypes(self.clone(), a_type, b_type));
                }
                Type::Bool
            }

        })
    }
}