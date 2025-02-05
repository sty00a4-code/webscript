use std::{fmt::Debug, ops::Range};

#[derive(Clone)]
pub struct Located<T: Debug + Clone> {
    pub value: T,
    pub loc: Range<usize>,
}
impl<T: Debug + Clone> Located<T> {
    pub fn new(value: T, loc: Range<usize>) -> Self {
        Self { value, loc }
    }
}
impl<T: Debug + Clone> Debug for Located<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct Module(pub Vec<Located<Definition>>);
#[derive(Debug, Clone)]
pub enum Definition {
    Fn {
        export: bool,
        name: Located<String>,
        params: Vec<Located<Parameter>>,
        result: Option<Located<Type>>,
        body: Located<Body>,
    },
}
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: Located<String>,
    pub typ: Located<Type>,
}

#[derive(Debug, Clone)]
pub struct Body(pub Vec<Located<Statement>>);

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Vec<Located<Self>>),
    Let {
        param: Located<Parameter>,
        expr: Located<Expression>,
    },
    Assign {
        op: AssignOperator,
        path: Located<Path>,
        expr: Located<Expression>,
    },
    Call {
        path: Located<Path>,
        args: Vec<Located<Expression>>,
    },
    If {
        cond: Located<Expression>,
        case: Box<Located<Self>>,
        else_case: Option<Box<Located<Self>>>,
    },
    IfSome {
        param: Located<Parameter>,
        expr: Located<Expression>,
        case: Box<Located<Self>>,
        else_case: Option<Box<Located<Self>>>,
    },
    While {
        cond: Located<Expression>,
        body: Box<Located<Self>>,
    },
    WhileSome {
        param: Located<Parameter>,
        expr: Located<Expression>,
        body: Box<Located<Self>>,
    },
    For {
        param: Located<Parameter>,
        iter: Located<Expression>,
        body: Box<Located<Self>>,
    },
    Break,
    Continue,
    Return(Located<Expression>),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssignOperator {
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Exponent,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Atom(Atom),
    Binary {
        op: BinaryOperator,
        left: Box<Located<Self>>,
        right: Box<Located<Self>>,
    },
    Unary {
        op: UnaryOperator,
        right: Box<Located<Self>>,
    },
    Call {
        head: Box<Located<Self>>,
        args: Vec<Located<Expression>>,
    },
    Field {
        head: Box<Located<Self>>,
        field: Located<String>,
    },
    Index {
        head: Box<Located<Self>>,
        index: Box<Located<Self>>,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperator {
    Then,
    And,
    Or,
    EqualEqual,
    ExclamationEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Exponent,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnaryOperator {
    Minus,
    Not,
}
#[derive(Debug, Clone)]
pub enum Atom {
    Null,
    Ident(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
    Expression(Box<Located<Expression>>),
    Tuple(Vec<Located<Expression>>),
    Vector(Vec<Located<Expression>>),
    Object(Vec<(Located<String>, Located<Expression>)>),
}
#[derive(Debug, Clone)]
pub enum Path {
    Ident(String),
    Field {
        head: Box<Located<Self>>,
        field: Located<String>,
    },
    Index {
        head: Box<Located<Self>>,
        index: Box<Located<Expression>>,
    },
}
#[derive(Debug, Clone)]
pub enum Type {
    Ident(String),
    Generic {
        head: Box<Located<Self>>,
        sub: Box<Located<Self>>,
    },
    Array {
        head: Box<Located<Self>>,
        size: Option<i64>,
    },
}