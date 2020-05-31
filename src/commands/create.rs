use crate::commands::adapter::create_group;
use crate::commands::adapter::contains_group_name;
use crate::commands::adapter::User;
use crate::commands::adapter::group_of_member;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	CommandResult,
	macros::command,
};

#[command]
pub fn create(ctx: &mut Context, msg: &Message) -> CommandResult {
	let user_out = &msg.author;
	let user_test = User::new(&user_out.name, "", user_out.discriminator);
	let users_group = group_of_member(&user_test);
	match users_group {
		None => {
			let parsed_message = parse_message(msg);
			match parsed_message {
				None => {
					let _ = msg.reply(&ctx, "You need to provide a group name!");
				},
				Some((group_name, group_description)) => {
					if contains_group_name(&group_name) {
						let _ = msg.reply(&ctx, "Sadly, a group with this name already exists...\nPlease choose another name!");
					} else {
						let user = User::new(&user_out.name, "", user_out.discriminator);
						create_group(&group_name, &group_description, &user);
						if group_description != "" {
							// TODO: Place fix with "group_description" here
							let _ = msg.reply(&ctx, format!("User '{}' created group '{}' with the description '{}'", user_out.name, group_name, group_description));
						} else {
							let _ = msg.reply(&ctx, format!("User '{}' created group '{}'\n -> You should provide a group description, so that other users know, what you are about!",
								user_out.name, group_name));
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
fn parse_message(msg: &Message) -> Option<(String, String)> {
	let message_text = &msg.content;
	let split = message_text.split(" ");
	let mut i = -1;
	let mut group_name: String = String::from("THIS IS A BUG!");
	let mut group_description: String = String::from("");
	for s in split {
		i += 1;
		if i == 0 {
			continue;
		} else if i == 1 {
			group_name = String::from(s);
		} else if i == 2 {
			group_description.push_str(&String::from(s));
		} else if i > 2 {
			group_description.push_str(&format!(" {}", String::from(s)));
		}
	}
	if i < 1 {
		return None;
	}
	Some((group_name, group_description))
}