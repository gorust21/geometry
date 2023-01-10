use crate::typings::{GqlResult};
use crate::dbs::StarWars;
use crate::models::{Character};
use crate::utils::query_characters;
use async_graphql::{FieldResult, Error};
use async_graphql::connection::{Connection, EmptyFields};

pub async fn get_carrier_by_id(db: &StarWars, id: &str) -> GqlResult<Character> {
    if let Some(current_id) = db.carrier(id) {
        let carrier = db.chars[current_id].to_carrier().into();
        Ok(carrier)
    } else {
        Err(Error::new("carrier not exist"))
    }
}

pub async fn get_hero(db: &StarWars) -> GqlResult<Character> {
    let hero = db.chars[db.darth_vader].to_carrier().into();
    Ok(hero)
}

pub async fn get_carriers(
    db: &StarWars,
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
) -> FieldResult<Connection<usize, Character, EmptyFields, EmptyFields>> {
    let ids = db
        .carriers()
        .iter()
        .copied()
        .collect::<Vec<usize>>();

    query_characters(after, before, first, last, &ids)
        .await
        .map(|conn| {
            // 将 usize 索引转为 Character
            conn.map_node(|item| {
                db.chars[item].to_carrier().into()
            })
        })
}