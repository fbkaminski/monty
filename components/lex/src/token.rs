use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Token {
    Eos,
    Lparen, // "("
    Rparen, // ")"
    Lbrack, // "["
    Rbrack, // "]"
    Lbrace, // "{"
    Rbrace, // "}"
    Colon, // ":"
    Semicolon, // ";"
    Period, // "."
    DivDiv,  // '//'
    DivDivEqual, // "//="
    At, // @
    AtEqual, // @=
    RArrow, // ->
    Ellipses, // ...
    Assign, // "="
    AssignBitOr, // "|=" VbarEqual
    AssignBitXor, // "^=" CircumflexEqual
    AssignBitAnd, // "&=" AmperEqual
    AssignAdd, // "+=" PlusEqual
    AssignSub, // "-=" MinusEqual
    AssignMul, // "*=" StarEqual
    AssignDiv, // "/=" SlashEqual
    AssignMod, // "%=" PercentEqual
    AssignLsh, // "<<="
    AssignRsh, // ">>="
    AssignMulMul, // "**="
    AssignColon, // :=
    Comma, // "," Comma
    BitOr, // "|" Vbar
    BitXor, // "^" CircumFlex
    BitAnd, // "&" Amper
    Add, // "+" Plus
    Sub, // "-" Minus
    Mul, // "*" Star
    Div, // "/" Slash
    Mod, // "%" Percent
    MulMul, // "**" DoubleStar
    Eq, // "==" EqEqual
    Ne, // "!=" NotEqual
    Lt, // "<" Less
    Gt, // ">" Greater
    Lte, // "<=" LessEqual
    Gte, // ">=" GreaterEqual
    BitNot, // "~" Tilde
    Shl,  // <<
    Shr, // >>
    Exclamation, // "!"
    And, // "and"
    As, // "as"
    Assert, // "assert"
    Break, // "break"
    Impl, // "impl"
    Continue, // "continue"
    Fn, // "fn"
    Del, // "del"
    Elif, // "elif"
    Else, // "else"
    Except, // "except"
    Finally, // "finally"
    For, // "for"
    From, // "from"
    Global, // "global"
    If, // "if"
    Use, // "use"
    In, // "in"
    Is, // "is"
    Lambda, // "lambda"
    Nonlocal, // "nonlocal"
    Not, // "not"
    Or, // "or"
    Pass, // "pass"
    Raise, // "raise"
    Return, // "return"
    Try, // "try"
    While, // "while"
    With, // "with"
    Yeld, // "yeld"
    Bool, // "bool"
    Float, // "float"
    Int, // "int"
    Str, // "str"
    Struct, // "struct"
    NoneLiteral, // "none"
    TrueLiteral, // "True"
    FalseLiteral, // "False"
    Number {value: String},
    String {value: String},
    Identifier {value: String},
    Illegal, // "Illegal" => not able to scan.
    Comment
}

impl Token {

    pub fn value(self) -> String {
        match self {
            Token::Lparen => "(".to_string(),
            Token::Rparen => ")".to_string(),
            Token::Lbrack => "[".to_string(),
            Token::Rbrack => "]".to_string(),
            Token::Lbrace => "{".to_string(),
            Token::Rbrace => "}".to_string(),
            Token::Colon => ":".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Period => ".".to_string(),
            Token::DivDiv => "//".to_string(),
            Token::DivDivEqual => "//=".to_string(),
            Token::At => "@".to_string(),
            Token::AtEqual => "@=".to_string(),
            Token::RArrow => "=>".to_string(),
            Token::Ellipses => "...".to_string(),
            Token::Assign => "=".to_string(),
            Token::AssignBitOr => "|=".to_string(),
            Token::AssignBitXor => "^=".to_string(),
            Token::AssignBitAnd => "&=".to_string(),
            Token::AssignAdd => "+=".to_string(),
            Token::AssignSub => "-=".to_string(),
            Token::AssignMul => "*=".to_string(),
            Token::AssignDiv => "/=".to_string(),
            Token::AssignMod => "%=".to_string(),
            Token::AssignLsh => "<<=".to_string(),
            Token::AssignRsh => ">>=".to_string(),
            Token::AssignMulMul => "**=".to_string(),
            Token::AssignColon => ":=".to_string(),
            Token::Comma => ",".to_string(),
            Token::BitOr => "|".to_string(),
            Token::BitXor => "^".to_string(),
            Token::BitAnd => "&".to_string(),
            Token::Add => "+".to_string(),
            Token::Sub => "-".to_string(),
            Token::Mul => "*".to_string(),
            Token::Div => "/".to_string(),
            Token::Mod => "%".to_string(),
            Token::MulMul => "**".to_string(),
            Token::Eq => "==".to_string(),
            Token::Ne => "!=".to_string(),
            Token::Lt => "<".to_string(),
            Token::Gt => ">".to_string(),
            Token::Lte => "<=".to_string(),
            Token::Gte => ">=".to_string(),
            Token::BitNot => "~".to_string(),
            Token::Shl => "<<".to_string(),
            Token::Shr => ">>".to_string(),
            Token::Exclamation => "!".to_string(),
            Token::And => "and".to_string(),
            Token::As => "as".to_string(),
            Token::Assert => "assert".to_string(),
            Token::Break => "break".to_string(),
            Token::Impl => "impl".to_string(),
            Token::Continue => "continue".to_string(),
            Token::Fn => "fn".to_string(),
            Token::Del => "del".to_string(),
            Token::Elif => "elif".to_string(),
            Token::Else => "else".to_string(),
            Token::Except => "except".to_string(),
            Token::Finally => "finally".to_string(),
            Token::For => "for".to_string(),
            Token::From => "from".to_string(),
            Token::Global => "global".to_string(),
            Token::If => "if".to_string(),
            Token::Use => "use".to_string(),
            Token::In => "in".to_string(),
            Token::Is => "is".to_string(),
            Token::Lambda => "lambda".to_string(),
            Token::Nonlocal => "nonlocal".to_string(),
            Token::Not => "not".to_string(),
            Token::Or => "or".to_string(),
            Token::Pass => "pass".to_string(),
            Token::Raise => "raise".to_string(),
            Token::Return => "return".to_string(),
            Token::Try => "try".to_string(),
            Token::While => "while".to_string(),
            Token::With => "with".to_string(),
            Token::Yeld => "yeld".to_string(),
            Token::Bool => "bool".to_string(),
            Token::Float => "float".to_string(),
            Token::Int => "int".to_string(),
            Token::Str => "str".to_string(),
            Token::Struct => "struct".to_string(),
            Token::NoneLiteral => "none".to_string(),
            Token::TrueLiteral => "True".to_string(),
            Token::FalseLiteral => "False".to_string(),
            Token::Number{value: val} => val,
            Token::String{value: val} => val,
            Token::Identifier{value: val} => val,
            _ => "".to_string()
        }
    }

    pub fn is_assignment_op(self) -> bool {
        //use Self::*;
        //matches!(self, Assign | AssignBitOr | ....)
        match self {
            Token::Assign => true,
            Token::AssignBitOr => true,
            Token::AssignBitXor => true,
            Token::AssignBitAnd => true,
            Token::AssignAdd => true,
            Token::AssignSub => true,
            Token::AssignMul => true,
            Token::AssignDiv => true,
            Token::AssignMod => true,
            _ => false,
        }
    }

    pub fn is_binary_op(self) -> bool {
        match self {
            Token::Comma => true,
            Token::Or => true,
            Token::And => true,
            Token::BitOr => true,
            Token::BitXor => true,
            Token::BitAnd => true,
            Token::Add => true,
            Token::Sub => true,
            Token::Mul => true,
            Token::Div => true,
            Token::Mod => true,
            _ => false,
        }
    }

    pub fn is_compare_op(self) -> bool {
        match self {
            Token::Eq => true,
            Token::Ne => true,
            Token::Lt => true,
            Token::Gt => true,
            Token::Lte => true,
            Token::Gte => true,
            Token::In => true,
            _ => false,
        }
    }

    pub fn is_bit_op(self) -> bool {
        match self {
            Token::BitOr => true,
            Token::BitXor => true,
            Token::BitAnd => true,
            Token::BitNot => true,
            _ => false,
        }
    }

    pub fn is_unary_op(self) -> bool {
        match self {
            Token::Not => true,
            Token::BitNot => true,
            // ??
            Token::Add => true,
            Token::Sub => true,
            _ => false,
        }
    }

}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_val = match self {
            Token::Eos => "Eos",
            Token::Lparen => "Lparen",
            Token::Rparen => "Rparen",
            Token::Lbrack => "Lbrack",
            Token::Rbrack => "Rbrack",
            Token::Lbrace => "Lbrace",
            Token::Rbrace => "Rbrace",
            Token::Colon => "Colon",
            Token::Semicolon => "Semicolon",
            Token::Period => "Period",
            Token::DivDiv => "DivDiv",
            Token::DivDivEqual => "DivDivEqual",
            Token::At => "At",
            Token::AtEqual => "AtEqual",
            Token::RArrow => "RArrow",
            Token::Ellipses => "Ellipses",
            Token::Assign => "Assign",
            Token::AssignBitOr => "AssignBitOr",
            Token::AssignBitXor => "AssignBitXor",
            Token::AssignBitAnd => "AssignBitAnd",
            Token::AssignAdd => "AssignAdd",
            Token::AssignSub => "AssignSub",
            Token::AssignMul => "AssignMul",
            Token::AssignDiv => "AssignDiv",
            Token::AssignMod => "AssignMod",
            Token::AssignLsh => "AssignLsh",
            Token::AssignRsh => "AssignRsh",
            Token::AssignMulMul => "AssignMulMul",
            Token::AssignColon => "AssignColon",
            Token::Comma => "Comma",
            Token::BitOr => "BitOr",
            Token::BitXor => "BitXor",
            Token::BitAnd => "BitAnd",
            Token::Add => "Add",
            Token::Sub => "Sub",
            Token::Mul => "Mul",
            Token::Div => "Div",
            Token::Mod => "Mod",
            Token::MulMul => "MulMul",
            Token::Eq => "Eq",
            Token::Ne => "Ne",
            Token::Lt => "Lt",
            Token::Gt => "Gt",
            Token::Lte => "Lte",
            Token::Gte => "Gte",
            Token::BitNot => "BitNot",
            Token::Shl => "Shl",
            Token::Shr => "Shr",
            Token::Exclamation => "Exclamation",
            Token::And => "And",
            Token::As => "As",
            Token::Assert => "Assert",
            Token::Break => "Break",
            Token::Impl => "Impl",
            Token::Continue => "Continue",
            Token::Fn => "Fn",
            Token::Del => "Del",
            Token::Elif => "Elif",
            Token::Else => "Else",
            Token::Except => "Except",
            Token::Finally => "Finally",
            Token::For => "For",
            Token::From => "From",
            Token::Global => "Global",
            Token::If => "If",
            Token::Use => "Use",
            Token::In => "In",
            Token::Is => "Is",
            Token::Lambda => "Lambda",
            Token::Nonlocal => "Nonlocal",
            Token::Not => "Not",
            Token::Or => "Or",
            Token::Pass => "Pass",
            Token::Raise => "Raise",
            Token::Return => "Return",
            Token::Try => "Try",
            Token::While => "While",
            Token::With => "With",
            Token::Yeld => "Yeld",
            Token::Bool => "Bool",
            Token::Float => "Float",
            Token::Int => "Int",
            Token::Str => "Str",
            Token::Struct => "Struct",
            Token::NoneLiteral => "None",
            Token::TrueLiteral => "True",
            Token::FalseLiteral => "False",
            Token::Number{value: _} => "Number",
            Token::String{value: _} => "String",
            Token::Identifier{value: _} => "Identifier",
            Token::Illegal => "Illegal",
            Token::Comment => "Comment",
        };
        write!(f, "{}", str_val)
    }
}

impl Default for Token {
    fn default() -> Self { Token::Eos }
}