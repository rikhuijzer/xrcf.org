#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
    BitwiseOr,
}

#[derive(Clone, Debug, PartialEq)]
enum Precedence {
    LeftBindsTighter,
    RightBindsTighter,
    Ambiguous,
}

use Precedence::*;

fn compare_precedence(left: &Operator, right: &Operator) -> Precedence {
    match left {
        Operator::Add => match right {
            Operator::Add => return LeftBindsTighter,
            Operator::Multiply => return RightBindsTighter,
            Operator::BitwiseOr => return Ambiguous,
        },
        Operator::Multiply => match right {
            Operator::Add => return LeftBindsTighter,
            Operator::Multiply => return LeftBindsTighter,
            Operator::BitwiseOr => return Ambiguous,
        },
        Operator::BitwiseOr => match right {
            Operator::Add => return Ambiguous,
            Operator::Multiply => return Ambiguous,
            Operator::BitwiseOr => return RightBindsTighter,
        },
    }
}

#[cfg(test)]
mod test_precedence {
    use super::*;

    #[test]
    fn test_operator_precedence() {
        let ops = vec![Operator::Add, Operator::Multiply, Operator::BitwiseOr];
        for a in ops.clone() {
            for b in ops.clone() {
                let ab = compare_precedence(&a, &b);
                let ba = compare_precedence(&b, &a);

                if ab == Ambiguous {
                    assert_eq!(ba, Ambiguous);
                }
                if a != b && ab == LeftBindsTighter {
                    assert_eq!(ba, RightBindsTighter);
                }
                if a != b && ab == RightBindsTighter {
                    assert_eq!(ba, LeftBindsTighter);
                }

                for c in ops.clone() {
                    let bc = compare_precedence(&b, &c);
                    let ac = compare_precedence(&a, &c);

                    // transitive
                    if ab == LeftBindsTighter && bc == LeftBindsTighter {
                        assert_eq!(ac, LeftBindsTighter);
                    }
                    if ab == RightBindsTighter && bc == RightBindsTighter {
                        assert_eq!(ac, RightBindsTighter);
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Number,
    Add,
    Multiply,
    BitwiseOr,
    OpenParen,
    CloseParen,
}

#[derive(Clone, Debug, PartialEq)]
struct BinaryOp {
    op: Operator,
    left: Box<Expr>,
    right: Box<Expr>,
}

#[derive(Clone, Debug)]
enum Expr {
    Number,
    BinaryOp(BinaryOp),
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Number, Expr::Number) => true,
            (Expr::BinaryOp(a), Expr::BinaryOp(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            position: 0,
        }
    }
    fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.tokens.len() {
            return None;
        }
        let token = self.tokens[self.position].clone();
        self.position += 1;
        Some(token)
    }
    fn parse_expr_inner(&mut self) -> Result<Expr, String> {
        let token = self.next_token().expect("Expected start of expression");
        match token {
            Token::Number => {
                return Ok(Expr::Number);
            }
            Token::OpenParen => {
                let expr = self.parse_expr_outer(None)?;
                let token = self.next_token().expect("Expected close paren");
                if token != Token::CloseParen {
                    return Err("Expected close paren".to_string());
                }
                return Ok(expr);
            }
            _ => {
                return Err(format!("Expected number or open paren, got {:?}", token));
            }
        }
    }
    fn parse_expr_outer(&mut self, prev_op_o: Option<Operator>) -> Result<Expr, String> {
        let mut left = self.parse_expr_inner()?;
        loop {
            let start = self.position;
            let token = match self.next_token() {
                Some(token) => token,
                None => return Ok(left),
            };
            let op = match token {
                Token::Add => Operator::Add,
                Token::Multiply => Operator::Multiply,
                Token::BitwiseOr => Operator::BitwiseOr,
                _ => {
                    self.position = start;
                    return Ok(left);
                }
            };
            let precedence = if let Some(ref prev_op) = prev_op_o {
                compare_precedence(&prev_op, &op)
            } else {
                RightBindsTighter
            };
            match precedence {
                LeftBindsTighter => {
                    self.position = start;
                    return Ok(left);
                }
                RightBindsTighter => {
                    let right = self.parse_expr_outer(Some(op.clone()))?;
                    let new_left = Expr::BinaryOp(BinaryOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    });
                    left = new_left;
                }
                Ambiguous => return Err("Ambiguous operator precedence".to_string()),
            }
        }
    }
    fn parse(tokens: &[Token]) -> Result<Expr, String> {
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr_outer(None)?;
        if parser.position != parser.tokens.len() {
            return Err("Expected end of expression".to_string());
        };
        Ok(expr)
    }
}

#[cfg(test)]
mod test_parser {
    use super::Token::*;
    use super::*;

    #[test]
    fn test_multiply_precedence_over_add() {
        assert_eq!(
            Parser::parse(&vec![Number, Add, Number, Multiply, Number]),
            Parser::parse(&vec![
                Number, Add, OpenParen, Number, Multiply, Number, CloseParen
            ])
        );
    }
    #[test]
    fn test_parens_override_precedence() {
        assert_eq!(
            Parser::parse(&vec![
                OpenParen, Number, Add, Number, CloseParen, Multiply, Number
            ]),
            Ok(Expr::BinaryOp(BinaryOp {
                op: Operator::Multiply,
                left: Box::new(Expr::BinaryOp(BinaryOp {
                    op: Operator::Add,
                    left: Box::new(Expr::Number),
                    right: Box::new(Expr::Number),
                })),
                right: Box::new(Expr::Number),
            }))
        );
    }
    #[test]
    fn test_ambiguous_precedence_against_bitwise_or() {
        assert_eq!(
            Parser::parse(&vec![Number, Add, Number, BitwiseOr, Number]),
            Err("Ambiguous operator precedence".to_string())
        );
        assert_eq!(
            Parser::parse(&vec![Number, Multiply, Number, BitwiseOr, Number]),
            Err("Ambiguous operator precedence".to_string())
        );
    }
    #[test]
    fn test_left_associative() {
        assert_eq!(
            Parser::parse(&vec![Number, Add, Number, Add, Number]),
            Parser::parse(&vec![
                OpenParen, Number, Add, Number, CloseParen, Add, Number
            ])
        );
    }
    #[test]
    fn test_right_associative() {
        assert_eq!(
            Parser::parse(&vec![Number, BitwiseOr, Number, BitwiseOr, Number]),
            Parser::parse(&vec![
                Number, BitwiseOr, OpenParen, Number, BitwiseOr, Number, CloseParen
            ])
        );
    }
}
