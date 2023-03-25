use async_graphql::{Context,Object};
use chrono::prelude::*;
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

#[Object]
impl MutationRoot {
    pub async fn add(&self,a: usize, b: usize) -> GqlResult<usize> {     
        Ok(a+b)//Ok(0usize)
    }

    pub async fn create_user(&self, ctx: &Context<'_>, username:String) -> Result<user::Model,DbErr> {
        //println!("name is {}",username);
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let local: DateTime<Local> = Local::now();
        let user = user::ActiveModel {
            id:ActiveValue::Set(local.timestamp_micros()),
            username: ActiveValue::Set(username),
            ..Default::default()
        };
        user.insert(conn).await
        //Ok(())
    }


}