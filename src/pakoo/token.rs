// src/parser/token.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String), // 变量名或函数名，如 "x", "sin", "length"
    Plus,               // +
    Minus,              // -
    Star,               // *
    Slash,              // /
    Caret,              // ^
    LParen,             // (
    RParen,             // )
    Comma,              // ,
    EOF,
}

pub struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.input.peek() {
            None => Token::EOF,
            Some(&c) => match c {
                '+' => {
                    self.input.next();
                    Token::Plus
                }
                '-' => {
                    self.input.next();
                    Token::Minus
                }
                '*' => {
                    self.input.next();
                    Token::Star
                }
                '/' => {
                    self.input.next();
                    Token::Slash
                }
                '^' => {
                    self.input.next();
                    Token::Caret
                }
                '(' => {
                    self.input.next();
                    Token::LParen
                }
                ')' => {
                    self.input.next();
                    Token::RParen
                }
                ',' => {
                    self.input.next();
                    Token::Comma
                }
                '0'..='9' | '.' => self.read_number(),
                'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
                _ => panic!("非法字符: {}", c), // 实际项目中应返回 Result
            },
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Token {
        let mut s = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_digit(10) || c == '.' {
                s.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        Token::Number(s.parse().unwrap_or(0.0))
    }

    fn read_identifier(&mut self) -> Token {
        let mut s = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        Token::Identifier(s)
    }
}
