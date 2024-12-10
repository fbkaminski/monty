pub mod token;
pub mod scanner;

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;
    use crate::token::Token;

    #[test]
    fn match_all_tokens() {
    //     // Lparen, // "(" Lpar
    //     // Rparen, // ")" Rpar
    //     // Lbrack, // "[" Lsqb
    //     // Rbrack, // "]" Rsqb
    //     // Lbrace, // "{" Lbrace
    //     // Rbrace, // "}" Rbrace
    //     // Colon, // ":" Colon
    //     // Period, // "." Dot
    //     // DivDiv,  // '//'
    //     // DivDivEqual, // "//="
    //     // At, // @
    //     // AtEqual, // @=
    //     // RArrow, // =>
    //     // Ellipses, // ...
    //     // Assign, // "=" Equal
    //     // AssignBitOr, // "|=" VbarEqual
    //     // AssignBitXor, // "^=" CircumflexEqual
    //     // AssignBitAnd, // "&=" AmperEqual
    //     // AssignAdd, // "+=" PlusEqual
    //     // AssignSub, // "-=" MinusEqual
    //     // AssignMul, // "*=" StarEqual
    //     // AssignDiv, // "/=" SlashEqual
    //     // AssignMod, // "%=" PercentEqual
    //     // Comma, // "," Comma
    //     // BitOr, // "|" Vbar
    //     // BitXor, // "^" CircumFlex
    //     // BitAnd, // "&" Amper
    //     // Add, // "+" Plus
    //     // Sub, // "-" Minus
    //     // Mul, // "*" Star
    //     // Div, // "/" Slash
    //     // Mod, // "%" Percent
    //     // MulMul, // "**" DoubleStar
    //     // Eq, // "==" EqEqual
    //     // Ne, // "!=" NotEqual
    //     // Lt, // "<" LeftShift
    //     // Gt, // ">" RightShift
    //     // Lte, // "<=" LeftShiftEqual
    //     // Gte, // ">=" RightShiftEqual
    //     // BitNot, // "~" Tilde
    //     // And, // "and"
    //     // As, // "as"
    //     // Assert, // "assert"
    //     // Break, // "break"
    //     // Class, // "class"
    //     // Continue, // "continue"
    //     // Def, // "def"
    //     // Del, // "del"
    //     // Elif, // "elif"
    //     // Else, // "else"
    //     // Except, // "except"
    //     // Finally, // "finally"
    //     // For, // "for"
    //     // From, // "from"
    //     // Global, // "global"
    //     // If, // "if"
    //     // Import, // "import"
    //     // In, // "in"
    //     // Is, // "is"
    //     // Lambda, // "lambda"
    //     // Nonlocal, // "nonlocal"
    //     // Not, // "not"
    //     // Or, // "or"
    //     // Pass, // "pass"
    //     // Raise, // "raise"
    //     // Return, // "return"
    //     // Try, // "try"
    //     // While, // "while"
    //     // With, // "with"
    //     // Yeld, // "yeld"
    //     // Bool, // "bool"
    //     // Float, // "float"
    //     // Int, // "int"
    //     // Str, // "str"
    //     // NoneLiteral, // "none"
    //     // TrueLiteral, // "True"
    //     // FalseLiteral, // "False"
    //     // Number {value: String},
    //     // String {value: String},
    //     // Identifier {value: String},
        let all_tokens = "()[]{}:.////=@@=";
        let expected_tokens = vec![
            Token::Lparen,
            Token::Rparen,
            Token::Lbrack,
            Token::Rbrack,
            Token::Lbrace,
            Token::Rbrace,
            Token::Colon,
            Token::Period,
            Token::DivDiv,
            Token::DivDivEqual,
            Token::At,
            Token::AtEqual
        ];

        let scanner = Scanner::new(all_tokens);
        for (i, token) in scanner.enumerate() {
            println!("comparing {} {}", token, expected_tokens[i]);
            assert!(token == expected_tokens[i]);
        }
    }

    #[test]
    fn scan_one_line_assignment() {
        let oneline = "abc: str = 'this is a string'";

        let expected_tokens = vec![
            Token::Identifier{value: "abc".to_string()},
            Token::Colon,
            Token::Str,
            Token::Assign,
            Token::String{value: "this is a string".to_string()},
            Token::Eos,
        ];

        let scanner = Scanner::new(oneline);

        for (i, token) in scanner.enumerate() {
            assert!(token == expected_tokens[i]);
        }
    }
}