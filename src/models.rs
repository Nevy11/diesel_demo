use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};

use crate::schema::posts;

#[derive(Queryable, Selectable)] // caused by the selectable
#[diesel(table_name = crate::schema::posts)] // constructs a matching select based on this
#[diesel(check_for_backend(diesel::pg::Pg))] // checks to see if all the fields are compatible in compile time
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
