use clap::{App, Arg};
use std::fs;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Token {
    Open,
    Close,
    Add,
    Multiply,
    Num(usize),
}

#[derive(Debug, Eq, PartialEq)]
struct Expr {
    tokens: Vec<Token>
}

impl Expr {
    fn eval1(&self) -> usize {
        let mut tokens = self.tokens.iter();
        let mut stack = vec![tokens.next().unwrap().clone()];
        loop {
            let head = stack.pop().unwrap();
            match head {
                Token::Close => {
                    let num = stack.pop().unwrap();
                    let open = stack.pop().unwrap();
                    match (open, num) {
                        (Token::Open, Token::Num(_)) => stack.push(num),
                        _ => unreachable!(),
                    }
                },
                Token::Num(n) => {
                    match stack.pop() {
                        None => {
                            match tokens.next() {
                                None => return n,
                                Some(token) => {
                                    stack.push(head);
                                    stack.push(token.clone());
                                }
                            }
                        },
                        Some(Token::Multiply) => {
                            match stack.pop().unwrap() {
                                Token::Num(m) => stack.push(Token::Num(m * n)),
                                _ => unreachable!(),
                            }
                        },
                        Some(Token::Add) => {
                            match stack.pop().unwrap() {
                                Token::Num(m) => stack.push(Token::Num(m + n)),
                                _ => unreachable!(),
                            }
                        },
                        Some(Token::Open) => {
                            if let Some(next) = tokens.next() {
                                stack.push(Token::Open);
                                stack.push(head);
                                stack.push(next.clone());
                            } else {
                                stack.push(head);
                            }
                        },
                        _ => unreachable!(),
                    }
                },
                _ => {
                    stack.push(head);
                    if let Some(token) = tokens.next() {
                        stack.push(token.clone());
                    }
                }
            }
        }
    }

    fn eval2(&self) -> usize {
        let mut tokens = self.tokens.iter();
        let mut stack = vec![tokens.next().unwrap().clone()];
        loop {
            let head = stack.pop().unwrap();
            match head {
                Token::Close => {
                    // Should be a sequence of multiplies until the Token::Open
                    let mut sub_res = 1;
                    loop {
                        match stack.pop().unwrap() {
                            Token::Num(n) => sub_res *= n,
                            Token::Multiply => {},
                            Token::Open => break,
                            _ => unreachable!(),
                        }
                    }
                    stack.push(Token::Num(sub_res));
                },
                Token::Num(n) => {
                    match stack.pop() {
                        None => {
                            match tokens.next() {
                                None => return n,
                                Some(token) => {
                                    stack.push(head);
                                    stack.push(token.clone());
                                }
                            }
                        },
                        Some(Token::Add) => {
                            match stack.pop().unwrap() {
                                Token::Num(m) => stack.push(Token::Num(n + m)),
                                _ => unreachable!(),
                            }
                        },
                        Some(Token::Multiply) => {
                            if let Some(token) = tokens.next() {
                                stack.push(Token::Multiply);
                                stack.push(head);
                                stack.push(token.clone());
                            } else {
                                match stack.pop().unwrap() {
                                    Token::Num(m) => stack.push(Token::Num(n * m)),
                                    _ => unreachable!(),
                                }
                            }
                        },
                        Some(Token::Open) => {
                            if let Some(next) = tokens.next() {
                                stack.push(Token::Open);
                                stack.push(head);
                                stack.push(next.clone());
                            } else {
                                stack.push(head);
                            }
                        },
                        _ => unreachable!(),
                    }
                },
                _ => {
                    stack.push(head);
                    if let Some(token) = tokens.next() {
                        stack.push(token.clone());
                    }
                }
            }
        }
    }
}

fn tokenise(expr: &str) -> Expr {
    let tokens = expr.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '(' => Token::Open,
            ')' => Token::Close,
            '+' => Token::Add,
            '*' => Token::Multiply,
            c => Token::Num(c.to_string().parse::<usize>().unwrap()),
        })
        .collect();
    
    Expr{tokens}
}

fn parse_input(s: &str) -> Vec<Expr> {
    s.split('\n')
        .filter(|l| !l.is_empty())
        .map(tokenise)
        .collect()
}

fn part1(exprs: Vec<Expr>) -> usize {
    exprs.iter().map(|expr| expr.eval1()).sum()
}

fn part2(exprs: Vec<Expr>) -> usize {
    exprs.iter().map(|expr| expr.eval2()).sum()
}

fn main() {
    let matches = App::new("AOC2020 Day18")
        .arg(Arg::with_name("input")
             .long("input")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("part")
             .long("part")
             .required(true)
             .takes_value(true))
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let path = fs::canonicalize(input_file).expect("file does not exist");
    let input = fs::read_to_string(path).expect("reading input");

    let exprs = parse_input(&input);


    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("{}", part1(exprs));
    } else {
        println!("{}", part2(exprs));
    }
}

#[cfg(test)]
mod tests {
    use crate::{Expr, Token, tokenise};

    #[test]
    fn tokenise_test() {
        let expr = "2 * 3 + (4 * 5)";
        assert_eq!(
            tokenise(expr), 
            Expr{tokens: vec![
                Token::Num(2),
                Token::Multiply,
                Token::Num(3),
                Token::Add,
                Token::Open,
                Token::Num(4),
                Token::Multiply,
                Token::Num(5),
                Token::Close,
            ]}
        )
    }

    #[test]
    fn expr_eval_test() {
        let expr = tokenise("2 * 3 + (4 * 5)");
        assert_eq!(expr.eval1(), 26);

        let expr = tokenise("1 + (2 * 3) + (4 * (5 + 6)");
        assert_eq!(expr.eval1(), 51);

        let expr = tokenise("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(expr.eval1(), 13632);

    }
}