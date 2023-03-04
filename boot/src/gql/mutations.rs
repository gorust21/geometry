use async_graphql::{Context,Object};
use crate::typings::GqlResult;
use crate::entity::{user, Entity as User};
use sea_orm::{ActiveModelTrait, ActiveValue, Database,DatabaseConnection, DbErr, EntityTrait, ModelTrait};
// use rbatis::crud;
// use rbatis::rbatis::Rbatis;
// use serde::{Serialize, Deserialize};
// extern crate rbdc;
// use rbatis::rbdc::datetime::FastDateTime;

// const  DATABASE_URL: &str = "mysql://root:GciOiJIUzI1NiIsInR5cCI6IkpXVCJ9@localhost:3306/demo";


pub struct MutationRoot;


// #[derive(Debug, PartialEq, Eq, Clone)]
// struct Payment {
//     customer_id: i32,
//     amount: i32,
//     account_name: Option<String>,
// }


// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct User {
//     id: i32,
//     name: String,
//     age: i32,
//     //create_at :u32
// }

// crud!(User{});

#[Object]
impl MutationRoot {
    pub async fn add(&self,a: usize, b: usize) -> GqlResult<usize> {     
        Ok(a+b)//Ok(0usize)
    }

    pub async fn create_user(&self, ctx: &Context<'_>, username:String) -> Result<user::Model,DbErr> {
        //println!("name is {}",username);
        let conn = ctx.data::<DatabaseConnection>().unwrap();

        let user = user::ActiveModel {
            id:ActiveValue::Set(111),
            username: ActiveValue::Set(username),
            ..Default::default()
        };

        user.insert(conn).await
        //Ok(())
    }


}

// async fn get_conn()->DatabaseConnection{
//     let conn = Database::connect("mysql://root:GciOiJIUzI1NiIsInR5cCI6IkpXVCJ9@localhost:3306/demo").await.unwrap(); 
//     conn
// }

// pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    // let db = Database::connect(DATABASE_URL).await?;

    // let db = match db.get_database_backend() {
    //     DbBackend::MySql => {
    //         db.execute(Statement::from_string(
    //             db.get_database_backend(),
    //             format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
    //         ))
    //         .await?;

    //         let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    //         Database::connect(&url).await?
    //     }
    //     DbBackend::Postgres => {
    //         db.execute(Statement::from_string(
    //             db.get_database_backend(),
    //             format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
    //         ))
    //         .await?;
    //         db.execute(Statement::from_string(
    //             db.get_database_backend(),
    //             format!("CREATE DATABASE \"{}\";", DB_NAME),
    //         ))
    //         .await?;

    //         let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    //         Database::connect(&url).await?
    //     }
    //     DbBackend::Sqlite => db,
    // };

    // Ok(db)
// }