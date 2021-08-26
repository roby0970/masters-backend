use crate::{coordinates::Coordinates};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone, Copy)]
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
    pub fn handle(req: RouteRequest) ->Vec<Coordinates>{
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
      
        result.0
       
    }
}
