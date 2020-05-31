use crate::commands::adapter::User;
use crate::commands::adapter::user_checkin;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	CommandResult,
	macros::command,
};

#[command]
pub fn checkin(ctx: &mut Context, msg: &Message) -> CommandResult {
	let user = &msg.author;
	let name = &user.name;
	let hash = user.discriminator;
	let flag: bool = user_checkin(&User::new(&name, "", hash));

	if !flag {
		let pri_message = format!("Hello, {}! Yor are now checked in!", name);
		let dm = msg.author.dm(&ctx, |m| {

			m.content(pri_message);

			m
		});
	} else {
		let pri_message = format!("You are already checked in, {}!", name);
		let dm = msg.author.dm(&ctx, |m| {

			m.content(pri_message);

			m
		});
	}
	let _ = msg.delete(&ctx);
	Ok(())
}