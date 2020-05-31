use crate::commands::adapter::group_of_member;
use crate::commands::adapter::User;
use crate::commands::adapter::pos_of_group_name;
use crate::commands::adapter::apply_for_group_name;
use crate::commands::adapter::user_description;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	CommandResult,
	macros::command,
};

#[command]
pub fn apply(ctx: &mut Context, msg: &Message) -> CommandResult {
	let user = User::new(&msg.author.name, &user_description(&msg.author.name), msg.author.discriminator);
	let users_group = group_of_member(&user);
	match users_group {
		None => {
			let parsed = parse_message(msg);
			match parsed {
				None => {
					let _ = msg.reply(&ctx, "You have to provide the group name for which to apply!\nUse `!ls` to list groups.");
				},
				Some(group_name) => {
					let pos = pos_of_group_name(&group_name);
					match pos {
						None => {
							let _ = msg.reply(&ctx, "The specified group doesn't exist.\nUse `!ls` to list groups.");
						},
						Some(_) => {
							// TODO: insert Users description
							let _ = msg.reply(&ctx, format!("Applying for group {}", group_name));
							let user = User::new(&msg.author.name, &user_description(&msg.author.name), msg.author.discriminator);
							apply_for_group_name(&user, &group_name);
						}
					}
				}
			}
		},
		Some(group) => {
			let _ = msg.reply(&ctx, format!("You are already in a group: {}. Leave it to apply for another one.", group.name));
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