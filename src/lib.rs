use kovi::{AllMsgEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;
use regex::Regex;

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();
    P::on_group_msg(move |e| on_group_msg(e, bot.clone()));
}

async fn on_group_msg(e: Arc<AllMsgEvent>, bot: Arc<RuntimeBot>) {
    // 获取消息文本
    let text = match e.borrow_text() {
        Some(v) => v,
        None => return, // 如果消息没有文本内容，直接返回
    };

    // 检查命令前缀
    if !text.starts_with("/nxetcp") {
        return; // 如果消息不是以 /nxetcp 开头，直接返回
    }

    // 使用正则表达式解析命令
    let re = Regex::new(r"^/nxetcp\s+-ban\s+(\d+)\s+(\d+)$").unwrap();
    let captures = match re.captures(text) {
        Some(caps) => caps,
        None => {
            // 如果命令格式不正确，向用户发送提示信息
            if let Some(group_id) = e.group_id {
                let _ = bot.send_group_msg(group_id, "命令格式错误！正确格式为：/nxetcp -ban [QQ号] [禁言时间(秒)]").await;
            }
            return;
        }
    };

    // 提取参数
    let qq_id = captures.get(1).unwrap().as_str().parse::<u64>().unwrap(); // QQ号
    let ban_duration = captures.get(2).unwrap().as_str().parse::<u32>().unwrap(); // 禁言时间

    // 检查群号是否存在
    let group_id = match e.group_id {
        Some(group_id) => group_id,
        None => {
            eprintln!("未找到群号，无法执行禁言操作！");
            return;
        }
    };

    // 执行禁言操作
    match bot.set_group_ban(group_id, qq_id, ban_duration).await {
        Ok(_) => {
            // 如果禁言成功，发送确认消息
            let _ = bot.send_group_msg(group_id, format!("已成功禁言用户 {}，时长 {} 秒。", qq_id, ban_duration)).await;
        }
        Err(err) => {
            // 如果禁言失败，记录错误日志并向用户反馈
            eprintln!("禁言操作失败：{}", err);
            let _ = bot.send_group_msg(group_id, "禁言操作失败，请稍后重试或检查命令格式！").await;
        }
    }
}
