use crate::commands::adapter::User;
use crate::commands::adapter::user_checkin;
use crate::commands::adapter::user_description;

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
	let mut flag: bool = false;
	match parse_message(msg) {
		None => {
			flag = user_checkin(&User::new(&name, "", hash));
		},
		Some(description) => {
			flag = user_checkin(&User::new(&name, &description, hash));
		}
	}

	if !flag {
		let pri_message = format!("Hello, {}! You are now checked in!", name);
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

// TODO: provide means to detect and prevent Code injection when storing data persistently!
fn parse_message(msg: &Message) -> Option<String> {
	let message_text = &msg.content;
	let split = message_text.split(" ");
	let mut i = -1;
	let mut user_description: String = String::from("");
	for s in split {
		i += 1;
		if i == 0 {
			continue;
		} else if i == 1 {
			user_description.push_str(&String::from(s));
		} else if i > 1 {
			user_description.push_str(&format!(" {}", String::from(s)));
		}
	}
	if i < 1 {
		return None;
	}
	Some(user_description)
}