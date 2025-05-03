use kovi::{MsgEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();
    P::on_group_msg(move |e| on_group_msg(e, bot.clone()));
}

fn on_group_msg(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>) {
    // Get the group ID from the event
    let group_id = match e.group_id {
        Some(id) => id,
        None => return, // Not a group message
    };

    let text = match e.borrow_text() {
        Some(v) => v,
        None => return,
    };

    // Only handle messages starting with /np
    if !text.starts_with("/np") {
        return;
    }

    // Split the command into parts
    let args: Vec<&str> = text.split_whitespace().collect();
    if args.len() < 2 {
        // Show main menu if no arguments provided
        bot.send_group_msg(group_id, MENU).await;
        return;
    }

    // Handle different commands
    match args[1].to_lowercase().as_str() {
        "menu" => {
            bot.send_group_msg(group_id, MENU).await;
        }
        "images" => {
            if args.len() < 3 {
                bot.send_group_msg(group_id, IMAGES).await;
                return;
            }
            match args[2].to_lowercase().as_str() {
                "-cats" => {
                    bot.send_group_msg(group_id, "Here are some cute cats! 🐱").await;
                }
                "-coser" => {
                    bot.send_group_msg(group_id, "Here are some coser images!").await;
                }
                "-search" => {
                    if args.len() < 4 {
                        bot.send_group_msg(
                            group_id,
                            "Usage: /np images -search [key]",
                        ).await;
                        return;
                    }
                    let search_key = &args[3..].join(" ");
                    bot.send_group_msg(
                        group_id,
                        format!("Searching for images with key: {}", search_key),
                    ).await;
                }
                _ => {
                    bot.send_group_msg(group_id, IMAGES).await;
                }
            }
        }
        "others" => {
            bot.send_group_msg(group_id, "OTHERS\n--\n -feature1\n -feature2").await;
        }
        _ => {
            bot.send_group_msg(group_id, "Unknown command. Type /np menu for help.").await;
        }
    }
}

// 菜单列表
static MENU: &str = "MENU\n--\n -menu\n -images\n -others";
// 图片菜单
static IMAGES: &str = "IMAGES\n--\n -cats\n -coser\n\n -search [key]";
