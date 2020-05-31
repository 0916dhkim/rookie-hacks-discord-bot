use crate::commands::adapter::list_applicants_for_group;
use crate::commands::adapter::group_of_member;
use crate::commands::adapter::accept_member;
use crate::commands::adapter::User;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	CommandResult,
	macros::command,
};

#[command]
pub fn accept(ctx: &mut Context, msg: &Message) -> CommandResult {
	// let user_name = format!("{}#{}", &msg.author.name, &msg.author.discriminator);
	let user = User::new(&msg.author.name, "", msg.author.discriminator);
	let users_group = group_of_member(&user);
	match users_group {
		None => {
			let _ = msg.reply(&ctx, "You are in no group, therefore you can't accept Applicants!");
		},
		Some(group) => {
			let applicants = list_applicants_for_group(&group.name);
			if applicants.len() == 0 {
				let _ = msg.reply(&ctx, "You currently have no Applicants for your group :/");
			} else {
				let mut to_accept = String::from("");
				if applicants.len() == 1 {
					to_accept = String::from(&applicants[0]);
					let _ = msg.reply(&ctx, format!("You accepted User {}!", applicants[0]));
				} else {
					let parsed = parse_message(msg);
					match parsed {
						None => {
							let _ = msg.reply(&ctx, format!("You currently have multiple Applicants, so you have to provide a name, of who to accept!\n
									These are your applicants: {:?}", applicants));
						},
						Some(member) => {
							for applicant in applicants {
								if applicant == member {
									let _ = msg.reply(&ctx, format!("You accepted User {}!", member));
									to_accept = String::from(member);
									break;
								}
							}
						}
					}
				}
				if to_accept != "" {
					accept_member(&group.name, &to_accept);
				}
			}
		}
	}
	let _ = msg.delete(&ctx);
	Ok(())
}

// TODO: provide means to detect and prevent Code injection when storing data persistently!
fn parse_message(msg: &Message) -> Option<String> {
	let message_text = &msg.content;
	let split = message_text.split(" ");
	let mut i = -1;
	let mut name: String = String::from("");
	for s in split {
		i += 1;
		if i == 0 {
			continue;
		} else if i > 1 {
			break;
		} else if i == 1 {
			name = String::from(s);
		}
	}
	if i < 1 {
		return None;
	}
	Some(name)
}