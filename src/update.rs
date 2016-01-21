use modifier::{Modifier, Set};
use std::borrow::Cow;
use super::{
    util, Assignment, Assignments, Condition, Conditions, Query, Selection, Selections, Statement
};

pub struct Update<'a> {
    table: Cow<'static, str>,
    assignments: Assignments<'a>,
    conditions: Conditions<'a>,
    selections: Selections,
}

impl<'a> Update<'a> {
    pub fn new(table: Cow<'static, str>) -> Update<'a> {
        Update { table: table, assignments: vec![], conditions: vec![], selections: vec![] }
    }
}

impl<'a> Statement<'a> for Update<'a> {
    fn into_query(self) -> Query<'a> {
        if self.assignments.is_empty() {
            return Query(String::from("NULL;"), vec![]);
        }

        let mut query = format!("UPDATE {} SET ", self.table);
        query.push_str(&util::assignment_list(&self.assignments, 1));

        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            let offset = self.assignments.len() + 1;
            query.push_str(&util::condition_list(&self.conditions, offset));
        }

        if !self.selections.is_empty() {
            query.push_str(" RETURNING ");
            query.push_str(&util::keyword_list(self.selections.iter().map(|a| &a.0)));
        }

        query.push(';');

        let assignment_values = self.assignments.into_iter().map(|a| a.1);
        let condition_predicates = self.conditions.into_iter().map(|a| a.1);
        let condition_values = condition_predicates.flat_map(|p| p.into_values_iter());
        let values = assignment_values.chain(condition_values).collect::<Vec<_>>();

        Query(query, values)
    }
}

impl<'a> Set for Update<'a> { }

impl<'a> Modifier<Update<'a>> for Assignment<'a> {
    fn modify(self, update: &mut Update<'a>) {
        update.assignments.push(self);
    }
}

impl<'a> Modifier<Update<'a>> for Assignments<'a> {
    fn modify(mut self, update: &mut Update<'a>) {
        update.assignments.append(&mut self);
    }
}

impl<'a> Modifier<Update<'a>> for Condition<'a> {
    fn modify(self, update: &mut Update<'a>) {
        update.conditions.push(self);
    }
}

impl<'a> Modifier<Update<'a>> for Conditions<'a> {
    fn modify(mut self, update: &mut Update<'a>) {
        update.conditions.append(&mut self);
    }
}

impl<'a> Modifier<Update<'a>> for Selection {
    fn modify(self, update: &mut Update<'a>) {
        update.selections.push(self);
    }
}

impl<'a> Modifier<Update<'a>> for Selections {
    fn modify(mut self, update: &mut Update<'a>) {
        update.selections.append(&mut self);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn update() {
        use ::{Assignment, Condition, Predicate, Selection, Set, Statement};
        use super::*;

        let expected = "UPDATE test_table SET name = $1, age = $2 WHERE id = $3 RETURNING id;";

        let skyler = "Skyler";
        let age = 23;
        let id = 1;

        {
            let update = Update::new("test_table".into())
                .set(Assignment("name".into(), &skyler))
                .set(Assignment("age".into(), &age))
                .set(Condition("id".into(), Predicate::Equal(&id)))
                .set(Selection("id".into()));
            assert_eq!(update.into_query().0, expected);
        }
    }
}
