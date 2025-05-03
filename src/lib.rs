use kovi::{MsgEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();
    P::on_group_msg(move |e| on_group_msg(e, bot.clone()));
}

fn on_group_msg(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>) {
    let text = match e.borrow_text() {
        Some(v) => v,
        None => return,
    };

    if !text.starts_with("/np") {
        return;
    }

    let args: Vec<&str> = text.split_whitespace().collect();
    if args.len() < 2 {
        bot.send_group_msg(e.group_id(), MENU);
        return;
    }

    match args[1].to_lowercase().as_str() {
        "menu" => {
            bot.send_group_msg(e.group_id(), MENU);
        }
        "images" => {
            if args.len() < 3 {
                bot.send_group_msg(e.group_id(), IMAGES);
                return;
            }
            match args[2].to_lowercase().as_str() {
                "-cats" => {
                    bot.send_group_msg(e.group_id(), "Here are some cute cats! 🐱");
                }
                "-coser" => {
                    bot.send_group_msg(e.group_id(), "Here are some coser images!");
                }
                "-search" => {
                    if args.len() < 4 {
                        bot.send_group_msg(e.group_id(), "Usage: /np images -search [key]");
                        return;
                    }
                    let search_key = &args[3..].join(" ");
                    bot.send_group_msg(
                        e.group_id(),
                        format!("Searching for images with key: {}", search_key),
                    )
                    ;
                }
                _ => {
                    bot.send_group_msg(e.group_id(), IMAGES);
                }
            }
        }
        "others" => {
            bot.send_group_msg(e.group_id(), "OTHERS\n--\n -feature1\n -feature2");
        }
        _ => {
            bot.send_group_msg(e.group_id(), "Unknown command. Type /np menu for help.");
        }
    }
}

// 菜单列表
static MENU: &str = "MENU\n--\n -menu\n -images\n -others";
// 图片菜单
static IMAGES: &str = "IMAGES\n--\n -cats\n -coser\n\n -search [key]";
