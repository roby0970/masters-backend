
use serde::{Deserialize, Serialize};
use crate::schema::bles;
use diesel::prelude::*;
use crate::db;
use crate::error_handler::CustomError;
#[derive(Deserialize, Serialize, AsChangeset, Debug, Insertable)]
#[table_name = "bles"]
pub struct Ble {
    
    pub title: String,
    pub idspace: i32,
}
#[derive(Deserialize, Serialize,  Debug,Queryable, Insertable)]
#[table_name = "bles"]
pub struct Bles {
    pub id: i32,
    pub title: String,
    pub idspace: i32,
}
impl Bles {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let bles = bles::table.load::<Bles>(&conn)?;
        Ok(bles)
    }
    pub fn find_by_space_id(idspace: i32) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let bles = bles::table.filter(bles::idspace.eq(idspace)).load(&conn)?;
        Ok(bles)
    }
    
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let ble = bles::table.filter(bles::id.eq(id)).first(&conn)?;
        Ok(ble)
    }
    pub fn create(ble: Ble) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let ble = Ble::from(ble);
        let ble = diesel::insert_into(bles::table)
            .values(ble)
            .get_result(&conn)?;
        Ok(ble)
    }
    pub fn update(id: i32, ble: Ble) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let ble = diesel::update(bles::table)
            .filter(bles::id.eq(id))
            .set(ble)
            .get_result(&conn)?;
        Ok(ble)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(bles::table.filter(bles::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl Ble {
    fn from(poi: Ble) -> Ble {
        Ble { title: poi.title, idspace: poi.idspace }
    }
}