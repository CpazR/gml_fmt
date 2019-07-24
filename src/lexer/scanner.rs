use super::error_tokens::*;
use super::lex_token::*;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    input: &'a str,
    line_number: u32,
    column_number: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Scanner<'a> {
        Scanner {
            input,
            line_number: 0,
            column_number: 0,
        }
    }

    pub fn lex_input<'b>(
        &mut self,
        mut tokens: &'b mut Vec<Token<'a>>,
    ) -> Result<&'b Vec<Token<'a>>, Error<LexError>> {
        let mut iter = self.input.chars().enumerate().peekable();

        while let Some((i, c)) = iter.next() {
            match c {
                // Single Char
                '(' => self.add_simple_token(TokenType::LeftParen, &mut tokens),
                ')' => self.add_simple_token(TokenType::RightParen, &mut tokens),
                '{' => self.add_simple_token(TokenType::LeftBrace, &mut tokens),
                '}' => self.add_simple_token(TokenType::RightBrace, &mut tokens),
                ',' => self.add_simple_token(TokenType::Comma, &mut tokens),
                '-' => self.add_simple_token(TokenType::Minus, &mut tokens),
                '+' => self.add_simple_token(TokenType::Plus, &mut tokens),
                ';' => self.add_simple_token(TokenType::Semicolon, &mut tokens),
                '*' => self.add_simple_token(TokenType::Star, &mut tokens),

                // Branching multichar
                '!' => {
                    if self.peek_and_check_consume(&mut iter, '=') {
                        self.add_multiple_token(TokenType::BangEqual, &mut tokens, 2);
                    } else {
                        self.add_simple_token(TokenType::Bang, &mut tokens)
                    }
                }
                '=' => {
                    if self.peek_and_check_consume(&mut iter, '=') {
                        self.add_multiple_token(TokenType::EqualEqual, &mut tokens, 2);
                    } else {
                        self.add_simple_token(TokenType::Equal, &mut tokens)
                    }
                }
                '<' => {
                    if self.peek_and_check_consume(&mut iter, '=') {
                        self.add_multiple_token(TokenType::LessEqual, &mut tokens, 2);
                    } else {
                        self.add_simple_token(TokenType::Less, &mut tokens)
                    }
                }
                '>' => {
                    if self.peek_and_check_consume(&mut iter, '=') {
                        self.add_multiple_token(TokenType::GreaterEqual, &mut tokens, 2);
                    } else {
                        self.add_simple_token(TokenType::Greater, &mut tokens)
                    }
                }

                // string literals
                '"' => {
                    let start = i;
                    let mut current = start;

                    while let Some((i, comment_char)) = iter.peek() {
                        match comment_char {
                            '\n' => {
                                current = *i;
                                break;
                            }
                            '"' => {
                                current = iter.next().unwrap().0 + 1;
                                break;
                            }
                            _ => current = iter.next().unwrap().0,
                        };
                    }

                    self.add_multiple_token(
                        TokenType::String(&self.input[start..current]),
                        &mut tokens,
                        (current - start) as u32,
                    );
                }

                // Number literals
                '.' => {
                    match iter.peek() {
                        Some((_, next_char)) if next_char.is_digit(10) => {
                            let start = i;
                            let mut current = start;

                            // eat the "."
                            iter.next();

                            while let Some((i, number_char)) = iter.peek() {
                                if number_char.is_digit(10) {
                                    current = *i + 1;
                                    iter.next();
                                } else {
                                    break;
                                }
                            }

                            self.add_multiple_token(
                                TokenType::Number(&self.input[start..current]),
                                &mut tokens,
                                (current - start) as u32,
                            );
                        }
                        _ => self.add_simple_token(TokenType::Dot, &mut tokens),
                    }
                }

                '0'..='9' => {
                    let start = i;
                    let mut current = start + 1;

                    // Check for Hex
                    if c == '0' {
                        if let Some((_, number_char)) = iter.peek() {
                            if *number_char == 'x' {
                                iter.next();

                                while let Some((i, number_char)) = iter.peek() {
                                    if number_char.is_digit(16) {
                                        current = *i + 1;
                                        iter.next();
                                    } else {
                                        break;
                                    }
                                }

                                self.add_multiple_token(
                                    TokenType::Number(&self.input[start..current]),
                                    &mut tokens,
                                    (current - start) as u32,
                                );
                                continue;
                            }
                        }
                    }

                    let mut is_fractional = false;

                    while let Some((i, number_char)) = iter.peek() {
                        if number_char.is_digit(10) {
                            current = *i + 1;
                            iter.next();
                        } else {
                            is_fractional = *number_char == '.';
                            break;
                        }
                    }

                    if is_fractional {
                        // eat the "."
                        current = iter.next().unwrap().0 + 1;

                        while let Some((i, number_char)) = iter.peek() {
                            if number_char.is_digit(10) {
                                current = *i + 1;
                                iter.next();
                            } else {
                                break;
                            }
                        }
                    }

                    self.add_multiple_token(
                        TokenType::Number(&self.input[start..current]),
                        &mut tokens,
                        (current - start) as u32,
                    )
                }

                // Secondary Hex
                '$' => {
                    let start = i;
                    let mut current = start;

                    while let Some((i, hex_char)) = iter.peek() {
                        if hex_char.is_digit(16) {
                            current = *i + 1;
                            iter.next();
                        } else {
                            break;
                        }
                    }

                    self.add_multiple_token(
                        TokenType::Number(&self.input[start..current]),
                        &mut tokens,
                        (current - start) as u32,
                    );
                }

                // Comments
                '/' => {
                    if self.peek_and_check_consume(&mut iter, '/') {
                        let start = i;
                        let mut current = start;

                        while let Some((i, comment_char)) = iter.peek() {
                            if comment_char != &'\n' {
                                // @Jack unroll the logic of this at some point. Current confusing.
                                current = *i + 1;
                                iter.next();
                            } else {
                                break;
                            }
                        }
                        self.add_multiple_token(
                            TokenType::Comment(&self.input[start..current]),
                            &mut tokens,
                            (current - start) as u32,
                        );
                    } else {
                        self.add_simple_token(TokenType::Slash, &mut tokens);
                    }
                }
                ' ' | '\t' => self.column_number += 1,

                '\n' => self.next_line(),
                '\r' => continue,

                'A'..='z' => {
                    println!("A wild character appeared! {}", c);
                }

                _ => {
                    println!("Unexpected character {}", c);
                    self.column_number += 1;
                }
            };
        }

        self.add_simple_token(TokenType::EOF, tokens);
        Ok(tokens)
    }

    fn add_simple_token(&mut self, token_type: TokenType<'a>, tokens: &mut Vec<Token<'a>>) {
        self.add_multiple_token(token_type, tokens, 1);
    }

    fn add_multiple_token(
        &mut self,
        token_type: TokenType<'a>,
        tokens: &mut Vec<Token<'a>>,
        size: u32,
    ) {
        tokens.push(Token::new(token_type, self.line_number, self.column_number));
        self.column_number += size;
    }

    fn peek_and_check_consume(
        &self,
        iter: &mut Peekable<Enumerate<Chars>>,
        char_to_check: char,
    ) -> bool {
        if let Some((_i, next_char)) = iter.peek() {
            let ret = next_char == &char_to_check;
            if ret {
                iter.next();
            }
            ret
        } else {
            false
        }
    }

    fn next_line(&mut self) {
        self.line_number += 1;
        self.column_number = 0;
    }
}
