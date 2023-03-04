use crate::typings::{GqlResult};
use crate::dbs::StarWars;
use crate::models::{Character};
use crate::utils::query_characters;
use async_graphql::{FieldResult, Error};
// use async_graphql::connection::{Connection, EmptyFields};
// use cherry::mysql::MySqlPool;
// use cherry::{Cherry, QueryExecutor};
// use cherry::clause::Where;
// use chrono::prelude::*;
// use::chrono::DateTime;
// use::chrono::Local;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

// #[derive(Cherry)]
// #[cherry(database = "mysql")]
// struct User {
//     id: i32,
//     name: String,
//     age: i32,
// }

pub async fn get_carrier_by_id(db: &StarWars, id: &str) -> GqlResult<Character> {
    if let Some(current_id) = db.carrier(id) {
        let carrier = db.chars[current_id].to_carrier().into();
        Ok(carrier)
    } else {
        Err(Error::new("carrier not exist"))
    }
}

pub async fn get_hero(db: &StarWars) -> GqlResult<Character> {
    // let pool = MySqlPool::connect("mysql://root:GciOiJIUzI1NiIsInR5cCI6IkpXVCJ9@localhost:3306/demo").await.unwrap();
    // let now:DateTime<Local> = Local::now();
    // let fmt = "%Y-%m-%d %H:%M:%S";
    // let user = User { id: 101, name: String::from("Larry"),age:20};
    // let result = user.insert().execute(&pool).await.unwrap();
    // println!("{:?}",result);  
    // let user = User::select().and_eq("id", 101).one(&pool).await?; 
    let name = "demo";
    let hero = db.chars[db.darth_vader].add_name_to_carrier(&name).into();
    Ok(hero)
}

// pub async fn get_carriers(
//     db: &StarWars,
//     after: Option<String>,
//     before: Option<String>,
//     first: Option<i32>,
//     last: Option<i32>,
// ) -> FieldResult<Connection<usize, Character, EmptyFields, EmptyFields>> {
//     let ids = db
//         .carriers()
//         .iter()
//         .copied()
//         .collect::<Vec<usize>>();

//     query_characters(after, before, first, last, &ids)
//         .await
//         .map(|conn| {
//             conn.map_node(|item| {
//                 db.chars[item].to_carrier().into()
//             })
//         })
// }