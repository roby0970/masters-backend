
use serde::{Deserialize, Serialize};
use crate::schema::spaces;
use diesel::prelude::*;
use crate::db;
use crate::error_handler::CustomError;
#[derive(Deserialize, Serialize, AsChangeset, Insertable)]
#[table_name = "spaces"]
pub struct Space {
    
    pub title: String,
    pub area: i32,
    pub longitude: f64,
    pub latitude: f64,
}
#[derive(Deserialize, Serialize, Debug, Queryable, Insertable)]
#[table_name = "spaces"]
pub struct Spaces {
    pub id: i32,
    pub title: String,
    pub area: i32,
    pub longitude: f64,
    pub latitude: f64,
}
impl Spaces {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let spaces = spaces::table.load::<Spaces>(&conn)?;
        Ok(spaces)
    }
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let space = spaces::table.filter(spaces::id.eq(id)).first(&conn)?;
        Ok(space)
    }
    pub fn create(space: Space) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let space = Space::from(space);
        let space = diesel::insert_into(spaces::table)
            .values(space)
            .get_result(&conn)?;
        Ok(space)
    }
    pub fn update(id: i32, space: Space) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let space = diesel::update(spaces::table)
            .filter(spaces::id.eq(id))
            .set(space)
            .get_result(&conn)?;
        Ok(space)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(spaces::table.filter(spaces::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl Space {
    fn from(space: Space) -> Space {
        Space { title: space.title, area: space.area, longitude: space.longitude,  latitude: space.latitude }
    }
}