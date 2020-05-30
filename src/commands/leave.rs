use crate::commands::adapter::group_of_member;
use crate::commands::adapter::User;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	CommandResult,
	macros::command,
};

#[command]
pub fn leave(ctx: &mut Context, msg: &Message) -> CommandResult {
	let user_out = &msg.author;
	let user = User::new(&user_out.name, "", user_out.discriminator);
	let group = group_of_member(&user);
	match group {
		None => {
			let _ = msg.reply(&ctx, "You can't leave a group, since you are in no group yet!");
		},
		Some(x) => {
			let _ = msg.reply(&ctx, format!("Leaving group {}", x.to_string_with_members()));
		}
	}
	Ok(())
}