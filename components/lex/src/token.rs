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
    Semicolon, //";"
    Period, // "."
    Conditional, // "?"
    Inc, // "++"
    Dec, // "--"

    /* Assignment operators. */
    Assign, // "="
    AssignBitOr, // "|="
    AssignBitXor, // "^="
    AssignBitAnd, // "&="
    AssignShl, // "<<="
    AssignSar, // ">>="
    AssignShr, // ">>>="
    AssignAdd, // "+="
    AssignSub, // "-="
    AssignMul, // "*="
    AssignDiv, // "/="
    AssignMod, // "%="

    /* Binary operators sorted by precedence. */
    Comma, // ","
    Or, // "||"
    And, // "&&"
    BitOr, // "|"
    BitXor, // "^"
    BitAnd, // "&"
    Shl, // "<<"
    Sar, // ">>"
    Shr, // ">>>"
    Add, // "+"
    Sub, // "-"
    Mul, // "*"
    Div, // "/"
    Mod, // "%"

    /* Compare operators sorted by precedence. */
    Eq, // "=="
    Ne, // "!="
    Lt, // "<"
    Gt, // ">"
    Lte, // "<="
    Gte, // ">="
    In, // "in"

    /* Unary operators. */
    Not, // "!"
    BitNot, // "~"
    Delete, // "delete"
    Typeof, // "typeof"
    Void, // "void"

    /* Keywords */
    Break, // "break"
    Case, // "case"
    Catch, // "catch"
    Continue, // "continue"
    Default, // "default"
    /* DELETE */
    Do, // "do"
    Else, // "else"
    Finally, // "finally"
    For, // "for"
    Function, // "function"
    If, // "if"
    /* IN */
    /* INSTANCEOF */
    New, // "new"
    Return, // "return"
    Switch, // "switch"
    This, // "this"
    Throw, // "throw"
    Try, // "try"
    /* TYPEOF */
    Var, // "var"
    /* VOID */
    While, // "while"
    With, // "with"

    /* reserved words ??*/
    Abstract, // "abstract"
    Boolean, // "boolean"
    Byte, // "byte"
    Char, // "char"
    Class, // "class"
    Const, // "const"
    Double, // "double"
    Enum, // "enum"
    Export, // "export"
    Extends, // "extends"
    Final, // "final"
    Float, // "float"
    Goto, // "goto"
    Implements, // "implements"
    Import, // "import"
    Int, // "int"
    Interface, // "interface"
    Long, // "long"
    Native, // "native"
    Package, // "package"
    Private, // "private"
    Protected, // "protected"
    Public, // "public"
    Short, // "short"
    Static, // "static"
    Super, // "super"
    Throws, // "throws"

    NullLiteral, // "null"
    TrueLiteral, // "true"
    FalseLiteral, // "false"
    Number(Vec<char>),
    String(Vec<char>),

    /* Identifiers (not keywords or future reserved words). */
    Identifier(Vec<char>),

    /* Illegal token - not able to scan. */
    Illegal, // "Illegal"

    Comment(u32)
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
            Token::Conditional => "?".to_string(),
            Token::Inc => "++".to_string(),
            Token::Dec => "--".to_string(),
            Token::Assign => "=".to_string(),
            Token::AssignBitOr => "|=".to_string(),
            Token::AssignBitXor => "^=".to_string(),
            Token::AssignBitAnd => "&=".to_string(),
            Token::AssignShl => "<<=".to_string(),
            Token::AssignSar => ">>=".to_string(),
            Token::AssignShr => ">>>=".to_string(),
            Token::AssignAdd => "+=".to_string(),
            Token::AssignSub => "-=".to_string(),
            Token::AssignMul => "*=".to_string(),
            Token::AssignDiv => "/=".to_string(),
            Token::AssignMod => "%=".to_string(),
            Token::Comma => ",".to_string(),
            Token::Or => "||".to_string(),
            Token::And => "&&".to_string(),
            Token::BitOr => "|".to_string(),
            Token::BitXor => "^".to_string(),
            Token::BitAnd => "&".to_string(),
            Token::Shl => "<<".to_string(),
            Token::Sar => ">>".to_string(),
            Token::Shr => ">>>".to_string(),
            Token::Add => "+".to_string(),
            Token::Sub => "-".to_string(),
            Token::Mul => "*".to_string(),
            Token::Div => "/".to_string(),
            Token::Mod => "%".to_string(),
            Token::Eq => "=".to_string(),
            Token::Ne => "!=".to_string(),
            Token::Lt => "<".to_string(),
            Token::Gt => ">".to_string(),
            Token::Lte => "<=".to_string(),
            Token::Gte => ">=".to_string(),
            Token::In => "in".to_string(),
            Token::Not => "!".to_string(),
            Token::BitNot => "~".to_string(),
            Token::Delete => "delete".to_string(),
            Token::Typeof => "typeof".to_string(),
            Token::Void => "void".to_string(),
            Token::Break => "break".to_string(),
            Token::Case => "case".to_string(),
            Token::Catch => "catch".to_string(),
            Token::Continue => "continue".to_string(),
            Token::Default => "default".to_string(),
            Token::Do => "do".to_string(),
            Token::Else => "else".to_string(),
            Token::Finally => "finally".to_string(),
            Token::For => "for".to_string(),
            Token::Function => "def".to_string(),
            Token::If => "if".to_string(),
            Token::New => "new".to_string(),
            Token::Return => "return".to_string(),
            Token::Switch => "switch".to_string(),
            Token::This => "this".to_string(),
            Token::Throw => "throw".to_string(),
            Token::Try => "try".to_string(),
            Token::Var => "var".to_string(),
            Token::While => "while".to_string(),
            Token::With => "with".to_string(),
            Token::Abstract => "abstract".to_string(),
            Token::Boolean => " bool".to_string(),
            Token::Byte => "byte".to_string(),
            Token::Char => "char".to_string(),
            Token::Class => "class".to_string(),
            Token::Const => "const".to_string(),
            Token::Double => "double".to_string(), //
            Token::Enum => "enum".to_string(), //
            Token::Export => "export".to_string(), //
            Token::Extends => "extends".to_string(), //
            Token::Final => "final".to_string(), //
            Token::Float => "float".to_string(), //
            Token::Goto => "goto".to_string(), //
            Token::Implements => "implements".to_string(), //
            Token::Import => "import".to_string(), //
            Token::Int => "int".to_string(), //
            Token::Interface => "interface".to_string(), //
            Token::Long => "long".to_string(), //
            Token::Native => "native".to_string(), //
            Token::Package => "package".to_string(), //
            Token::Private => "private".to_string(), //
            Token::Protected => "protected".to_string(), //
            Token::Public => "public".to_string(), //
            Token::Short => "short".to_string(), //
            Token::Static => "static".to_string(), //
            Token::Super => "super".to_string(), //
            Token::Throws => "throws".to_string(), //
            Token::NullLiteral => "null".to_string(), //
            Token::TrueLiteral => "true".to_string(), //
            Token::FalseLiteral => "false".to_string(), //
            Token::Number(val) => val.into_iter().collect(),
            Token::String(val) => val.into_iter().collect(),
            Token::Identifier(val) => val.into_iter().collect(),
            _ => "".to_string()
        }
    }

    pub fn is_assignment_op(self) -> bool {
        match self {
            Token::Assign => true,
            Token::AssignBitOr => true,
            Token::AssignBitXor => true,
            Token::AssignBitAnd => true,
            Token::AssignShl => true,
            Token::AssignSar => true,
            Token::AssignShr => true,
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
            Token::Shl => true,
            Token::Sar => true,
            Token::Shr => true,
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
            Token::Shl => true,
            Token::Sar => true,
            Token::Shr => true,
            Token::BitNot => true,
            _ => false,
        }
    }

    pub fn is_unary_op(self) -> bool {
        match self {
            Token::Not => true,
            Token::BitNot => true,
            Token::Delete => true,
            Token::Typeof => true,
            Token::Void => true,
            Token::Add => true,
            Token::Sub => true,
            _ => false,
        }
    }

    pub fn is_count_op(self) -> bool {
        match self {
            Token::Inc => true,
            Token::Dec => true,
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
            Token::Conditional => "Conditional",
            Token::Inc => "Inc",
            Token::Dec => "Dec",
            Token::Assign => "Assign",
            Token::AssignBitOr => "AssignBitOr",
            Token::AssignBitXor => "AssignBitXor",
            Token::AssignBitAnd => "AssignBitAnd",
            Token::AssignShl => "AssignShl",
            Token::AssignSar => "AssignSar",
            Token::AssignShr => "AssignShr",
            Token::AssignAdd => "AssignAdd",
            Token::AssignSub => "AssignSub",
            Token::AssignMul => "AssignMul",
            Token::AssignDiv => "AssignDiv",
            Token::AssignMod => "AssignMod",
            Token::Comma => "Comma",
            Token::Or => "Or",
            Token::And => "And",
            Token::BitOr => "BitOr",
            Token::BitXor => "BitXor",
            Token::BitAnd => "BitAnd",
            Token::Shl => "Shl",
            Token::Sar => "Sar",
            Token::Shr =>  "Shr",
            Token::Add => "Add",
            Token::Sub => "Sub",
            Token::Mul => "Mul",
            Token::Div => "Div",
            Token::Mod => "Mod",
            Token::Eq => "Eq",
            Token::Ne => "Ne",
            Token::Lt => "Lt",
            Token::Gt => "Gt",
            Token::Lte => "Lte",
            Token::Gte => "Gte",
            Token::In => "In",
            Token::Not => "Not",
            Token::BitNot => "BitNot",
            Token::Delete => "Delete",
            Token::Typeof => "Typeof",
            Token::Void => "Void",
            Token::Break => "Break",
            Token::Case => "Case",
            Token::Catch => "Catch",
            Token::Continue => "Continue",
            Token::Default => "Default",
            Token::Do => "Do",
            Token::Else => "Else",
            Token::Finally => "Finally",
            Token::For => "For",
            Token::Function => "Function",
            Token::If => "If",
            Token::New => "New",
            Token::Return => "Return",
            Token::Switch => "Switch",
            Token::This => "This",
            Token::Throw => "Throw",
            Token::Try => "Try",
            Token::Var => "Var",
            Token::While => "While",
            Token::With => "With",
            Token::Abstract => "Abstract",
            Token::Boolean => " Boolean",
            Token::Byte => "Byte",
            Token::Char => "Char",
            Token::Class => "Class",
            Token::Const => "Const",
            Token::Double => "Double",
            Token::Enum => "Enum",
            Token::Export => "Export",
            Token::Extends => "Extends",
            Token::Final => "Final",
            Token::Float => "Float",
            Token::Goto => "Goto",
            Token::Implements => "Implements",
            Token::Import => "Import",
            Token::Int => "Int",
            Token::Interface => "Interface",
            Token::Long => "Long",
            Token::Native => "Native",
            Token::Package => "Package",
            Token::Private => "Private",
            Token::Protected => "Protected",
            Token::Public => "Public",
            Token::Short => "Short",
            Token::Static => "Static",
            Token::Super => "Super",
            Token::Throws => "Throws",
            Token::NullLiteral => "Null",
            Token::TrueLiteral => "True",
            Token::FalseLiteral => "False",
            Token::Number(_) => "Number",
            Token::String(_) => "String",
            Token::Identifier(_) => "Identifier",
            Token::Illegal => "Illegal",
            Token::Comment(_) => "Comment",
        };
        write!(f, "{}", str_val)
    }
}

impl Default for Token {
    fn default() -> Self { Token::Eos }
}