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
