use super::model::{Permission, Role};
use super::service;
use std::rc::Rc;
use rocket::State;
use rocket_contrib::JSON;

#[get("/core/permission")]
pub fn get_permissions(service: State<service::Service>) -> JSON<Vec<Permission>> {
    return JSON(service.permissions);
}

#[get("/core/role")]
pub fn get_roles() -> JSON<Vec<Role>> {
    return JSON(service::find_roles());
}


#[get("/core/role/<id>")]
pub fn get_role(id: u32) -> JSON<Role> {
    return JSON(service::find_role(id));
}
