#[macro_use]
extern crate lazy_static;

mod commands;
use std::{
	env,
};
use dotenv::dotenv;
use serenity::{
	framework::standard::{
		StandardFramework,
		macros::group
	},
	model::{event::ResumedEvent, gateway::Ready},
	prelude::*,
};
use commands::{
	help::*,
	ls::*,
	ping::*,
	whoami::*,
	create::*,
	leave::*,
	checkin::*,
	ls_free::*
};

struct Handler;
impl EventHandler for Handler {
	fn ready(&self, _:Context, ready: Ready) {
		println!("Connected as {}", ready.user.name);
	}

	fn resume(&self, _:Context, _:ResumedEvent) {
		println!("Resumed");
	}
}

// Create GENERAL_GROUP for commands.
#[group]
#[commands(
	help,
	ls,
	ping,
	whoami,
	create,
	leave,
	checkin,
	ls_free
)]
struct General;

fn main() {
	dotenv().ok(); // Read environment variables.
	
	// Note: You need a valid Client Token that's generated by Discord, similar to Google Api's Key.
	// Apparently, posting a token publicly is a bad idea, so I did not put it here.
	let token = env::var("DISCORD_TOKEN")
		.expect("Expected a token in the environment");

	// Declare discord API client.
	let mut client = Client::new(&token, Handler).expect("Err creating client");

	// Use serenity's command framework.
	client.with_framework(StandardFramework::new()
		.configure(|c| c.prefix("!"))
		.group(&GENERAL_GROUP));

	// Start discord API client.
	if let Err(why) = client.start() {
		println!("Client error: {:?}", why);
	}
}
