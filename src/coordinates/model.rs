
use serde::{Deserialize, Serialize};
use crate::schema::coordinates;
use diesel::prelude::*;
use crate::db;
use crate::error_handler::CustomError;
use pathfinding::prelude::{absdiff};
#[derive(Deserialize, Serialize,  Debug, AsChangeset, Insertable, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[table_name = "coordinates"]
pub struct Coordinate {
    
    pub x: i32,
    pub y: i32,
    pub idspace: i32,
    pub idpoi: i32,
    pub blocked: bool,
    pub wallup: bool,
    pub wallright: bool,
    pub walldown: bool,
    pub wallleft: bool,

}
#[derive(Deserialize, Serialize,  Debug,Queryable, Insertable, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[table_name = "coordinates"]
pub struct Coordinates {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub idspace: i32,
    pub idpoi: i32,
    pub blocked: bool,
    pub wallup: bool,
    pub wallright: bool,
    pub walldown: bool,
    pub wallleft: bool,
}
impl Coordinates {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let coordinates = coordinates::table.load::<Coordinates>(&conn)?;
        Ok(coordinates)
    }
    pub fn find_by_space_id(idspace: i32) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let coordinates = coordinates::table.filter(coordinates::idspace.eq(idspace)).load(&conn)?;
        Ok(coordinates)
    }
    pub fn find_by_poi_id(idpoi: i32) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let coordinates = coordinates::table.filter(coordinates::idpoi.eq(idpoi)).load(&conn)?;
        Ok(coordinates)
    }
    pub fn find_by_space_id_and_xy(idspace: i32, x:i32, y:i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let coordinates = coordinates::table.filter(coordinates::idspace.eq(idspace)).filter(coordinates::x.eq(x)).filter(coordinates::y.eq(y)).first(&conn)?;
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

    pub fn distance(&self, other: &Coordinates) -> u32 {
        (absdiff(self.x, other.x) + absdiff(self.y, other.y)) as u32
    }
    pub fn successors(&self, coords: Vec<Coordinates>) -> Vec<(Coordinates, u32)> {
        let &Coordinates {
            id: _,
            x,
            y,
            blocked: _,
            idspace: _,
            idpoi: _,
            wallup: _,
            
            wallright: _,
            walldown: _,
            wallleft: _,
            
        } = self;
        
        let left = coords.iter().find(|&c| c.x == x - 1 && c.y == y).clone();
        let top = coords.iter().find(|&c| c.x == x && c.y == y + 1).clone();
        let right = coords.iter().find(|&c| c.x == x + 1 && c.y == y).clone();
        let down = coords.iter().find(|&c| c.x == x && c.y == y - 1).clone();

        let mut succ: Vec<Coordinates> = vec![];
        match left {
            Some(c) => {
                if c.blocked == false && self.wallleft == false {
                    succ.push(c.clone());
                }
            }
            None => {}
        }
        match top {
            Some(c) => {
                if c.blocked == false && self.wallup == false {
                    succ.push(c.clone());
                }
            }
            None => {}
        }
        match right {
            Some(c) => {
                if c.blocked == false && self.wallright == false {
                    succ.push(c.clone());
                }
            }
            None => {}
        }
        match down {
            Some(c) => {
                if c.blocked == false && self.walldown == false {
                    succ.push(c.clone());
                }
            }
            None => {}
        }

        

        succ.into_iter().map(|p| (p, 1)).collect()
    }
}
impl Coordinate {
    fn from(coordinate: Coordinate) -> Coordinate {
        Coordinate { x: coordinate.x, y: coordinate.y,  idspace: coordinate.idspace, idpoi : coordinate.idpoi, blocked: coordinate.blocked, wallup:coordinate.wallup, wallright: coordinate.wallright, walldown: coordinate.walldown, wallleft: coordinate.wallleft  }
    }

    
}