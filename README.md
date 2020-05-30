# rookie-hacks-discord-bot
Coolest discord bot made within 48 hours.

## Contribute
This application is built with Rust and Cargo.
### Build
```
cargo build
```
### Run
```
cargo run
```
### Adding A New Command
If you want to define a new command named "mycommand", you need to do the following:
- Create a new file `commands/mycommand.rs`
	- Define your function in `commands/mycommand.rs` like the following:
	```rust
	use serenity::prelude::*;
	use serenity::model::prelude::*;
	use serenity::framework::standard::{
		CommandResult,
		macros::command,
	};

	#[command]
	pub fn mycommand(ctx: &mut Context, msg: &Message) -> CommandResult {
		let _ = msg.reply(&ctx, "pong!");
		Ok(())
	}
	```
- In `commands/mod.rs`
	- Import the newly created file. by appending:
	```
	pub mod mycommand; // Add this line to mod.rs file.
	```
- In `main.rs`:
	- Import your command:
	```rust
	use commands::{
		command1::*, // Existing command 1
		command2::*,
		command3::*,
		command4::*, // Existing command 4
		mycommand::* // Add yours here.
	};
	```
	- Also, add your command to the general group.
	```rust
	#[group]
	#[commands(command1, command2, command3, mycommand)] // Modify this line.
	struct General;
	```
