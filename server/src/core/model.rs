use std::rc::Rc;

#[derive(Serialize, Deserialize)]
pub struct Permission {
    pub id: u32,
    pub resource: String,
    pub access: Access,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Access {
    READ,
    CREATE,
    UPDATE,
    DELETE,
    ADMIN,
}

#[derive(Serialize, Deserialize)]
pub struct Role<> {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub permissions: Vec<Permission>,
}
