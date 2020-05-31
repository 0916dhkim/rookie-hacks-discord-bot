use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	CommandResult,
	macros::command,
};

#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
	let _ = msg.reply(&ctx, "pong!");
	let _ = msg.delete(&ctx);
	Ok(())
}
