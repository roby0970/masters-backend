use crate::{MyWs, coordinates::Coordinates, python_knn::RawCoordinate};
use futures::stream::ForEach;
use serde::{Deserialize, Serialize};
use actix_web_actors::ws;

use micromath::F32Ext;
#[derive(Deserialize, Serialize, Clone)]
pub struct CoordinateRequest {
    pub source: Vec<BLEReading>
}
#[derive(Deserialize, Serialize, Clone)]
pub struct CoordinateResponse {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Serialize,Debug, Clone, Copy)]
pub struct BLEReading {
    id_ble: i32,
    rssi: i32,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct RouteRequest {
    pub id: String,
    pub compass: f32,
    pub space: i32,
    pub destination_poi: i32,
    pub source: Vec<BLEReading>,
    pub finished: bool,
}

impl RouteRequest {
    pub fn handleCoordinates(req: CoordinateRequest) -> RawCoordinate{
        let source = req.source;
        
        let predicted_coordinate = crate::python_knn::classify(source).unwrap();
        predicted_coordinate
        
    }
    pub fn handle(req: RouteRequest) ->Vec<CoordinateResponse>{
        let source = req.source;


        //Predict x and y values based on BLE Readings
        let predicted_coordinate = crate::python_knn::classify(source).unwrap();

        
        //Find the source coordinate according to prediction
        let source_coordinate = Coordinates::find_by_space_id_and_xy(
            req.space,
            predicted_coordinate.x,
            predicted_coordinate.y,
        )
        .unwrap();

       
        
        //Find all coordinates for this space
        let space_coordinates = Coordinates::find_by_space_id(req.space).unwrap();
        //Find all coordinates for destination POI
        let poi_coordinates = Coordinates::find_by_poi_id(req.destination_poi).unwrap();

        //If POI has multiple coordinates, take first -- Should improve
        let dest = poi_coordinates.first().unwrap();

        //Using the predicted source coordinate, space coordinates and selected destination, find the shortest path
        let result = crate::astar::pathfind(source_coordinate, space_coordinates, dest).unwrap();
        
        //Converting the main coordinates to new AR Space coordinates, while rotating the axes based on orientation
        let mut converted : Vec<CoordinateResponse> = [].to_vec();
       
        for c in result.0{
            converted.push(convert_to_new_cartesian(c, req.compass, predicted_coordinate.x as f32, predicted_coordinate.y as f32));
        }
        converted
    }

   
}
pub fn convert_to_new_cartesian(coordinate: Coordinates, compass: f32, source_x: f32, source_y: f32) -> CoordinateResponse{
    let old_x = coordinate.x as f32;
    let old_y = coordinate.y as f32;
    let rad  = std::f32::consts::PI / 180.0 * compass;
    
    let new_x = (((old_x - source_x)* rad.cos() - (old_y - source_y) * rad.sin() + source_x) * 1000.0).round() / 1000.0;
    let new_y = (((old_x - source_x)* rad.sin() + (old_y - source_y) * rad.cos() + source_y) * 1000.0).round() / 1000.0;
    println!("{}, {}", old_x, old_y);
    println!("{}, {}", new_x, new_y);
    println!("{}, {}", new_x - source_x, new_y - source_y);
   CoordinateResponse{
       x: new_x - source_x,
       y: new_y - source_y 
   } 
}