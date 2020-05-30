// This file is for managing & manipulating data.

use std::collections::HashMap;
use std::sync::Mutex;

// User struct.
pub struct User {
	pub discord_name: String,   // discord name of the user
	pub description: String,	// user description
	pub hash: u16
}
impl User {
	// constructor
	pub fn new(discord_name: &str, description: &str, hash: u16) -> User {
		User {
			discord_name: String::from(discord_name),
			description: String::from(description),
			hash: hash
		}
	}

	// copy constructor
	pub fn from(original: &User) -> User {
		User::new(&original.discord_name[..], &original.description[..], original.hash)
	}

	// Equals Method
	pub fn equals(&self, other: &User) -> bool {
		other.discord_name == self.discord_name && other.hash == self.hash
	}

	// to_string method, with description
	pub fn to_string_with_description(&self) -> String {
		format!("{}#{}; description: {}", self.discord_name, self.hash, self.description)
	}

	// to_string method
	pub fn to_string(&self) -> String {
		format!("{}#{}", self.discord_name, self.hash)
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

	// Get the position of a User in the members Vector
	pub fn pos_of_member(&mut self, member: &User) -> Option<usize> {
		for (i, e) in self.members.iter().enumerate() {
			if member.equals(&e) {
				Some(i);
			}
		}
		None
 	}

	// Returns, if a User is listed in the members field of the group
	pub fn contains_member(&mut self, member: &User) -> bool {
		match self.pos_of_member(member) {
			None => {false},
			Some(x) => {true}
		}
	}

	// Remove a User from an existing Group
	pub fn remove_member(&mut self, member: &User) -> Option<User> {
		let v = self.pos_of_member(member);
		match v {
			None => {None},
			Some(x) => {Some(self.members.swap_remove(x))}
		}
	}

	pub fn to_string(&self) -> String {
		let mut s: String = String::from("");
		if self.description == "" {
			s.push_str(format!("{}", self.name).as_str());
		} else {
			s.push_str(format!("{}: {}", self.name, self.description).as_str());
		}
		s.push_str(&format!(", {}/{}", self.num_members(), self.max_members()));
		s
	}

	// to_string method
	pub fn to_string_with_members(&self) -> String {
		let mut ret: String = self.to_string();
		let mut first: bool = true;
		for member in &self.members {
			if !first {
				ret = format!("{}, {}", ret, member.to_string());
			} else {
				ret = format!("{}\n{}", ret, member.to_string());
				first = false;
			}
		}
		ret
	}

	// Get the number of members
	pub fn num_members(&self) -> usize {
		self.members.len()
	}

	// TODO Find a way to save this as a constant and to input it via command line
	pub fn max_members(&self) -> i32 {
		4
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

pub fn group_of_member(user: &User) -> Option<Group> {
	for (i, (_string, g)) in GROUPS.lock().unwrap().iter().enumerate() {
		let mut group = (Group::from(&g));
		if (group.contains_member(user)) {
			return Some(group);
		}
	}
	None
}

// List all groups in the system.
pub fn pos_of_group(group: &Group) -> Option<usize> {
	pos_of_group_name(&group.name)
}

// List all groups in the system.
pub fn pos_of_group_name(group_name: &str) -> Option<usize> {
	for (i, (_string, g)) in GROUPS.lock().unwrap().iter().enumerate() {
		if g.name == group_name {
			return Some(i);
		}
	}
	None
}

// Returns true, if the group is listed in the system
pub fn contains_group(group: &Group) -> bool {
	match pos_of_group(group) {
		None => {false},
		Some(x) => {true}
	}
}

// Returns true, if the group is listed in the system
pub fn contains_group_name(group_name: &str) -> bool {
	match pos_of_group_name(group_name) {
		None => {false},
		Some(x) => {true}
	}
}

// List all free (without group) users.
pub fn list_free_users() -> Vec<User> {
	let mut v = Vec::new();
	for u in USERS.lock().unwrap().iter() {
		// TODO: Filter free users.
		// Still WIP
		let user = User::from(&u.1);
		let mut is_free: bool = true;

		for g in GROUPS.lock().unwrap().iter() {
			if Group::from(&g.1).contains_member(&user) {
				is_free = false;
				break;
			}
		}

		if is_free {
			v.push(user);
		}		
	}
	v
}

// Create a group with the given initial member.
pub fn create_group(group_name: &str, group_description: &str, member: &User) {
	let groups: &mut HashMap<String, Group> = &mut GROUPS.lock().unwrap();
	let mut new_group: Group = Group::new(group_name, group_description);
	new_group.members.push(User::from(member));
	groups.insert(
		String::from(group_name),
		new_group
	);
}
