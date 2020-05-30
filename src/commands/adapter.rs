// This file is for managing & manipulating data.

use std::collections::HashMap;
use std::sync::Mutex;

// Group struct.
pub struct Group {
    // TODO: Define group structure.
    pub name: String
}
impl Group {
    fn from(original: &Group) -> Group {
        return Group {
            name: String::from(&original.name)
        };
    }
}

// User struct.
pub struct User {
    // TODO: Define user structure.
    pub discord_name: String
}
impl User {
    fn from(original: &User) -> User {
        User {
            discord_name: String::from(&original.discord_name)
        }
    }
}

// Global variable for storing groups.
// use like this: groups.lock().unwrap()
lazy_static! {
    static ref GROUPS: Mutex<HashMap<String, Group>> = {
        Mutex::new({
            let mut m = HashMap::new();
            m.insert(String::from("b"), Group{name: String::from("b")});
            m.insert(String::from("c"), Group{name: String::from("c")});
            m.insert(String::from("d"), Group{name: String::from("d")});
            m.insert(String::from("e"), Group{name: String::from("e")});
            m
        })
    };
}

// Global variable for storing users.
// use like this: users.lock().unwrap()
lazy_static! {
    static ref USERS: Mutex<HashMap<String, User>> = {
        Mutex::new(
            HashMap::new()
        )
    };
}


// List all groups in the system.
pub fn list_groups() -> Vec<Group> {
    let mut v = Vec::new();
    for g in GROUPS.lock().unwrap().iter() {
        v.push(Group::from(&g.1));
    }
    v
}

// List all free (without group) users.
pub fn list_free_users() -> Vec<User> {
    let mut v = Vec::new();
    for u in USERS.lock().unwrap().iter() {
        // TODO: Filter free users.
        v.push(User::from(&u.1));
    }
    v
}
