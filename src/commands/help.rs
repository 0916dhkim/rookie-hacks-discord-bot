use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	CommandResult,
	macros::command,
};

#[command]
pub fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
	// TODO: I cut !me and !merge. Do we need it?
	// -> `!me`: Show my team building status.
	// -> `!merge <first_group_name> <second_group_name>`: Merge two groups.
	let help_message = "I can help you explore and join groups.
	Following is the list of available commands.
	`!help`: Show this message.
	`!whoami`: See some info about you.
	`!checkin <description>`: Register as \"Team seeking\". Provide a description of yourself so others can read up on you!
	`!ls`: List all groups.
	`!ls_free`: List all available registered users.
	`!create <group_name> <description>`: Create a group and join it. Your `group_name` may not contain spaces.
	`!join <group_name>`: Join an existing group.
	`!describe <group_name>`: Show group summary.";
	let _ = msg.reply(&ctx, help_message);
	let _ = msg.delete(&ctx);
	Ok(())
}
