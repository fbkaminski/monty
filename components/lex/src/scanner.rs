use std::str::Chars;
use std::iter::Peekable;
use crate::token::Token;

#[derive(Clone, Default)]
pub struct Span {
    begin: usize,
    end: usize,
}

#[derive(Clone, Default)]
struct TokenInfo {
    token: Token,
    location: Span,
    literal: Span,
}

pub struct Scanner<'a> {
    literals: String,
    current: TokenInfo,
    next: TokenInfo,
    position: usize,
    peekable: Peekable<Chars<'a>>,
    c0: Option<char>
}

impl Scanner<'_> {

    pub fn new<'a>(code: &'a str) -> Scanner<'a> {
        let mut peekable = code.chars().peekable();
        let ch = peekable.next();
        Scanner {
            literals: String::new(),
            current: Default::default(),
            next: Default::default(),
            position: 0,
            peekable: peekable,
            c0: ch,
        }
    }

    pub fn location(&self) -> Span {
        self.current.location.clone()
    }

    pub fn peek_location(&self) -> Span {
        self.next.location.clone()
    }

    pub fn next_token(&mut self) -> Token {
        self.current = self.next.clone();
        self.scan();
        self.current.token.clone()
    }

    pub fn peek_token(&self) -> Token {
        self.next.token.clone()
    }

    pub fn scan(&mut self) {
        let mut token = Token::Illegal;
        loop {
            self.next.location.begin = self.position;
            self.skip_whitespace();
            token = self.scan_token();
            if token != Token::Comment {
                break;
            }
        }
        self.next.location.end = self.position;
        self.current.token = token;
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                Some(' ') | Some('\n') | Some('\r') => { let _ = self.next_char();},
                _ => break
            }
        }
    }

    pub fn scan_token(&mut self) -> Token {
        match self.peek_char() {
            Some('"') | Some('\'') => self.scan_string(),
            Some('<') => {
                match self.next_char() {
                    Some('=') => self.select(Token::Lte),
                    None => Token::Eos,
                    _ => Token::Lt,
                }
            },
            Some('>') => {
                match self.next_char() {
                    Some('=') => self.select(Token::Gte),
                    None => Token::Eos,
                    _ => Token::Gt,
                }
            },
            Some('=') => {
                match self.next_char() {
                    Some('=') => self.select(Token::Eq),
                    _ => Token::Assign
                }
            },
            Some('+') => {
                match self.next_char() {
                    Some('=') => self.select(Token::AssignAdd),
                    _ => Token::Add,
                }
            },
            Some('-') => {
                match self.next_char() {
                    Some('=') => self.select(Token::AssignSub),
                    Some('>') => self.select(Token::RArrow),
                    _ => Token::Sub,
                }
            },
            Some('*') => {
                match self.next_char() {
                    Some('=') => self.select(Token::AssignMul),
                    _ => Token::Mul,
                }
            },
            Some('%') => {
                match self.next_char() {
                    Some('=') => self.select(Token::AssignMod),
                    _ => Token::Mod,
                }
            },
            Some('/') => {
                match self.next_char() {
                    Some('=') => self.select(Token::AssignDiv),
                    Some('/') => {
                        match self.next_char() {
                            Some('=') => self.select(Token::DivDivEqual),
                            _ => Token::DivDiv
                        }
                    },
                    _ => Token::Div
                }
            },
            Some('&') => {
                match self.next_char() {
                    Some('&') => self.select(Token::And),
                    Some('=') => self.select(Token::AssignBitAnd),
                    _ => Token::BitAnd
                }
            },
            Some('|') => {
                match self.next_char() {
                    Some('|') => self.select(Token::Or),
                    Some('=') => self.select(Token::AssignBitOr),
                    _ => Token::BitOr
                }
            },
            Some('^') => {
                match self.next_char() {
                    Some('=') => self.select(Token::AssignBitXor),
                    _ => Token::BitXor
                }
            },
            Some('.') => {
                let ch = self.next_char();
                if self.is_decimal_digit(ch) {
                    return self.scan_number();
                }
                return Token::Period;
            },
            Some('#') => self.skip_single_line_comment(),
            Some(':') => self.select(Token::Colon),
            Some(',') => self.select(Token::Comma),
            Some('(') => self.select(Token::Lparen),
            Some(')') => self.select(Token::Rparen),
            Some('[') => self.select(Token::Lbrack),
            Some(']') => self.select(Token::Rbrack),
            Some('{') => self.select(Token::Lbrace),
            Some('}') => self.select(Token::Rbrace),
            Some('~') => self.select(Token::BitNot),
            Some('0'..='9') => self.scan_number(),
            Some('a'..='z') | Some('A'..='Z') => self.scan_identifier(),
            Some(_) => self.select(Token::Illegal),
            None => Token::Eos
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.c0
    }

    fn next_char(&mut self) -> Option<char> {
        self.c0 = self.peekable.next();
        self.position += 1;
        self.c0
    }

    fn scan_keyword(&self) -> Option<Token> {
        let keyword = self.literal();
        match keyword {
            "and" => Some(Token::And),
            "as" => Some(Token::As),
            "assert" => Some(Token::Assert),
            "break" => Some(Token::Break),
            "impl" => Some(Token::Impl),
            "continue" => Some(Token::Continue),
            "fn" => Some(Token::Fn),
            "del" => Some(Token::Del),
            "elif" => Some(Token::Elif),
            "else" => Some(Token::Else),
            "except" => Some(Token::Except),
            "finally" => Some(Token::Finally),
            "for" => Some(Token::For),
            "from" => Some(Token::From),
            "global" => Some(Token::Global),
            "if" => Some(Token::If),
            "use" => Some(Token::Use),
            "in" => Some(Token::In),
            "is" => Some(Token::Is),
            "lambda" => Some(Token::Lambda),
            "nonlocal" => Some(Token::Nonlocal),
            "not" => Some(Token::Not),
            "or" => Some(Token::Or),
            "pass" => Some(Token::Pass),
            "raise" => Some(Token::Raise),
            "return" => Some(Token::Return),
            "struct" => Some(Token::Struct),
            "try" => Some(Token::Try),
            "while" => Some(Token::While),
            "with" => Some(Token::With),
            "yeld" => Some(Token::Yeld),
            _ => None
        }

    }

    fn scan_literal(&self) -> Option<Token> {
        let literal = self.literal();
        match literal {
            "bool" => Some(Token::Bool),
            "float" => Some(Token::Float),
            "int" => Some(Token::Int),
            "str" => Some(Token::Str),
            "none" => Some(Token::NoneLiteral),
            "True" => Some(Token::TrueLiteral),
            "False" => Some(Token::FalseLiteral),
            _ => None
        }
    }

    fn skip_single_line_comment(&mut self) -> Token {
        loop {
            match self.peek_char() {
                Some('\n') => break,
                None => break,
                _ => {self.next_char();},
            };
        }
        return Token::Comment;
    }

    fn skip_multi_line_comment(&mut self) -> Token {
        return Token::Comment;
    }

    fn scan_identifier(&mut self) -> Token {
        self.start_literal();
        while self.is_character(self.peek_char()) || self.is_decimal_digit(self.peek_char()) {
            self.add_char(self.peek_char());
            self.next_char();
        }
        self.end_literal();
        let mut tok = self.scan_keyword();
        if tok.is_some() {
            return tok.unwrap();
        }
        tok = self.scan_literal();
        if tok.is_some() {
            return tok.unwrap();
        }
        let identifier = self.literal();
        Token::Identifier{value: identifier.to_string()}
    }

    fn scan_number(&mut self) -> Token {
        self.start_literal();
        let mut ch = self.peek_char();
        while self.is_decimal_digit(ch) {
            self.add_char(ch);
            ch = self.next_char();
        }
        self.end_literal();
        let number = self.literal();
        return Token::Number{value: number.to_string()};
    }

    fn scan_string(&mut self) -> Token {
        // fixme: naive implementation
        let mut string_begin = false;
        let mut ch = self.peek_char();
        while self.is_character(ch) || self.is_decimal_digit(ch) || ch == Some(' ') || ch == Some('\'') || ch == Some('"') {
            if ch == Some('\'') || ch == Some('"') {
                if string_begin {
                    self.end_literal();
                    let _ = self.next_char();
                    break;
                } else {
                    self.start_literal();
                    ch = self.next_char();
                    string_begin = true;
                    continue;
                }
            }
            self.add_char(ch);
            ch = self.next_char();
        }

        let string = self.literal();
        return Token::String{value: string.to_string()};
    }

    fn select(&mut self, tok: Token) -> Token {
        self.next_char();
        tok
    }

    fn select_if(&mut self, ch: char, then: Token, el: Token) -> Token {
        if Some(ch) == self.next_char() {
            return then
        }
        el
    }

    fn is_character(&self, ch: Option<char>) -> bool {
        match ch {
            Some('a'..='z') | Some('A'..='Z') => true,
            _ => false
        }
    }

    fn is_decimal_digit(&self, ch: Option<char>) -> bool {
        match ch {
            Some('0'..='9') => true,
            _ => false,
        }
    }

    fn add_char(&mut self, ch: Option<char>) {
        if ch.is_some() {
            self.literals.push(ch.unwrap());
        }
    }

    fn start_literal(&mut self) {
        self.next.literal.begin = self.literals.len();
    }

    fn end_literal(&mut self) {
        self.next.literal.end = self.literals.len();
        self.add_char(Some(0 as char));
    }

    fn literal(&self) -> &str {
        &self.literals[self.next.literal.begin..self.next.literal.end]
    }

}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();
        match tok {
            Token::Eos => None,
            _ => Some(tok)
        }
    }
}