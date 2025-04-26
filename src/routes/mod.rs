pub mod planner;
pub mod programmer;

use planner::route_planner;
use programmer::route_programmer;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![route_planner, route_programmer]
}
