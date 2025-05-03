use kovi::{MsgEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;
use tokio::runtime::Runtime; // 需要添加 tokio 依赖

#[kovi::plugin]
async fn main() {
    let rt = Runtime::new().unwrap();
    let bot = P::get_runtime_bot();
    
    P::on_group_msg(move |e| {
        rt.block_on(async move {
            if let Err(e) = on_group_msg(e, bot.clone()).await {
                eprintln!("Error: {:?}", e);
            }
        });
    });
}

fn on_group_msg(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>) -> Result<(), Box<dyn std::error::Error>> {
    let group_id = e.group_id.ok_or("Not a group message")?;
    let text = e.borrow_text().ok_or("No message text")?;

    if !text.starts_with("/np") {
        return Ok(());
    }

    let mut parts = text.trim_start_matches('/').trim().split_whitespace();
    parts.next(); // 跳过 "np"

    match parts.next().ok_or("Missing subcommand")?.to_lowercase().as_str() {
        "menu" => bot.send_group_msg(group_id, MENU)?, // 注意移除了 await
        "images" => handle_image_commands(&parts.collect::<Vec<_>>(), group_id, bot)?,
        "others" => bot.send_group_msg(group_id, OTHERS)?,
        _ => bot.send_group_msg(group_id, UNKNOWN_CMD_MSG)?,
    }

    Ok(())
}

fn handle_image_commands(
    args: &[&str],
    group_id: i64,
    bot: Arc<RuntimeBot>
) -> Result<(), Box<dyn std::error::Error>> {
    match args.first().map(|s| s.to_lowercase().as_str()) {
        Some("cats") => bot.send_group_msg(group_id, CATS_MSG)?,
        Some("coser") => bot.send_group_msg(group_id, COSER_MSG)?,
        Some("search") => {
            let query = args.get(1..).map(|v| v.join(" ")).unwrap_or_default();
            bot.send_group_msg(group_id, format!("Searching: {}", query))?
        }
        Some("all") => bot.send_group_msg(group_id, ALL_IMAGES_MSG)?,
        _ => bot.send_group_msg(group_id, IMAGES)?,
    }
    Ok(())
}

// ---- 常量定义 ----
const MENU: &str = "📜 NP Menu
───
• menu
• images
• others";
const IMAGES: &str = "🖼️ Image Commands
───
• cats
• coser
• search [key]
• all";
const CATS_MSG: &str = "🐱 Cats";
const COSER_MSG: &str = "👗 Cosers";
const ALL_IMAGES_MSG: &str = "📸 All image commands";
const UNKNOWN_CMD_MSG: &str = "❌ Unknown command";
const OTHERS: &str = "🔧 Other commands";
