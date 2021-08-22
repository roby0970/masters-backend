
use serde::{Deserialize, Serialize};
use crate::schema::coordinates;
use diesel::prelude::*;
use crate::db;
use crate::error_handler::CustomError;
#[derive(Deserialize, Serialize, AsChangeset, Insertable)]
#[table_name = "coordinates"]
pub struct Coordinate {
    
    pub x: i32,
    pub y: i32,
    pub idspace: i32,
    pub idpoi: i32,
    pub blocked: bool,
}
#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "coordinates"]
pub struct Coordinates {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub idspace: i32,
    pub idpoi: i32,
    pub blocked: bool,
}
impl Coordinates {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let coordinates = coordinates::table.load::<Coordinates>(&conn)?;
        Ok(coordinates)
    }
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let coordinate = coordinates::table.filter(coordinates::id.eq(id)).first(&conn)?;
        Ok(coordinate)
    }
    pub fn create(coordinate: Coordinate) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let coordinate = Coordinate::from(coordinate);
        let coordinate = diesel::insert_into(coordinates::table)
            .values(coordinate)
            .get_result(&conn)?;
        Ok(coordinate)
    }
    pub fn update(id: i32, coordinate: Coordinate) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let coordinate = diesel::update(coordinates::table)
            .filter(coordinates::id.eq(id))
            .set(coordinate)
            .get_result(&conn)?;
        Ok(coordinate)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(coordinates::table.filter(coordinates::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl Coordinate {
    fn from(coordinate: Coordinate) -> Coordinate {
        Coordinate { x: coordinate.x, y: coordinate.y,  idspace: coordinate.idspace, idpoi : coordinate.idpoi, blocked: coordinate.blocked }
    }
}