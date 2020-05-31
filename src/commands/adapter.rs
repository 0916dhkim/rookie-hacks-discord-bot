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
		if self.description != "" {
			return format!("{}#{}; description: {}", self.discord_name, self.hash, self.description)
		}
		self.to_string()
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
			if member.equals(e) {
				return Some(i);
			}
		}
		None
 	}

	// Returns, if a User is listed in the members field of the group
	pub fn contains_member(&mut self, member: &User) -> bool {
		match self.pos_of_member(member) {
			None => {false},
			Some(_) => {true}
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

	// TODO: Find a way to save this as a constant and to input it via command line
	pub fn max_members(&self) -> i32 {
		4
	}

	// TODO: Shitty last minute code
	pub fn is_full(&self) -> bool {
		self.num_members() == 4
	}
}

// Global variable for storing users.
// use like this: USERS.lock().unwrap()
lazy_static! {
	static ref USERS: Mutex<HashMap<String, User>> = {
		Mutex::new(
			HashMap::new()
		)
	};
}

// Global variable for storing groups.
// use like this: GROUPS.lock().unwrap()
lazy_static! {
	static ref GROUPS: Mutex<HashMap<String, Group>> = {
		Mutex::new(
			HashMap::new()
		)
	};
}

// Global variables for group application state.
// Key = discord_name + "#" + discord_hash
// Value = Vector of group names
// use like this APPLICANTS.lock().unwrap()
lazy_static! {
	static ref APPLICANTS: Mutex<HashMap<String, Vec<String>>> = {
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

// Get the group where the member belongs to
pub fn group_of_member(user: &User) -> Option<Group> {
	for (_, (_string, g)) in GROUPS.lock().unwrap().iter().enumerate() {
		let mut group = Group::from(&g);
		if group.contains_member(user) {
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

// List all groups in the system.
pub fn group_description_of_group_name(group_name: &str) -> Option<String> {
	for (i, (_string, g)) in GROUPS.lock().unwrap().iter().enumerate() {
		if g.name == group_name {
			return Some(String::from(&g.description));
		}
	}
	None
}

// List all groups in the system.
pub fn group_of_group_name(group_name: &str) -> Option<Group> {
	for (i, (_string, g)) in GROUPS.lock().unwrap().iter().enumerate() {
		if g.name == group_name {
			return Some(Group::from(&g));
		}
	}
	None
}

// Returns true, if the group is listed in the system
pub fn contains_group(group: &Group) -> bool {
	match pos_of_group(group) {
		None => {false},
		Some(_) => {true}
	}
}

// Returns true, if the group is listed in the system
pub fn contains_group_name(group_name: &str) -> bool {
	match pos_of_group_name(group_name) {
		None => {false},
		Some(_) => {true}
	}
}

// Returns the Group, if it could be removed (and does so as well)
pub fn remove_group(group: &Group) {
	GROUPS.lock().unwrap().remove(&group.name);
}

// Check-in the user. Solely for list_free_users method
// true if user is already checked in
pub fn user_checkin(member: &User) -> bool {
	let users: &mut HashMap<String, User> = &mut USERS.lock().unwrap();
	let flag: bool = users.contains_key(&member.to_string());
	if !flag {
		users.insert(
			String::from(&member.to_string()),
			User::from(member),
		);
	}
	flag
}

pub fn get_user(user_str: &str) -> Option<User> {
	for (i, (_string, u)) in USERS.lock().unwrap().iter().enumerate() {
		if u.to_string() == user_str {
			return Some(User::from(&u));
		}
	}
	None
}

pub fn user_description(user_str: &str) -> String {
	match get_user(user_str) {
		None => {
			return String::from("");
		},
		Some(user) => {
			return String::from(&user.description);
		}
	}
}

// List all free (without group) users.
pub fn list_free_users() -> Vec<User> {
	let mut v = Vec::new();
	for u in USERS.lock().unwrap().iter() {
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

// Create a group with the given initial member.
pub fn create_group_existing(group: &Group) {
	let groups: &mut HashMap<String, Group> = &mut GROUPS.lock().unwrap();
	groups.insert(
		String::from(String::from(&group.name)),
		Group::from(group)
	);
}

// Apply for a group with name.
pub fn apply_for_group_name(member: &User, group_name: &str) -> bool {
	let mut applicants = APPLICANTS.lock().unwrap();
	let key = &member.to_string();
	let value_element = group_name;
	let mut flag: bool = true;

	match applicants.get_mut(key) {
		None => { // APPLICANT does not have user's string
			applicants.insert(
				String::from(key),
				vec![String::from(value_element)]
			);
		},
		Some(x) => { // APPLICANT has user's string
			let placeholder: Vec<String> = x.to_vec();
			for elements in placeholder {
				if elements == value_element.to_string() {
					flag = false;
				}
			}
			if flag {
				x.push(String::from(value_element));
			}
		}
	}
	flag
}

// Apply for a group.
pub fn apply_for_group(member: &User, group: &Group) -> bool {
	apply_for_group_name(member, &group.name)
}

// Unapply for a group
pub fn unapply_for_group(member: &User, group: &Group) {
	let mut applicants = APPLICANTS.lock().unwrap();
	let key = &member.to_string();
	let value_element = &group.name;

	match applicants.get_mut(key) {
		None => {	}, // APPLICANT does not have user's string
		Some(x) => { // APPLICANT has user's string
			x.retain(|x| *x == String::from(value_element));
		}
	}
}

// Show your current application
// show vector length 0 if user has not applied for a group (I'm sorry this is bad)
pub fn show_current_application(member: &User) -> Vec<String> {
	let mut applicants = APPLICANTS.lock().unwrap();
	let key = &member.to_string();

	match applicants.get(key) {
		None => {Vec::new()}, // APPLICANT does not have user's string
		Some(x) => { // APPLICANT has user's string
			x.to_vec()
		}
	}
}

// Straight up remove the entry
pub fn clear_all_applications_from_user(member: &User) {
	APPLICANTS.lock().unwrap().remove(&member.to_string());
}

// Accept a member
pub fn accept_member(group: &Group, user_str: &str) -> String {
	let mut group = Group::from(&group);
	/*if !contains_group_name(group_name) {
		return String::from("This group does not exist.");
	}*/
	let split = user_str.split("#");
	let mut i: i32 = -1;
	let mut name = String::from("");
	let mut hash: u16 = 0;
	for s in split {
		i += 1;
		if i == 0 {
			name = String::from(s);
		} else if i > 1 {
			break;
		} else if i == 1 {
			hash = s.parse::<u16>().unwrap();
		}
	}
	let user = User::new(&name, &user_description(&name), hash);
	/*match group_of_member(&user) {
		None => {
			return String::from("You are in no group. Therefore you can't accept members.");
		},
		Some(mut group) => {*/
	if group.is_full() {
		return String::from("Your group is already full. You can't accept further members.");
	}
	group.add_member(&user);
	remove_group(&group);
	create_group_existing(&group);
	clear_all_applications_from_user(&user);
	String::from("All fine!")
}

// Remove a member
pub fn remove_member(group_name: &str, user: &User) -> String {
	if !contains_group_name(group_name) {
		return String::from("This group does not exist.");
	}
	match group_of_member(&user) {
		None => {
			return String::from("You are in no group. Therefore you can't be removed.");
		},
		Some(mut group) => {
			&group.remove_member(&user);
			remove_group(&group);
			create_group_existing(&group);
		}
	}
	String::from("All fine!")
}

// Returns a list of all User names, that apply for the group
pub fn list_applicants_for_group(group_name: &str) -> Vec<String> {
	let mut ret: Vec<String> = Vec::new();
	for (applicant, groups) in APPLICANTS.lock().unwrap().iter() {
		for group in groups {
			if group == group_name {
				ret.push(String::from(applicant));
				break;			// TODO: Might be problematic
			}
		}
	}
	ret
}
