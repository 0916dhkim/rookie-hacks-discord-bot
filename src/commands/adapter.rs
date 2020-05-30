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
	pub fn new(discord_name: &str, description: &str) -> User {
		User {
			discord_name: String::from(discord_name),
			description: String::from(description)
		}
	}

	// copy constructor
	pub fn from(original: &User) -> User {
		User::new(&original.discord_name[..], &original.description[..])
	}

	// Equals Method
	pub fn equals(&self, other: &User) -> bool {
		other.discord_name == self.discord_name && other.description == self.description
	}

	// to_string method
	pub fn to_string(&self) -> String {
		format!("{}; description: {}", self.discord_name, self.description)
	}
}

// Group struct.
pub struct Group {
	pub name: String,   		// name of the group
	pub description: String,	// group description
	pub members: Vec<User>		// group member discord names
}
impl Group {
	// constructor
	pub fn new(name: &str, description: &str) -> Group {
		Group {
			name: String::from(name),
			description: String::from(description),
			members: Vec::new() // 0 members by default.
		}
	}
	// copy constructor
	pub fn from(original: &Group) -> Group {
		let mut ret = Group::new(
			&original.name[..],
			&original.description[..]
		);
		for member in original.members.as_slice() {
			ret.add_member(member);
		}
		ret
	}

	// Add a User to an existing Group
	pub fn add_member(&mut self, new_member: &User) {
		self.members.push(User::from(new_member));
	}

  fn contains_member(&mut self, member: &User) -> Option<usize> {
		for (i, e) in self.members.iter().enumerate() {
			if member.equals(&e) {
				return Some(i);
			}
		}
		None
	}

	// Remove a User from an existing Group
	fn remove_member(&mut self, member: &User) -> Option<User> {
		let v = self.contains_member(member);
		match v {
			None => {None},
			Some(x) => {Some(self.members.swap_remove(x))}
		}
	}

	// Remove a User from an existing Group
	fn remove_member(&mut self, member: &User) -> Option<User> {
		let v = self.contains_member(member);
		match v {
			None => {None},
			Some(x) => {Some(self.members.swap_remove(x))}
		}
	}

	// to_string method
	pub fn to_string(&self) -> String {
		let mut ret: String = format!("{}; Description: {}", self.name, self.description);
		for member in &self.members {
			ret = format!("{}\n{}", ret, member.to_string());
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
pub fn create_group(group_name: &str, member: &User) {
	let groups: &mut HashMap<String, Group> = &mut GROUPS.lock().unwrap();
	let mut new_group: Group = Group::new(group_name, "");
	new_group.members.push(User::from(member));
	groups.insert(
		String::from(group_name),
		new_group
	);
}
