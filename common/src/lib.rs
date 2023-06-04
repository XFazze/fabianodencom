pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use yew::prelude::*;

#[derive(Deserialize, PostgresMapper, Serialize, Properties, PartialEq, Clone)]
#[pg_mapper(table = "project_tag")] // singular 'user' is a keyword..
pub struct ProjectTag {
    pub name: String,
    pub color: String,
}

#[derive(Deserialize, PostgresMapper, Serialize, Properties, PartialEq, Clone)]
#[pg_mapper(table = "project")] // singular 'user' is a keyword..
pub struct Project {
    pub name: String,
    pub tags: Vec<String>,
    pub url: String,
    pub description: String,
}
