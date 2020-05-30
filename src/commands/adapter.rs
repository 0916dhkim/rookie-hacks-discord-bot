// This file is for managing & manipulating data.

use std::collections::HashMap;
use std::sync::Mutex;

// User struct.
pub struct User {
	pub discord_name: String,   // discord name of the user
	pub description: String // user description
}
impl User {
	// constructor
	fn new(discord_name: &str, description: &str) -> User {
		User {
			discord_name: String::from(discord_name),
			description: String::from(description)
		}
	}
	// copy constructor
	fn from(original: &User) -> User {
		User::new(&original.discord_name[..], &original.description[..])
	}
}

// Group struct.
pub struct Group {
	pub name: String,   // name of the group
	pub description: String,	// group description
	pub members: Vec<String>	// group member discord names
}
impl Group {
	// constructor
	fn new(name: &str, description: &str) -> Group {
		Group {
			name: String::from(name),
			description: String::from(description),
			members: Vec::new() // 0 members by default.
		}
	}
	// copy constructor
	fn from(original: &Group) -> Group {
		let mut ret = Group::new(
			&original.name[..],
			&original.description[..]
		);
		for member in original.members.as_slice() {
			ret.members.push(String::from(member));
		}
		ret
	}
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

// Global variable for storing groups.
// use like this: groups.lock().unwrap()
lazy_static! {
	static ref GROUPS: Mutex<HashMap<String, Group>> = {
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

// Create a group with the given initial member.
pub fn create_group(group_name: &str, member_discord_name: &str) {
	let groups: &mut HashMap<String, Group> = &mut GROUPS.lock().unwrap();
	let mut new_group: Group = Group::new(group_name, "");
	new_group.members.push(String::from(member_discord_name));
	groups.insert(
		String::from(group_name),
		new_group
	);
}
