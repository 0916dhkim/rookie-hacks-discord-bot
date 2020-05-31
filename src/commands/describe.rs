use crate::commands::adapter::group_description_of_group_name;
use crate::commands::adapter::group_of_group_name;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	CommandResult,
	macros::command,
};

#[command]
pub fn describe(ctx: &mut Context, msg: &Message) -> CommandResult {
	let group_str = parse_group_name(msg);
	match group_str {
		None => {
			let _ = msg.reply(&ctx, "You have to provide the group name to see a description!\nUse `!ls` to list groups.");
		},
		Some(group_name) => {
			let g_opt = group_description_of_group_name(&group_name);
			match g_opt {
				None => {
					let _ = msg.reply(&ctx, "The specified group doesn't exist.\nUse `!ls` to list groups.");
				},
				Some(description) => {
					if description != "" {
						let _ = msg.reply(&ctx, format!("Description of group '{}': {}", group_name, description));
					} else {
						let _ = msg.reply(&ctx, "Sadly, this group has no description :/");
					}
					let mut ret = String::from(format!("Users of group '{}':\n", group_name));
					match group_of_group_name(&group_name) {
						None => {
							ret.push_str("No Users!");
						},
						Some(group) => {
							for user in group.members {
								ret.push_str(&user.to_string_with_description());
							}
						}
					}
					let _ = msg.reply(&ctx, ret);
				}
			}
		}
	}
	let _ = msg.delete(&ctx);
	Ok(())
}

// TODO: provide means to detect and prevent Code injection when storing data persistently!
fn parse_group_name(msg: &Message) -> Option<String> {
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