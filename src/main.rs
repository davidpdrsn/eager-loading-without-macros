#[macro_use]
extern crate diesel;

use juniper::{Executor, FieldResult};
use juniper_from_schema::graphql_schema;
use diesel::prelude::*;

graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        users: [User!]! @juniper(ownership: "owned")
    }

    type User {
        country: Country!
    }

    type Country {
        id: Int!
    }
}

table! {
    users {
        id -> Integer,
        country_id -> Integer,
    }
}

table! {
    countries {
        id -> Integer,
    }
}

mod models {
    #[derive(Queryable, Debug)]
    pub struct User {
        pub id: i32,
        pub country_id: i32,
    }

    #[derive(Queryable, Debug)]
    pub struct Country {
        pub id: i32,
    }
}

#[derive(Debug)]
pub struct Context;

impl juniper::Context for Context {}

#[derive(Debug)]
pub struct Query;

impl QueryFields for Query {
    fn field_users(
        &self,
        exec: &Executor<'_, Context>,
        trail: &QueryTrail<'_, User, Walked>,
    ) -> FieldResult<Vec<User>> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct User {
    user: models::User,
}

impl UserFields for User {
    fn field_country(
        &self,
        exec: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Country, Walked>,
    ) -> FieldResult<&Country> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Country {
    country: models::Country,
}

impl CountryFields for Country {
    fn field_id(&self, exec: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.country.id)
    }
}

fn main() {}
