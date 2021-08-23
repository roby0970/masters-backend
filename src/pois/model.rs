
use serde::{Deserialize, Serialize};
use crate::schema::pois;
use diesel::prelude::*;
use crate::db;
use crate::error_handler::CustomError;
#[derive(Deserialize, Serialize, AsChangeset, Insertable)]
#[table_name = "pois"]
pub struct Poi {
    
    pub title: String,
    pub idspace: i32
}
#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "pois"]
pub struct Pois {
    pub id: i32,
    pub title: String,
    pub idspace: i32
}
impl Pois {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let pois = pois::table.load::<Pois>(&conn)?;
        Ok(pois)
    }
    pub fn find_by_space_id(idspace: i32) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let pois = pois::table.filter(pois::idspace.eq(idspace)).load(&conn)?;
        Ok(pois)
    }
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let poi = pois::table.filter(pois::id.eq(id)).first(&conn)?;
        Ok(poi)
    }
    pub fn create(poi: Poi) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let poi = Poi::from(poi);
        let poi = diesel::insert_into(pois::table)
            .values(poi)
            .get_result(&conn)?;
        Ok(poi)
    }
    pub fn update(id: i32, poi: Poi) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let poi = diesel::update(pois::table)
            .filter(pois::id.eq(id))
            .set(poi)
            .get_result(&conn)?;
        Ok(poi)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(pois::table.filter(pois::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl Poi {
    fn from(poi: Poi) -> Poi {
        Poi { title: poi.title, idspace: poi.idspace }
    }
}