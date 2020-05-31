use crate::commands::adapter::list_free_users;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args,
	CommandResult,
	macros::command,
};

#[command]
pub fn ls_free(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	let free_users = list_free_users();
	if free_users.len() == 0 {
		let _ = msg.reply(&ctx, "No one is available for recruiting.");
	} else {
		let mut ret: String = String::from("List of available people:\n");
		for user in free_users.iter() {
			ret.push_str(&user.to_string());
			ret.push('\n');
		}
		let _ = msg.reply(&ctx, ret);
	}
	Ok(())
}
