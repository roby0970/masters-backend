
use serde::{Deserialize, Serialize};
use crate::schema::spaces;
use diesel::prelude::*;
use crate::db;
use crate::error_handler::CustomError;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RawLocation {
    pub x: f64,
    pub y: f64,
}
#[derive(Deserialize, Serialize, AsChangeset, Insertable)]
#[table_name = "spaces"]
pub struct Space {
    
    pub title: String,
    pub area: i32,
    pub longitude: f64,
    pub latitude: f64,
}
#[derive(Deserialize, Serialize, Debug, Queryable, Clone, Insertable)]
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
    pub fn find_closest(x: f64, y: f64) ->  Result<Self, CustomError> {
        let conn = db::connection()?;
        let spaces = spaces::table.load::<Spaces>(&conn)?;
        if spaces.len() > 0 {
            let mut closest = spaces[0].clone();
            let closest_distance = distance(closest.longitude, x , closest.latitude, y );
            for s in spaces {
               if distance(s.longitude, x, s.latitude, y ) < closest_distance {
                   closest = s;
               }
            }
            return Ok(closest)
        }
        

        Err(CustomError{error_status_code: 1, error_message : String::from("None found")})
    }
    
}
impl Space {
    fn from(space: Space) -> Space {
        Space { title: space.title, area: space.area, longitude: space.longitude,  latitude: space.latitude }
    }
}
pub fn distance(x1: f64, x2: f64, y1 : f64, y2: f64,) -> f64 {
    f64::sqrt(f64::powi(x2 - x1, 2) + f64::powi(y2 - y1, 2))
}