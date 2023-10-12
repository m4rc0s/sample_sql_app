use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::project)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: uuid::Uuid,
    pub title: String,
    pub status: String,
}
