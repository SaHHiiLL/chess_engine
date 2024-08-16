use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EvaluationValue {
    Eval(isize),
    // bool represents if the who has been checkmate
    // false is opp and true is engine side;
    CheckMate(bool),
}

impl From<isize> for EvaluationValue {
    fn from(value: isize) -> Self {
        if value == isize::MAX {
            EvaluationValue::CheckMate(true)
        } else if value == -isize::MAX {
            EvaluationValue::CheckMate(false)
        } else {
            EvaluationValue::Eval(value)
        }
    }
}

impl Ord for EvaluationValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (other, self) {
            (EvaluationValue::Eval(other_eval), EvaluationValue::Eval(self_eval)) => {
                if other_eval > self_eval {
                    Ordering::Less
                } else if other_eval < self_eval {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (EvaluationValue::Eval(other_eval), EvaluationValue::CheckMate(self_mate)) => {
                if *self_mate == true {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (EvaluationValue::CheckMate(other_mate), EvaluationValue::Eval(self_eval)) => {
                if *other_mate == true {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (EvaluationValue::CheckMate(other_mate), EvaluationValue::CheckMate(self_mate)) => {
                if other_mate == self_mate {
                    Ordering::Equal
                } else if *other_mate == true {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    }
}

impl PartialOrd for EvaluationValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::EvaluationValue;

    #[test]
    fn test_eval_value() {
        let less = EvaluationValue::CheckMate(false);
        let more = EvaluationValue::CheckMate(true);
        assert!(more > less);
    }

    #[test]
    fn test_eval_value_1() {
        let less = EvaluationValue::Eval(-12837);
        let more = EvaluationValue::CheckMate(true);
        assert!(more > less);
    }
}
