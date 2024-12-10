use std::fmt;
use std::marker::Copy;

// pub enum StatementKind {
//     Loop,
//     ForIn,
//     Continue,
//     Break,
//     Return,
//     Switch,
//     If,
//     Empty
// }

// pub enum ExpressionKind {
//     Literal,
//     ObjectLiteral,
//     ArrayLiteral,
//     Property,
//     Call,
//     CallRuntime,
//     Unary,
//     Binary,
//     Conditional,
//     Assignment,
//     FunctionLiteral
// }

// // statement
// pub struct Statement {
//     pub kind: StatementKind
// }

// // expression
// pub struct Expression {
//     pub kind: ExpressionKind
// }

// // literal
// pub struct Literal {

// }

// // call
// pub struct Call {

// }


// pub struct Declaration {

// }

// // block
// pub struct Block {
//     pub statements: Vec<Statement>,
//     pub is_initializer_block: bool
// }

// impl Block {

//     pub fn add_statement<'a>(&mut self, statement: Statement) {
//         self.statements.push(statement);
//     }

// }

#[derive(Clone, Copy)]
pub enum BinaryOp {
    Addition,
    Subtraction,
    Division,
    Multiplication
}

pub enum NumberKind {
    Integer,
    Double
}

pub struct Number {
    pub kind: NumberKind,
    // fixme: value should be an Identifier instead
    pub value: String
}

pub struct BinaryExpression {
    pub op: BinaryOp,
    pub left: Number,
    pub right: Number
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryOp::Addition => write!(f, "+"),
            BinaryOp::Division => write!(f, "/"),
            BinaryOp::Multiplication => write!(f, "*"),
            BinaryOp::Subtraction => write!(f, "-"),
            _ => write!(f, "unknown")
        }
    }
}