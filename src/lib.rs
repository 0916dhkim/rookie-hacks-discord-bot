#[macro_use]
extern crate lazy_static;

mod commands;

// Pool of test users for reference
/*
let alpha: User = User::new("Alpha", "First User", 1);
let beta: User = User::new("Beta", "Second User", 2);
let gamma: User = User::new("Gamma", "Third User", 3);
let delta: User = User::new("Delta", "Fourth User",  4);
let epsilon: User = User::new("Epsilon", "Fifth User", 5);
let zeta: User = User::new("Zeta", "Sixth User", 6);
let eta: User = User::new("Eta", "Seventh User", 7);
let theta: User = User::new("Theta", "Eighth User", 8);
*/

#[cfg(test)]
mod tests {
	use crate::commands::adapter::*;
	
	#[test]
	fn basic_create_users_and_groups() {
		// this tests the Constructs in adapter.rs 
		// no methods were used
		let alpha: User = User::new("Alpha", "First User", 1);
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

	#[test]
	fn listing_test() {
		let alpha: User = User::new("Alpha", "First User", 1);

		user_checkin(&alpha);
		assert_eq!(1, list_free_users().len());

		let beta: User = User::new("Beta", "Second User", 2);
		let gamma: User = User::new("Gamma", "Third User", 3);

		user_checkin(&beta);
		user_checkin(&gamma);
		assert_eq!(3, list_free_users().len());

		create_group("Group 1", "Group number 1", &alpha);
		assert_eq!(2, list_free_users().len());
		assert_eq!(1, list_groups().len());

		create_group("Group 2", "Group number 2", &beta);
		assert_eq!(1, list_free_users().len());
		assert_eq!(2, list_groups().len());
	}

	#[test]
	fn applicants_test() {
		// 4 users
		let alpha: User = User::new("Alpha", "First User", 1);
		let beta: User = User::new("Beta", "Second User", 2);
		let gamma: User = User::new("Gamma", "Third User", 3);
		let delta: User = User::new("Delta", "Fourth User",  4);
		
		// 2 groups
		let mut grp1: Group = Group::new("Group 1", "Group number 1");
		grp1.add_member(&alpha);
		let mut grp2: Group = Group::new("Group 2", "Group number 2");
		grp2.add_member(&beta);

		apply_for_group(&gamma, &grp1);
		apply_for_group(&gamma, &grp2);
		assert_eq!(2, show_current_application(&gamma).len());

		unapply_for_group(&gamma, &grp1);
		assert_eq!(1, show_current_application(&gamma).len());

		clear_all_applications_from_user(&gamma);
		assert_eq!(0, show_current_application(&gamma).len());
	}
}
