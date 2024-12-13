use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    iter::Peekable,
};

use regex::Regex;

#[derive(Debug)]
enum LexToken {
    Label(String),
    Number(u64),
    Symbol(char),
    Unknown(String),
}

#[derive(Debug)]
enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
    Unknown(Vec<LexToken>),
}

struct Lexer<T>
where
    T: Iterator<Item = char>,
{
    input: Peekable<T>,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    fn from_iter(input: T) -> Self {
        Self {
            input: input.peekable(),
        }
    }

    fn parse_label(&mut self) -> LexToken {
        let mut label = String::new();
        while let Some(&c) = self.input.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '\'' => {
                    label.push(c);
                    self.input.next();
                }
                _ => break,
            }
        }
        LexToken::Label(label)
    }

    fn parse_number(&mut self) -> LexToken {
        let mut number = String::new();
        while let Some(&c) = self.input.peek() {
            match c {
                '0'..='9' => {
                    number.push(c);
                    self.input.next();
                }
                _ => break,
            }
        }
        LexToken::Number(number.parse::<u64>().unwrap())
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = LexToken;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(&next) = self.input.peek() else {
            return None;
        };
        return match next {
            'a'..='z' | 'A'..='Z' => Some(self.parse_label()),
            '(' | ')' | ',' => {
                self.input.next();
                Some(LexToken::Symbol(next))
            }
            '0'..='9' => Some(self.parse_number()),
            _ => {
                self.input.next();
                Some(LexToken::Unknown(next.to_string()))
            }
        };
    }
}

struct Parser<T>
where
    T: Iterator<Item = LexToken>,
{
    input: Peekable<T>,
}

impl<T> Parser<T>
where
    T: Iterator<Item = LexToken>,
{
    fn from_iter(input: T) -> Self {
        Self {
            input: input.peekable(),
        }
    }
}

impl<T> Iterator for Parser<T>
where
    T: Iterator<Item = LexToken>,
{
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(token) = self.input.next() else {
            return None;
        };

        match token {
            LexToken::Label(x) if x == "mul" || x == "do" || x == "don't" => match x.as_str() {
                "mul" => {
                    let Some(LexToken::Symbol('(')) = self.input.peek() else {
                        return Some(Instruction::Unknown(vec![LexToken::Label(
                            "mul".to_string(),
                        )]));
                    };
                    self.input.next();
                    let Some(LexToken::Number(x)) = self.input.peek() else {
                        return Some(Instruction::Unknown(vec![
                            LexToken::Label("mul".to_string()),
                            LexToken::Symbol('('),
                        ]));
                    };
                    let x = *x;
                    self.input.next();
                    let Some(LexToken::Symbol(',')) = self.input.peek() else {
                        return Some(Instruction::Unknown(vec![
                            LexToken::Label("mul".to_string()),
                            LexToken::Symbol('('),
                            LexToken::Number(x),
                        ]));
                    };
                    self.input.next();
                    let Some(LexToken::Number(y)) = self.input.peek() else {
                        return Some(Instruction::Unknown(vec![
                            LexToken::Label("mul".to_string()),
                            LexToken::Symbol('('),
                            LexToken::Number(x),
                            LexToken::Symbol(','),
                        ]));
                    };
                    let y = *y;
                    self.input.next();
                    let Some(LexToken::Symbol(')')) = self.input.peek() else {
                        return Some(Instruction::Unknown(vec![
                            LexToken::Label("mul".to_string()),
                            LexToken::Symbol('('),
                            LexToken::Number(x),
                            LexToken::Symbol(','),
                            LexToken::Number(y),
                        ]));
                    };
                    self.input.next();
                    Some(Instruction::Mul(x, y))
                }
                "do" => {
                    let Some(LexToken::Symbol('(')) = self.input.peek() else {
                        return Some(Instruction::Unknown(vec![LexToken::Label(
                            "do".to_string(),
                        )]));
                    };
                    self.input.next();
                    let Some(LexToken::Symbol(')')) = self.input.peek() else {
                        return Some(Instruction::Unknown(vec![
                            LexToken::Label("do".to_string()),
                            LexToken::Symbol('('),
                        ]));
                    };
                    self.input.next();
                    Some(Instruction::Do)
                }
                "don't" => {
                    let Some(LexToken::Symbol('(')) = self.input.peek() else {
                        return Some(Instruction::Unknown(vec![LexToken::Label(
                            "don't".to_string(),
                        )]));
                    };
                    self.input.next();
                    let Some(LexToken::Symbol(')')) = self.input.peek() else {
                        return Some(Instruction::Unknown(vec![
                            LexToken::Label("don't".to_string()),
                            LexToken::Symbol('('),
                        ]));
                    };
                    self.input.next();
                    Some(Instruction::Dont)
                }
                _ => None,
            },
            _ => Some(Instruction::Unknown(vec![token])),
        }
    }
}

struct Runtime {
    acc: u64,
    enabled: bool,
}

impl Runtime {}

fn run_instructions(instructions: impl Iterator<Item = Instruction>) -> u64 {
    let mut acc = 0;
    let mut enabled = false;
    for instruction in instructions {
        match instruction {
            Instruction::Mul(x, y) => {
                if enabled {
                    acc += x * y;
                }
            }
            Instruction::Do => {
                enabled = true;
            }
            Instruction::Dont => {
                enabled = false;
            }
            Instruction::Unknown(_) => {}
        }
    }
    acc
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).unwrap();
    println!("Opening file: {}", filename);
    let file = File::open(filename)?;
    let chars = BufReader::new(file)
        .bytes()
        .filter_map(|b| b.ok())
        .map(|b| b as char);

    let lexer = Lexer::from_iter(chars);
    let parser = Parser::from_iter(lexer);
    // for input in parser {
    //     println!("{:?}", input);
    // }
    let output = run_instructions(parser);
    println!("Output: {}", output);
    Ok(())
}
