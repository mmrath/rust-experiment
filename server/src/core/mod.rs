pub mod handler;
pub mod model;
pub mod service;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    return routes![
        handler::get_permissions,
        handler::get_roles,
        handler::get_role,
    ];
}