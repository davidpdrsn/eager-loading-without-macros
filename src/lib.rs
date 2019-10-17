#![allow(unused_imports, unused_variables)]

#[macro_use]
extern crate diesel;

mod models;

use diesel::{prelude::*, result::Error as DieselError};
use juniper::{Executor, FieldResult};
use juniper_eager_loading::LoadResult;
use juniper_eager_loading::{
    EagerLoadAllChildren, EagerLoadChildrenOfType, GraphqlNodeForModel, HasOne, LoadFrom
};
use juniper_from_schema::graphql_schema;
use models::{countries, users};

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

pub struct Context {
    db: PgConnection,
}

impl juniper::Context for Context {}

#[derive(Debug)]
pub struct Query;

impl QueryFields for Query {
    fn field_users(
        &self,
        exec: &Executor<'_, Context>,
        trail: &QueryTrail<'_, User, Walked>,
    ) -> FieldResult<Vec<User>> {
        let db = &exec.context().db;
        let user_models = users::table.load::<models::User>(db)?;
        let mut users = User::from_db_models(&user_models);
        User::eager_load_all_children_for_each(&mut users, &user_models, db, trail)?;

        Ok(users)
    }
}

#[derive(Debug)]
pub struct User {
    user: models::User,
    country: HasOne<Country>,
}

impl GraphqlNodeForModel for User {
    type Model = models::User;
    type Id = i32;
    type Connection = PgConnection;
    type Error = DieselError;

    fn new_from_model(model: &models::User) -> Self {
        User {
            user: model.clone(),
            country: HasOne::default(),
        }
    }
}

impl EagerLoadAllChildren for User {
    fn eager_load_all_children_for_each(
        nodes: &mut [User],
        models: &[models::User],
        db: &PgConnection,
        trail: &QueryTrail<User, Walked>,
    ) -> Result<(), DieselError> {
        if let Some(country_trail) = trail.country().walk() {
            User::eager_load_children(nodes, models, db, &country_trail)?;
        }

        Ok(())
    }
}

struct CountryForUserContext;

impl EagerLoadChildrenOfType<Country, CountryForUserContext, ()> for User {
    type ChildId = i32;

    fn child_ids(
        users: &[models::User],
        db: &Self::Connection,
    ) -> Result<LoadResult<Self::ChildId, (models::Country, ())>, Self::Error> {
        let country_ids = users.iter().map(|user| user.country_id).collect();
        Ok(LoadResult::Ids(country_ids))
    }

    fn load_children(
        ids: &[i32],
        db: &PgConnection,
    ) -> Result<Vec<models::Country>, DieselError> {
        LoadFrom::load(ids, db)
    }

    fn is_child_of(user: &User, (country, ()): &(Country, &())) -> bool {
        user.user.country_id == country.country.id
    }

    fn loaded_child(user: &mut User, country: Country) {
        user.country.loaded(country)
    }

    fn assert_loaded_otherwise_failed(user: &mut Self) {
        user.country.assert_loaded_otherwise_failed()
    }
}

impl UserFields for User {
    fn field_country(
        &self,
        exec: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Country, Walked>,
    ) -> FieldResult<&Country> {
        Ok(self.country.try_unwrap()?)
    }
}

#[derive(Debug, Clone)]
pub struct Country {
    country: models::Country,
}

impl GraphqlNodeForModel for Country {
    type Model = models::Country;
    type Id = i32;
    type Connection = PgConnection;
    type Error = DieselError;

    fn new_from_model(model: &models::Country) -> Self {
        Country {
            country: model.clone(),
        }
    }
}

impl EagerLoadAllChildren for Country {
    fn eager_load_all_children_for_each(
        nodes: &mut [Country],
        models: &[models::Country],
        db: &PgConnection,
        trail: &QueryTrail<Country, Walked>,
    ) -> Result<(), DieselError> {
        Ok(())
    }
}
impl CountryFields for Country {
    fn field_id(&self, exec: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.country.id)
    }
}
