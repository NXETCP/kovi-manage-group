use kovi::{MsgEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;
use regex::Regex;

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();
    P::on_group_msg(move |e| on_group_msg(e, bot.clone()));
}

async fn on_group_msg(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>) {
    let text = match e.borrow_text() {
        Some(v) => v,
        None => return,
    };

    if !text.starts_with("/nxetcp") {
        return;
    }

    let re = Regex::new(r"^/nxetcp\s+-ban\s+(\d+)\s+(\d+)$").unwrap();
    let captures = match re.captures(text) {
        Some(caps) => caps,
        None => {
            if let Some(group_id) = e.group_id {
                bot.send_group_msg(group_id, "命令格式错误！正确格式为：/nxetcp -ban [QQ号] [禁言时间(秒)]");
            }
            return;
        }
    };

    let qq_id: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
    let ban_duration: usize = captures.get(2).unwrap().as_str().parse().unwrap();

    let group_id = match e.group_id {
        Some(group_id) => group_id,
        None => {
            eprintln!("未找到群号，无法执行禁言操作！");
            return;
        }
    };

    // 调用禁言方法
    bot.set_group_ban(group_id, qq_id, ban_duration);

    // 发送成功消息
    bot.send_group_msg(
        group_id,
        format!("已成功禁言用户 {}，时长 {} 秒。", qq_id, ban_duration),
    );
}
