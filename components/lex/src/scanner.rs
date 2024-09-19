use std::str::Chars;
use std::iter::Peekable;
use crate::token::Token;

#[derive(Clone, Default)]
pub struct Span {
    begin: u32,
    end: u32,
}

#[derive(Clone, Default)]
struct TokenInfo {
    token: Token,
    location: Span,
    literal: Span,
}

pub struct Scanner<'a> {
    literals: Vec<String>,
    current: TokenInfo,
    next: TokenInfo,
    position: u32,
    peekable: Peekable<Chars<'a>>,
}

impl Scanner<'_> {

    pub fn new<'a>(code: &'a str, position: u32) -> Scanner<'a> {
        Scanner {
            literals: Vec::new(),
            current: Default::default(),
            next: Default::default(),
            position: position,
            peekable: code.chars().peekable(),
        }
    }

    pub fn init(&mut self) {

    }

    pub fn location(&self) -> Span {
        self.current.location.clone()
    }

    pub fn peek_location(&self) -> Span {
        self.next.location.clone()
    }

    pub fn next(&mut self) -> Token {
        self.current = self.next.clone();
        self.scan();
        self.current.token.clone()
    }

    pub fn peek(&self) -> Token {
        self.next.token.clone()
    }

    pub fn advance(&mut self) {
        self.peekable.next();
    }

    pub fn scan(&mut self) {
        //let token = self.scan_token();
        // let len = 0;
        // while token == Token::Comment(len) {
        //     self.skip_white_space();
        //     self.next.location.begin = self.position;
        //     token = self.scan_token()
        // }
        //self.current.token = token;

        self.current.token = self.scan_token();
    }

    fn skip_white_space(&mut self) {
        while self.peekable.next_if(|&ch| ch == ' ').is_some() {
            self.advance()
        }
    }

    pub fn scan_token(&mut self) -> Token {
        match self.peekable.peek() {
            Some(&ch) => match ch {
                '"' => self.scan_string(),
                '\'' => self.scan_string(),
                '<' => {
                    match self.peekable.next() {
                        Some('=') => self.select(Token::Lte),
                        Some('<') => self.select_if('=', Token::AssignShl, Token::Shl),
                        None => Token::Eos,
                        _ => Token::Lt,
                    }
                },
                '>' => {
                    match self.peekable.next() {
                        Some('=') => self.select(Token::Gte),
                        Some('>') => {
                            match self.peekable.next() {
                                Some('=') => self.select(Token::AssignSar),
                                Some('>') => self.select_if('=', Token::AssignShr, Token::Shr),
                                _ => Token::Sar
                            }
                        },
                        None => Token::Eos,
                        _ => Token::Gt,
                    }
                },
                '=' => {
                    match self.peekable.next() {
                        Some('=') => self.select(Token::Eq),
                        _ => Token::Assign
                    }
                },
                '!' => {
                    match self.peekable.next() {
                        Some('=') => self.select(Token::Ne),
                        _ => Token::Not
                    }
                },
                '+' => {
                    match self.peekable.next() {
                        Some('+') => self.select(Token::Inc),
                        Some('=') => self.select(Token::AssignAdd),
                        _ => Token::Add,
                    }
                },
                '-' => {
                    match self.peekable.next() {
                        Some('-') => self.select(Token::Dec),
                        Some('=') => self.select(Token::AssignSub),
                        _ => Token::Sub,
                    }
                },
                '*' => {
                    match self.peekable.next() {
                        Some('=') => self.select(Token::AssignMul),
                        _ => Token::Mul,
                    }
                },
                '%' => {
                    match self.peekable.next() {
                        Some('=') => self.select(Token::AssignMod),
                        _ => Token::Mod,
                    }
                },
                '/' => {
                    match self.peekable.next() {
                        Some('/') => self.skip_single_line_comment(),
                        Some('*') => self.skip_multi_line_comment(),
                        Some('=') => self.select(Token::AssignDiv),
                        _ => Token::Div
                    }
                },
                '&' => {
                    match self.peekable.next() {
                        Some('&') => self.select(Token::And),
                        Some('=') => self.select(Token::AssignBitAnd),
                        _ => Token::BitAnd
                    }
                },
                '|' => {
                    match self.peekable.next() {
                        Some('|') => self.select(Token::Or),
                        Some('=') => self.select(Token::AssignBitOr),
                        _ => Token::BitOr
                    }
                },
                '^' => {
                    match self.peekable.next() {
                        Some('=') => self.select(Token::AssignBitXor),
                        _ => Token::BitXor
                    }
                },
                '.' => {
                    let digit = self.peekable.next().unwrap();
                    if self.is_decimal_digit(digit) {
                        return self.scan_number();
                    }
                    return Token::Period;
                },
                ':' => self.select(Token::Colon),
                ';' => self.select(Token::Semicolon),
                ',' => self.select(Token::Comma),
                '(' => self.select(Token::Lparen),
                ')' => self.select(Token::Rparen),
                '[' => self.select(Token::Lbrack),
                ']' => self.select(Token::Rbrack),
                '{' => self.select(Token::Lbrace),
                '}' => self.select(Token::Rbrace),
                '?' => self.select(Token::Conditional),
                '~' => self.select(Token::BitNot),
                _ => {
                    //println!("Default => ");
                    let c0 = self.peekable.peek();
                    if c0.unwrap().is_ascii() {
                        //println!("Identifier");
                        //if c0 in "a..z".range() {
                            return self.scan_identifier()
                        //}
                        //if c0 in "0..1".range() {
                        //    return self.scan_number();
                        //}
                    } else if c0 == None {
                        //println!("Eos");
                        return Token::Eos;
                    } else {
                        //println!("Illegal");
                        return Token::Illegal;
                    }

                }
            },
            None => { println!("None => EOS"); Token::Eos }
        }
    }

    fn skip_single_line_comment(&mut self) -> Token {
        let mut ch = self.peekable.next();
        while ch != Some('\n') {
            ch = self.peekable.next();
        }
        return Token::Comment(0);
    }

    fn skip_multi_line_comment(&mut self) -> Token {
        return Token::Comment(0);
    }

    fn scan_identifier(&mut self) -> Token {
        let tok = Token::Identifier("hello".chars().collect());
        self.advance();
        tok
    }

    fn scan_number(&mut self) -> Token {
        let tok = Token::Number("0".chars().collect());
        self.advance();
        tok
    }

    fn scan_string(&mut self) -> Token {
        let tok = Token::String("hello".chars().collect());
        self.advance();
        tok
    }

    fn select(&mut self, tok: Token) -> Token {
        self.advance();
        tok
    }

    fn select_if(&mut self, ch: char, then: Token, el: Token) -> Token {
        let c0 = self.peekable.next();
        if ch == c0.unwrap() {
            self.advance();
            return then
        }
        el
    }

    fn is_decimal_digit(&self, ch: char) -> bool {
        // FIXME: this is bollocks
        if ch == '0' {
            return true;
        }
        false
    }

}