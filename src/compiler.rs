use super::ast::*;
use std::fmt::Display;

pub trait WATCompilable: Sized {
    type Output;
    fn compile(compiler: &mut Compiler) -> Result<Self::Output, CompilerError>;
}
pub struct Compiler {
    pub func_frames: Vec<FunctionFrame>,
}
#[derive(Debug, Clone)]
pub struct FunctionFrame {
    pub instrs: Vec<String>,
    pub parameter: Vec<(String, Type)>,
    pub locals: Vec<(String, Type)>,
}
pub type Register = u16;

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerError {
    NotFound(String),
}
impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::NotFound(ident) => write!(f, "can't find {ident:?}"),
        }
    }
}
