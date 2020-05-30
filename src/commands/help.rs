use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
pub fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    let help_message = "I can help you explore and join groups.
    Following is the list of available commands.
    `!help`: Show this message.
    `!me`: Show my team building status.
    `!ls groups`: List all groups.
    `!ls free`: List all available users.
    `!create <group_name>`: Create a group and join it.
    `!join <group_name>`: Join an existing group.
    `!merge <first_group_name> <second_group_name>`: Merge two groups.
    `!describe <group_name>`: Show group summary.";
    let _ = msg.reply(&ctx, help_message);
    Ok(())
}
