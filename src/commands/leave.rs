use crate::commands::adapter::group_of_member;
use crate::commands::adapter::remove_group;
use crate::commands::adapter::User;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	CommandResult,
	macros::command,
};

#[command]
pub fn leave(ctx: &mut Context, msg: &Message) -> CommandResult {
	let user_out = &msg.author;
	let user = User::new(&user_out.name, "", user_out.discriminator);
	let group_opt = group_of_member(&user);
	match group_opt {
		None => {
			let _ = msg.reply(&ctx, "You can't leave a group, since you are in no group yet!");
		},
		Some(mut group) => {
			let _ = msg.reply(&ctx, format!("Leaving group {}", group.to_string_with_members()));
			if group.num_members() <= 1 {
				remove_group(&group);
			} else {
				// TODO: This may be problematic. I think, I am just removing the user from tho local variable
				group.remove_member(&user);
			}
		}
	}
	let _ = msg.delete(&ctx);
	Ok(())
}