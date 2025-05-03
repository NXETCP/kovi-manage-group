use kovi::{MsgEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();
    P::on_group_msg(move |e| async move {
        if let Err(e) = on_group_msg(e, bot.clone()).await {
            eprintln!("Command handling error: {:?}", e);
        }
    });
}

async fn on_group_msg(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>) -> Result<(), Box<dyn std::error::Error>> {
    // 显式获取群组ID
    let group_id = e.group_id.ok_or("Not a group message")?;

    // 获取消息文本
    let text = e.borrow_text().ok_or("No message text")?;

    // 检查命令前缀
    if !text.starts_with("/np") {
        return Ok(());
    }

    // 解析参数
    let args: Vec<&str> = text.trim_start_matches('/').trim().split_whitespace().collect();
    if args.is_empty() {
        bot.send_group_msg(group_id, MENU).await?;
        return Ok(());
    }

    // 命令处理
    match args[0].to_lowercase().as_str() {
        "menu" => bot.send_group_msg(group_id, MENU).await?,
        "images" => handle_images(&args, group_id, bot).await?,
        "others" => bot.send_group_msg(group_id, OTHERS).await?,
        _ => bot.send_group_msg(group_id, "Unknown command. Use /np menu").await?,
    }

    Ok(())
}

async fn handle_images(args: &[&str], group_id: i64, bot: Arc<RuntimeBot>) -> Result<(), Box<dyn std::error::Error>> {
    match args.get(0).map(|s| s.to_lowercase().as_str()) {
        Some("-cats") => bot.send_group_msg(group_id, "Cute cats coming soon... 🐱").await?,
        Some("-coser") => bot.send_group_msg(group_id, "Coser images loading...").await?,
        Some("-search") => {
            let query = args.get(1..).map(|v| v.join(" ")).unwrap_or_default();
            bot.send_group_msg(group_id, format!("Searching for: {}", query)).await?;
        }
        _ => bot.send_group_msg(group_id, IMAGES).await?,
    }
    Ok(())
}

// 菜单常量
const MENU: &str = "MENU
-- 
 - menu
 - images
 - others";

const IMAGES: &str = "IMAGES
-- 
 - cats
 - coser
 - search [key]";
