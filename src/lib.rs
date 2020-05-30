#[macro_use]
extern crate lazy_static;

mod commands;

// Pool of test users for reference
/*
let alpha: User = User::new("Alpha", "First User");
let beta: User = User::new("Beta", "Second User");
let gamma: User = User::new("Gamma", "Third User");
let delta: User = User::new("Delta", "Fourth User");
let epsilon: User = User::new("Epsilon", "Fifth User");
let zeta: User = User::new("Zeta", "Sixth User");
let eta: User = User::new("Eta", "Seventh User");
let theta: User = User::new("Theta", "Eighth User");
*/

#[cfg(test)]
mod tests {
	use crate::commands::adapter::*;
	
	#[test]
	fn basic_create_users_and_groups() {
		// this tests the Constructs in adapter.rs 
		// no methods were used
		let alpha: User = User::new("Alpha", "First User");
		assert_eq!("Alpha", alpha.discord_name);
		assert_eq!("First User", alpha.description);

		let mut grp1: Group = Group::new("Group 1", "Group number 1");
		// used the constructor and not the given method create_group
		assert_eq!("Group 1", grp1.name);
		assert_eq!("Group number 1", grp1.description);
		assert_eq!(0, grp1.members.len());

		grp1.add_member(&alpha);
		assert_eq!(1, grp1.members.len());

		grp1.remove_member(&alpha);
		assert_eq!(0, grp1.members.len());
	}
}
