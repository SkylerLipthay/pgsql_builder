use modifier::{Modifier, Set};
use std::borrow::Cow;
use super::{util, Condition, Conditions, Query, Statement};

pub struct Delete<'a> {
    table: Cow<'static, str>,
    conditions: Conditions<'a>,
}

impl<'a> Delete<'a> {
    pub fn new(table: Cow<'static, str>) -> Delete<'a> {
        Delete { table: table, conditions: vec![] }
    }
}

impl<'a> Statement<'a> for Delete<'a> {
    fn into_query(self) -> Query<'a> {
        let mut query = format!("DELETE FROM {}", self.table);

        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&util::condition_list(&self.conditions, 1));
        }

        query.push(';');

        let condition_predicates = self.conditions.into_iter().map(|a| a.1);
        let condition_values = condition_predicates.flat_map(|p| p.into_values_iter());
        let values = condition_values.collect::<Vec<_>>();

        Query(query, values)
    }
}

impl<'a> Set for Delete<'a> { }

impl<'a> Modifier<Delete<'a>> for Condition<'a> {
    fn modify(self, delete: &mut Delete<'a>) {
        delete.conditions.push(self);
    }
}

impl<'a> Modifier<Delete<'a>> for Conditions<'a> {
    fn modify(mut self, delete: &mut Delete<'a>) {
        // TODO: Can we just swap out self.conditions if `self.conditions.is_empty()`? This could
        // be applied to all other `Vec`-like modifiers.
        delete.conditions.append(&mut self);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn delete() {
        use ::{Condition, Predicate, Set, Statement};
        use super::*;

        let expected = "DELETE FROM test_table WHERE id = $1;";

        let id = 1;

        {
            let delete = Delete::new("test_table".into())
                .set(Condition("id".into(), Predicate::Equal(&id)));
            assert_eq!(delete.into_query().0, expected);
        }
    }
}
