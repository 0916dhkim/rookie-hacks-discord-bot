use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	CommandResult,
	macros::command,
};

#[command]
pub fn whoami(ctx: &mut Context, msg: &Message) -> CommandResult {
	//let _ = msg.reply(&ctx, format!("{:?}", "Test"));
	let test_author = &msg.author;
	let _ = msg.reply(&ctx, format!("Hi, you are {:?}!", test_author.name));
	let _ = msg.delete(&ctx);
	Ok(())
}