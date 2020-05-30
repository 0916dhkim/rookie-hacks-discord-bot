use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	CommandResult,
	macros::command,
};

#[command]
pub fn whoami(ctx: &mut Context, msg: &Message) -> CommandResult {
	let user = &msg.author;
	let _ = msg.reply(&ctx, format!("Hi, you are {}#{}!", user.name, user.discriminator));
	let _ = msg.delete(&ctx);
	Ok(())
}