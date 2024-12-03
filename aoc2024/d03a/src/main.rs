#![allow(dead_code)]
use std::{iter::Peekable, str::Chars};

const INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    let mut parser = Parser::new(INPUT_STR);
    let result = parser.parse();
    println!("TOTAL: {}", result.total);
}

struct Parser<'a> {
    last_token: Option<ValidChars>,
    next_token: Option<char>,
    token: Option<char>,
    index: usize,
    input_source: String,
    token_stream: Peekable<Chars<'a>>,
    tokenized: Vec<ValidChars>,
}

#[derive(Debug)]
struct ParseResult {
    parsed: Vec<ValidChars>,
    total: i64,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        let token_stream = text.chars().peekable();
        Self {
            last_token: None,
            next_token: None,
            token: None,
            index: 0,
            input_source: text.to_owned(),
            token_stream,
            tokenized: Vec::new(),
        }
    }

    fn parse(&mut self) -> ParseResult {
        let result = self.parse_stream();
        result
    }

    fn parse_stream(&mut self) -> ParseResult {
        use ValidChars::*;

        let mut tokenized = vec![];
        let mut matches = 0;
        let mut left_num = None;
        let mut total = 0;

        self.next();
        loop {
            let Some(token) = self.token else { break };

            match ValidChars::parse(token) {
                Some(LetterM) => {
                    if self.parse_mul() {
                        tokenized.push(LetterM);
                        tokenized.push(LetterU);
                        tokenized.push(LetterL);
                    } else {
                        self.next();
                        self.last_token = None;
                    }
                }
                Some(LeftParen) => {
                    let None = left_num else {
                        self.next();
                        self.last_token = None;
                        left_num = None;
                        continue;
                    };

                    if let Some(num) = self.parse_num() {
                        tokenized.push(LeftParen);
                        tokenized.push(Number(num));
                        left_num = Some(num);
                    } else {
                        self.next();
                        self.last_token = None;
                        continue;
                    }
                }
                Some(Comma) => {
                    let Some(past_num) = left_num else {
                        self.next();
                        self.last_token = None;
                        continue;
                    };

                    let Some(num) = self.parse_num() else {
                        self.next();
                        self.last_token = None;
                        left_num = None;
                        continue;
                    };

                    let Some(token) = self.token else {
                        self.next();
                        self.last_token = None;
                        left_num = None;
                        continue;
                    };

                    if ValidChars::parse(token) != Some(RightParen) {
                        self.next();
                        self.last_token = None;
                        left_num = None;
                        continue;
                    }

                    tokenized.push(Comma);
                    tokenized.push(Number(num));
                    total += past_num * num;
                    matches += 1;
                    left_num = None;
                }

                _ => {
                    left_num = None;
                    self.next();
                    self.last_token = None;
                }
            }
        }

        println!("MATCHES: {}", matches);

        ParseResult {
            parsed: tokenized,
            total,
        }
    }

    fn next(&mut self) -> Option<char> {
        self.last_token = self.token.and_then(ValidChars::parse);
        self.token = self.token_stream.next();
        self.next_token = self.token_stream.peek().copied();
        self.index += 1;

        self.token
    }

    fn parse_num(&mut self) -> Option<i64> {
        if self.last_token.is_none() || self.token.is_none() {
            return None;
        }

        let mut num = vec![];
        while let Some(token) = self.next() {
            match ValidChars::parse(token) {
                Some(ValidChars::Number(n)) => num.push(n),
                _ => break,
            }
        }
        if num.is_empty() {
            return None;
        } else {
            Some(num.iter().fold(0, |acc, &x| acc * 10 + x))
        }
    }

    fn parse_mul(&mut self) -> bool {
        if self.token.is_none() {
            return false;
        }

        let mut mul = vec![ValidChars::LetterM];
        while let Some(token) = self.next() {
            match ValidChars::parse(token) {
                Some(ValidChars::LetterU) if mul.len() == 1 => mul.push(ValidChars::LetterU),
                Some(ValidChars::LetterL) if mul.len() == 2 => mul.push(ValidChars::LetterL),
                _ => break,
            }
        }

        mul == vec![
            ValidChars::LetterM,
            ValidChars::LetterU,
            ValidChars::LetterL,
        ]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ValidChars {
    Number(i64),
    LetterM,
    LetterU,
    LetterL,
    LeftParen,
    RightParen,
    Comma,
}

impl std::fmt::Display for ValidChars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidChars::Number(n) => write!(f, "{}", n),
            ValidChars::LetterM => write!(f, "m"),
            ValidChars::LetterU => write!(f, "u"),
            ValidChars::LetterL => write!(f, "l"),
            ValidChars::LeftParen => write!(f, "("),
            ValidChars::RightParen => write!(f, ")"),
            ValidChars::Comma => write!(f, ","),
        }
    }
}

impl ValidChars {
    fn parse(c: char) -> Option<ValidChars> {
        match c {
            'M' | 'm' => Some(ValidChars::LetterM),
            'U' | 'u' => Some(ValidChars::LetterU),
            'L' | 'l' => Some(ValidChars::LetterL),
            '(' => Some(ValidChars::LeftParen),
            ')' => Some(ValidChars::RightParen),
            ',' => Some(ValidChars::Comma),
            '0'..='9' => Some(ValidChars::Number(c.to_digit(10).unwrap() as i64)),
            _ => None,
        }
    }
}
