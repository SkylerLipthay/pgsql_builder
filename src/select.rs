use modifier::{Modifier, Set};
use std::borrow::Cow;
use super::{
    util, Condition, Conditions, Limit, Offset, Order, Orders, Query, Selection, Selections,
    Statement
};

pub struct Select<'a> {
    table: Cow<'static, str>,
    conditions: Conditions<'a>,
    selections: Selections,
    orders: Orders,
    limit: Option<Limit>,
    offset: Option<Offset>,
}

impl<'a> Select<'a> {
    pub fn new(table: Cow<'static, str>) -> Select<'a> {
        Select {
            table: table,
            conditions: vec![],
            selections: vec![],
            orders: vec![],
            limit: None,
            offset: None
        }
    }
}

impl<'a> Statement<'a> for Select<'a> {
    fn into_query(self) -> Query<'a> {
        let mut query = String::from("SELECT ");

        if !self.selections.is_empty() {
            query.push_str(&util::keyword_list(self.selections.iter().map(|a| &a.0)));
        } else {
            query.push('*');
        }

        query.push_str(&format!(" FROM {}", self.table));

        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&util::condition_list(&self.conditions, 1));
        }

        if !self.orders.is_empty() {
            query.push_str(" ORDER BY ");
            query.push_str(&util::order_list(&self.orders));
        }

        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        query.push(';');

        let condition_predicates = self.conditions.into_iter().map(|a| a.1);
        let condition_values = condition_predicates.flat_map(|p| p.into_values_iter());
        let values = condition_values.collect::<Vec<_>>();

        Query(query, values)
    }
}

impl<'a> Set for Select<'a> { }

impl<'a> Modifier<Select<'a>> for Condition<'a> {
    fn modify(self, select: &mut Select<'a>) {
        select.conditions.push(self);
    }
}

impl<'a> Modifier<Select<'a>> for Conditions<'a> {
    fn modify(mut self, select: &mut Select<'a>) {
        select.conditions.append(&mut self);
    }
}

impl<'a> Modifier<Select<'a>> for Selection {
    fn modify(self, select: &mut Select<'a>) {
        select.selections.push(self);
    }
}

impl<'a> Modifier<Select<'a>> for Selections {
    fn modify(mut self, select: &mut Select<'a>) {
        select.selections.append(&mut self);
    }
}

impl<'a> Modifier<Select<'a>> for Order {
    fn modify(self, select: &mut Select<'a>) {
        select.orders.push(self);
    }
}

impl<'a> Modifier<Select<'a>> for Orders {
    fn modify(mut self, select: &mut Select<'a>) {
        select.orders.append(&mut self);
    }
}

impl<'a> Modifier<Select<'a>> for Limit {
    fn modify(self, select: &mut Select<'a>) {
        select.limit = Some(self);
    }
}

impl<'a> Modifier<Select<'a>> for Offset {
    fn modify(self, select: &mut Select<'a>) {
        select.offset = Some(self);
    }
}

#[cfg(test)]
mod tests {
    use ::{Condition, Limit, Offset, Predicate, Selection, Set, Statement};
    use super::*;

    #[test]
    fn select() {
        let expected = "SELECT name FROM test_table WHERE id = $1 AND xyz < $2;";

        let id = 1;

        {
            let select = Select::new("test_table".into())
                .set(Selection("name".into()))
                .set(Condition("id".into(), Predicate::Equal(&id)))
                .set(Condition("xyz".into(), Predicate::Less(&id)));
            assert_eq!(select.into_query().0, expected);
        }
    }

    #[test]
    fn select_all() {
        let expected = "SELECT * FROM test_table WHERE id = $1 LIMIT 20 OFFSET 5;";

        let id = 1;

        {
            let select = Select::new("test_table".into())
                .set(Condition("id".into(), Predicate::Equal(&id)))
                .set(Limit(20))
                .set(Offset(5));
            assert_eq!(select.into_query().0, expected);
        }
    }
}
