use ::{Assignments, Conditions, Orders};
use itertools::Itertools;
use std::borrow::Cow;

pub fn escape(keyword: &str) -> String {
    let mut result = keyword.replace("\"", "\\\"");
    result.insert(0, '"');
    result.push('"');
    result
}

#[doc(hidden)]
pub fn keyword_list<'a, I: Iterator<Item=&'a Cow<'static, str>>>(keywords: I) -> String {
    keywords.map(|i| i.to_owned()).join(", ")
}

#[doc(hidden)]
pub fn assignment_list(assignments: &Assignments, offset: usize) -> String {
    assignments.iter()
        .map(|a| &a.0)
        .enumerate()
        .map(|(i, n)| format!("{} = ${}", n, offset + i))
        .join(", ")
}

#[doc(hidden)]
pub fn condition_list(conditions: &Conditions, mut offset: usize) -> String {
    conditions.iter()
        .map(|c| format!("{} {}", c.0, c.1.to_placeholder_string(&mut offset)))
        .join(" AND ")
}

#[doc(hidden)]
pub fn order_list(orders: &Orders) -> String {
    orders.iter()
        .map(|o| format!("{} {}", o.0, o.1))
        .join(", ")
}

#[doc(hidden)]
pub fn placeholders(offset: usize, n: usize) -> String {
    (offset..offset+n).map(|i| format!("${}", i)).join(", ")
}
