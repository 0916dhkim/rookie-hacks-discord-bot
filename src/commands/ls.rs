use crate::commands::adapter::list_groups;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args,
	CommandResult,
	macros::command,
};

#[command]
pub fn ls(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	let groups = list_groups();
	if groups.len() == 0 {
		let _ = msg.reply(&ctx, "There are currently no groups. Go on and create one!");
	} else {
		let mut ret: String = String::from("Groups:\n");
		for group in groups.iter() {
			ret.push_str(&group.to_string_with_members());
			ret.push('\n');
		}
		let _ = msg.reply(&ctx, ret);
	}
	Ok(())
}
