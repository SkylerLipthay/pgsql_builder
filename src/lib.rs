#[macro_use]
extern crate itertools;
extern crate modifier;
extern crate postgres;

use postgres::types::ToSql;
use std::borrow::Cow;
use std::fmt;

pub use self::delete::Delete;
pub use self::insert::Insert;
pub use self::predicate::Predicate;
pub use self::select::Select;
pub use self::update::Update;
pub use modifier::Set;
pub use util::escape;

mod delete;
mod insert;
mod predicate;
mod select;
mod update;
mod util;

#[derive(Debug)]
pub struct Assignment<'a>(pub Cow<'static, str>, pub &'a ToSql);
pub type Assignments<'a> = Vec<Assignment<'a>>;

#[derive(Debug)]
pub struct Condition<'a>(pub Cow<'static, str>, pub Predicate<'a>);
pub type Conditions<'a> = Vec<Condition<'a>>;

#[derive(Debug)]
pub struct Selection(pub Cow<'static, str>);
pub type Selections = Vec<Selection>;

#[derive(Debug)]
pub struct Order(pub Cow<'static, str>, pub Dir);
pub type Orders = Vec<Order>;

#[derive(Debug)]
pub struct Query<'a>(pub String, pub Vec<&'a ToSql>);

#[derive(Debug)]
pub struct Limit(pub usize);

impl fmt::Display for Limit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug)]
pub struct Offset(pub usize);

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

pub trait Statement<'a> {
    fn into_query(self) -> Query<'a>;
}

#[derive(Debug)]
pub enum Dir {
    Asc,
    Desc,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Dir::Asc => fmt::Display::fmt("ASC", f),
            Dir::Desc => fmt::Display::fmt("DESC", f),
        }
    }
}
