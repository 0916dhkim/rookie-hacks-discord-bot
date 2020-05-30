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
	let mut s: String = String::from("list of groups\n");
	let g = list_groups();
	for i in g.iter() {
		if i.description == "" {
			s.push_str(format!("{}\n", i.name).as_str());
		} else {
			s.push_str(format!("{}: {}\n", i.name, i.description).as_str());
		}
	}
	let _ = msg.reply(&ctx, s);
	Ok(())
}
