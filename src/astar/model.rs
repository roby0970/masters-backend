


use pathfinding::prelude::{astar};

use crate::coordinates::Coordinates;

pub fn pathfind(source: Coordinates, succ: Vec<Coordinates>, dest: &Coordinates ) -> Option<(Vec<Coordinates>, u32)>{
    astar(
        &source,
        |p| p.successors(succ.clone()),
        |p| p.distance(&dest),
        |p| *p == *dest,
    )
}