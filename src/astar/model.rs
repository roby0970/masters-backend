


use pathfinding::prelude::{astar, astar_bag};

use crate::coordinates::Coordinates;

pub fn pathfind_bad(source: Coordinates, succ: Vec<Coordinates>, dest: &Coordinates ) -> Option<(Vec<Coordinates>, u32)>{
    
    astar(
        &source,
        |p| p.successors(succ.clone()),
        |p| p.distance(&dest),
        |p| *p == *dest,
    )
}
pub fn pathfind(source: Coordinates, space_coords: Vec<Coordinates>, dest: &Coordinates) -> Option<(Vec<Coordinates>, u32)> {
    let mut res = astar_bag(&source, |p| p.successors(space_coords.clone()), |p| p.distance(&dest), |p| p.idpoi == dest.idpoi).unwrap();
    return match res.0.next(){
        Some(x) => Some((x,res.1)),
        None => None,
    }
}