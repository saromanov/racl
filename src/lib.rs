use std;
use std::collections::HashMap;
// https://fasterthanli.me/articles/a-half-hour-to-learn-rust

//https://github.com/kildevaeld/go-acl/blob/master/acl.go
//https://rust-unofficial.github.io/patterns/intro.html

pub trait Store {
    fn new() -> Self;
    fn add_role(&mut self, name:&str, inherits:&str) -> bool;
    fn get_role(&mut self, name: &str) -> Result<Role, &'static str>;
    fn update_permissions(&mut self, name: &str, action: &str, resource: &str) -> Result<bool, &'static str>;
    fn exists(&mut self, name: &str) -> bool;
}

struct Mem {
    roles: HashMap<String, Role>,
}
impl Store for Mem {
    fn new() -> Self {
        Mem {
            roles: HashMap::new(),
        }
    }
    fn add_role(&mut self, name:&str, inherits:&str) -> bool{
        let role = Role::new(name, inherits);
        self.roles.entry(name.to_string()).or_insert(role);
        true
    }
    fn get_role(&mut self, name: &str) -> Result<Role, &'static str> {
        if self.roles.is_empty() {
            return Err("roles is not found")
        }
        match self.roles.get(name) {
            Some(role) => {
                return Ok(role.clone());
            },
            None => {
                return Err("unable to find role")
            },
        };
    }
    fn exists(&mut self, name: &str) -> bool {
        self.roles.contains_key(name)
    }
    fn update_permissions(&mut self, name:&str, action: &str, resource: &str) -> Result<bool, &'static str> {
        match self.roles.get(name) {
            Some(role) => {
                let perm = Permission{
                    action: action.to_string(),
                    resource: resource.to_string()
                };
                let mut perms = role.permissions.clone();
                perms.push(perm);
                let new_perm = Role{
                    name: name.to_string(),
                    parent: role.parent.clone(),
                    permissions: perms.to_vec(),
                };
                self.roles.remove(&name.to_string());
                self.roles.insert(name.to_string(), new_perm);
                Ok(true)
            },
            None =>Err("role is not found")
        };
        return Err("unable to find role");
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Role {
    name: String,
    parent: String,
    permissions: Vec<Permission>
}

impl Role {
    fn new(name: &str, parent: &str) -> Self {
        Role{ name: name.to_string(), parent: parent.to_string(), permissions:Vec::new() }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Permission {
    action: String,
    resource: String
}

pub struct Acl<T:Store> {
    store: T,
}

impl<T:Store> Acl<T> {
    pub fn new(store: T) -> Acl<T> {
        Acl{
            store: store,
        }
    }
    pub fn add_role(&mut self, name:&str, inherits:&str) -> Result<(), &'static str> {
        if name.is_empty() {
            return Err("string is empty");
        };
        self.store.add_role(name, inherits);
        return Ok(());
    }

    pub fn allow(&mut self, roles: Vec<&str>, action: &str, resource: &str) {
        assert!(roles.iter().all(|x| self.store.exists(x)));
        for role in &roles {
            self.store.update_permissions(role, action, resource);
        }
    }

    pub fn available(&mut self, role: &str, action: &str, resource: &str) {
        assert!(self.store.exists(role));
        let role_obj = self.store.get_role(role);
        match role_obj {
            Ok(role) => {
                let permisions = role.permissions;
                let result = permisions.iter().any(|x| x.action == action && x.resource == resource);
                println!("{}", result);
            },
            Err(err) => {
                println!("{}", err);
            }
        }

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
