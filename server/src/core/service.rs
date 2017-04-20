use super::model::{Role, Permission, Access};
use std::sync::Arc;

pub fn find_roles() -> Vec<Role> {
    let roles = vec![
        Role {
            id: 1,
            name: String::from("ROLE"),
            description: String::from("Admin Role"),
            permissions: vec![
                Permission { id: 1, resource: String::from("ROLE"), access: Access::READ }, ]
        }
    ];

    return roles;
}


pub fn find_role(id: u32) -> Role {
    let role = Role {
        id: id,
        name: String::from("ROLE"),
        description: String::from("Admin Role"),
        permissions: vec![
            Permission { id: 1, resource: String::from("ROLE"), access: Access::READ }, ]
    };

    return role;
}

pub struct Service {
    pub permissions: Arc<Vec<Permission>>,
    pub roles: Vec<Role>,
}

impl Service {
    pub fn new() -> Service {
        let permissions = Arc::new(vec![
            Permission { id: 1, resource: String::from("ROLE"), access: Access::READ },
            Permission { id: 2, resource: String::from("ROLE"), access: Access::CREATE },
            Permission { id: 3, resource: String::from("ROLE"), access: Access::UPDATE },
            Permission { id: 4, resource: String::from("ROLE"), access: Access::DELETE },
            Permission { id: 5, resource: String::from("USER"), access: Access::READ },
            Permission { id: 6, resource: String::from("USER"), access: Access::CREATE },
            Permission { id: 7, resource: String::from("USER"), access: Access::UPDATE },
            Permission { id: 8, resource: String::from("USER"), access: Access::DELETE },
        ]);
        let roles = vec![
            Role {
                id: 1,
                name: String::from("READ_ONLY"),
                description: String::from("Read only access"),
                permissions: vec![Permission { id: 3, resource: String::from("ROLE"), access: Access::UPDATE },]
            }
        ];
        return Service { permissions: permissions, roles: roles };
    }
}