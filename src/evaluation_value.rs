use std::cmp::Ordering;

#[derive(Ord, Debug, PartialEq, Eq)]
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

impl PartialOrd for EvaluationValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (other, self) {
            (EvaluationValue::Eval(other_eval), EvaluationValue::Eval(self_eval)) => {
                let d = if other_eval > self_eval {
                    Ordering::Greater
                } else if other_eval < self_eval {
                    Ordering::Less
                } else {
                    Ordering::Equal
                };
                Some(d)
            }
            (EvaluationValue::Eval(other_eval), EvaluationValue::CheckMate(self_mate)) => {
                if *self_mate == true {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            (EvaluationValue::CheckMate(other_mate), EvaluationValue::Eval(self_eval)) => {
                if *other_mate == true {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
            (EvaluationValue::CheckMate(other_mate), EvaluationValue::CheckMate(self_mate)) => {
                if other_mate == self_mate {
                    Some(Ordering::Equal)
                } else if *other_mate == true {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
        }
    }
}
