#![allow(dead_code)]
use strum_macros::EnumIter;

#[derive(Clone, Debug, EnumIter, PartialEq)]
enum Operator {
    Add,
    Multiply,
    BitwiseOr,
}

#[derive(Clone, Debug, EnumIter, PartialEq)]
enum Precedence {
    LeftBindsTighter,
    RightBindsTighter,
    Ambiguous,
}

fn compare_precedence(left: &Operator, right: &Operator) -> Precedence {
    match left {
        Operator::Add => match right {
            Operator::Add => return Precedence::LeftBindsTighter,
            Operator::Multiply => return Precedence::RightBindsTighter,
            Operator::BitwiseOr => return Precedence::Ambiguous,
        },
        Operator::Multiply => match right {
            Operator::Add => return Precedence::LeftBindsTighter,
            Operator::Multiply => return Precedence::LeftBindsTighter,
            Operator::BitwiseOr => return Precedence::Ambiguous,
        },
        Operator::BitwiseOr => match right {
            Operator::Add => return Precedence::Ambiguous,
            Operator::Multiply => return Precedence::Ambiguous,
            Operator::BitwiseOr => return Precedence::RightBindsTighter,
        },
    }
}

#[cfg(test)]
mod test_precedence {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_operator_precedence() {
        let ops = Operator::iter();
        for a in ops.clone() {
            for b in ops.clone() {
                let ab = compare_precedence(&a, &b);
                let ba = compare_precedence(&b, &a);

                if ab == Precedence::Ambiguous {
                    assert_eq!(ba, Precedence::Ambiguous);
                }
                if a != b && ab == Precedence::LeftBindsTighter {
                    assert_eq!(ba, Precedence::RightBindsTighter);
                }
                if a != b && ab == Precedence::RightBindsTighter {
                    assert_eq!(ba, Precedence::LeftBindsTighter);
                }

                for c in ops.clone() {
                    let bc = compare_precedence(&b, &c);
                    let ac = compare_precedence(&a, &c);

                    // transitive
                    if ab == Precedence::LeftBindsTighter && bc == Precedence::LeftBindsTighter {
                        assert_eq!(ac, Precedence::LeftBindsTighter);
                    }
                    if ab == Precedence::RightBindsTighter && bc == Precedence::RightBindsTighter {
                        assert_eq!(ac, Precedence::RightBindsTighter);
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
            (Expr::BinaryOp(a), Expr::BinaryOp(b)) => {
                a == b
            },
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
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
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
            },
            Token::OpenParen => {
                let expr = self.parse_expr_outer(None)?;
                let token = self.next_token().expect("Expected close paren");
                if token != Token::CloseParen {
                    return Err("Expected close paren".to_string());
                }
                return Ok(expr);
            },
            _ => {
                return Err(format!("Expected number or open paren, got {:?}", token));
            },
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
                Precedence::RightBindsTighter
            };
            match precedence {
                Precedence::LeftBindsTighter => {
                    self.position = start;
                    return Ok(left);
                },
                Precedence::RightBindsTighter => {
                    let right = self.parse_expr_outer(Some(op.clone()))?;
                    let new_left = Expr::BinaryOp(BinaryOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    });
                    left = new_left;
                },
                Precedence::Ambiguous => return Err("Ambiguous operator precedence".to_string()),
            }
        }
    }
    fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr_outer(None)?;
        if parser.position != parser.tokens.len() {
            return Err("Expected end of expression".to_string());
        };
        Ok(expr)
    }
}
