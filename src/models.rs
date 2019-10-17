use diesel::{prelude::*, result::Error as DieselError};
use juniper_eager_loading::LoadFrom;

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

#[derive(Queryable, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub country_id: i32,
}

#[derive(Queryable, Debug, Clone)]
pub struct Country {
    pub id: i32,
}

impl LoadFrom<i32> for Country {
    type Error = DieselError;
    type Connection = PgConnection;

    fn load(ids: &[i32], db: &Self::Connection) -> Result<Vec<Self>, Self::Error> {
        countries::table.filter(countries::id.eq_any(ids)).load(db)
    }
}
