use modifier::{Modifier, Set};
use std::borrow::Cow;
use super::{util, Assignment, Assignments, Query, Selection, Selections, Statement};

pub struct Insert<'a> {
    table: Cow<'static, str>,
    assignments: Assignments<'a>,
    selections: Selections,
}

impl<'a> Insert<'a> {
    pub fn new(table: Cow<'static, str>) -> Insert<'a> {
        Insert { table: table, assignments: vec![], selections: vec![] }
    }
}

impl<'a> Statement<'a> for Insert<'a> {
    fn into_query(self) -> Query<'a> {
        let mut query = format!("INSERT INTO {} ", self.table);
        if self.assignments.is_empty() {
            query.push_str("DEFAULT VALUES");
        } else {
            query.push('(');
            query.push_str(&util::keyword_list(self.assignments.iter().map(|a| &a.0)));
            query.push_str(") VALUES (");
            query.push_str(&util::placeholders(1, self.assignments.len()));
            query.push(')');
        }

        if !self.selections.is_empty() {
            query.push_str(" RETURNING ");
            query.push_str(&util::keyword_list(self.selections.iter().map(|a| &a.0)));
        }

        query.push(';');

        Query(query, self.assignments.iter().map(|a| a.1).collect::<Vec<_>>())
    }
}

impl<'a> Set for Insert<'a> { }

impl<'a> Modifier<Insert<'a>> for Assignment<'a> {
    fn modify(self, insert: &mut Insert<'a>) {
        insert.assignments.push(self);
    }
}

impl<'a> Modifier<Insert<'a>> for Assignments<'a> {
    fn modify(mut self, insert: &mut Insert<'a>) {
        insert.assignments.append(&mut self);
    }
}

impl<'a> Modifier<Insert<'a>> for Selection {
    fn modify(self, insert: &mut Insert<'a>) {
        insert.selections.push(self);
    }
}

impl<'a> Modifier<Insert<'a>> for Selections {
    fn modify(mut self, insert: &mut Insert<'a>) {
        insert.selections.append(&mut self);
    }
}

#[cfg(test)]
mod tests {
    use ::{Assignment, Selection, Set, Statement};
    use super::*;

    #[test]
    fn insert() {
        let expected = "INSERT INTO test_table (name, age) VALUES ($1, $2) RETURNING id;";
        let skyler = "Skyler";
        let age = 23;

        {
            let insert = Insert::new("test_table".into())
                .set(Assignment("name".into(), &skyler))
                .set(Assignment("age".into(), &age))
                .set(Selection("id".into()));
            assert_eq!(insert.into_query().0, expected);
        }
    }

    #[test]
    fn default_insert() {
        let expected = "INSERT INTO test_table DEFAULT VALUES RETURNING id;";

        {
            let insert = Insert::new("test_table".into()).set(Selection("id".into()));
            assert_eq!(insert.into_query().0, expected);
        }
    }
}
