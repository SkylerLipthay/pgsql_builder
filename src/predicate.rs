use postgres::types::ToSql;
use std::borrow::Cow;
use self::Predicate::*;
use super::util;

#[derive(Debug)]
pub enum Predicate<'a> {
    Equal(&'a ToSql),
    NotEqual(&'a ToSql),
    Greater(&'a ToSql),
    GreaterOrEqual(&'a ToSql),
    Less(&'a ToSql),
    LessOrEqual(&'a ToSql),
    In(Cow<'a, [&'a ToSql]>),
    Between(&'a ToSql, &'a ToSql),
    Like(&'a ToSql),
    IsNull,
    IsNotNull,
}

impl<'a> Predicate<'a> {
    pub fn to_placeholder_string(&self, placeholder_idx: &mut usize) -> Cow<'static, str> {
        match *self {
            Equal(..) => {
                let result = format!("= ${}", placeholder_idx);
                *placeholder_idx += 1;
                result.into()
            },
            NotEqual(..) => {
                let result = format!("<> ${}", placeholder_idx);
                *placeholder_idx += 1;
                result.into()
            },
            Greater(..) => {
                let result = format!("> ${}", placeholder_idx);
                *placeholder_idx += 1;
                result.into()
            },
            GreaterOrEqual(..) => {
                let result = format!(">= ${}", placeholder_idx);
                *placeholder_idx += 1;
                result.into()
            },
            Less(..) => {
                let result = format!("< ${}", placeholder_idx);
                *placeholder_idx += 1;
                result.into()
            },
            LessOrEqual(..) => {
                let result = format!("<= ${}", placeholder_idx);
                *placeholder_idx += 1;
                result.into()
            },
            In(ref values) => {
                let count = values.len();
                let result = format!("IN ({})", util::placeholders(*placeholder_idx, count));
                *placeholder_idx += count;
                result.into()
            },
            Between(..) => {
                let result = format!("BETWEEN ${} AND ${}", placeholder_idx, *placeholder_idx + 1);
                *placeholder_idx += 2;
                result.into()
            },
            Like(..) => {
                let result = format!("LIKE ${}", placeholder_idx);
                *placeholder_idx += 1;
                result.into()
            },
            IsNull => "IS NULL".into(),
            IsNotNull => "IS NOT NULL".into(),
        }
    }

    pub fn into_values_iter(self) -> ValuesIter<'a> {
        ValuesIter { index: 0, inner: self }
    }
}

pub struct ValuesIter<'a> {
    index: usize,
    inner: Predicate<'a>,
}

impl<'a> Iterator for ValuesIter<'a> {
    type Item = &'a ToSql;

    fn next(&mut self) -> Option<&'a ToSql> {
        let result = match self.inner {
            Equal(value) => (&[value]).get(self.index).map(|v| *v),
            NotEqual(value) => (&[value]).get(self.index).map(|v| *v),
            Greater(value) => (&[value]).get(self.index).map(|v| *v),
            GreaterOrEqual(value) => (&[value]).get(self.index).map(|v| *v),
            Less(value) => (&[value]).get(self.index).map(|v| *v),
            LessOrEqual(value) => (&[value]).get(self.index).map(|v| *v),
            In(ref values) => values.get(self.index).map(|v| *v),
            Between(value_1, value_2) => (&[value_1, value_2]).get(self.index).map(|v| *v),
            Like(value) => (&[value]).get(self.index).map(|v| *v),
            IsNull => None,
            IsNotNull => None,
        };

        if result.is_some() {
            self.index += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use postgres::types::ToSql;
    use std::borrow::Cow;

    macro_rules! test_predicate {
        ($predicate:expr, $string:expr, $placeholder_increment:expr) => {
            {
                let mut index = 1;
                let previous_index = index;
                assert_eq!($predicate.to_placeholder_string(&mut index), $string);
                assert_eq!(previous_index + $placeholder_increment, index);
            }
        }
    }

    #[test]
    fn predicates() {
        let value = 0;
        let values: [&ToSql; 3] = [&0, &1, &2];

        test_predicate!(Predicate::Equal(&value), "= $1", 1);
        test_predicate!(Predicate::NotEqual(&value), "<> $1", 1);
        test_predicate!(Predicate::Greater(&value), "> $1", 1);
        test_predicate!(Predicate::GreaterOrEqual(&value), ">= $1", 1);
        test_predicate!(Predicate::Less(&value), "< $1", 1);
        test_predicate!(Predicate::LessOrEqual(&value), "<= $1", 1);
        test_predicate!(Predicate::In(Cow::Borrowed(&values)), "IN ($1, $2, $3)", 3);
        test_predicate!(Predicate::Between(&value, &value), "BETWEEN $1 AND $2", 2);
        test_predicate!(Predicate::Like(&value), "LIKE $1", 1);
        test_predicate!(Predicate::IsNull, "IS NULL", 0);
        test_predicate!(Predicate::IsNotNull, "IS NOT NULL", 0);
    }
}
