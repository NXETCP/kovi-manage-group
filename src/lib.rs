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

    // 解析命令类型
    if let Some(captures) = Regex::new(r"^/nxetcp\s+-ban\s+(\d+)\s+(\d+)$").unwrap().captures(text) {
        handle_ban_command(e, bot, captures);
    } else if let Some(captures) = Regex::new(r"^/nxetcp\s+-wholeban\s+(on|off)$").unwrap().captures(text) {
        handle_whole_ban_command(e, bot, captures);
    } else if let Some(captures) = Regex::new(r"^/nxetcp\s+-title\s+(\d+)\s+(.+)$").unwrap().captures(text) {
        handle_special_title_command(e, bot, captures);
    } else if let Some(captures) = Regex::new(r"^/nxetcp\s+-admin\s+(\d+)\s+(on|off)$").unwrap().captures(text) {
        handle_admin_command(e, bot, captures);
    } else if let Some(group_id) = e.group_id {
        bot.send_group_msg(group_id, "命令格式错误！\n支持的命令有：\n/nxetcp -ban [QQ号] [禁言时间(秒)]\n/nxetcp -wholeban [on/off]\n/nxetcp -title [QQ号] [专属头衔]\n/nxetcp -admin [QQ号] [on/off]");
    }
}

// 处理禁言命令
fn handle_ban_command(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>, captures: regex::Captures<'_>) {
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

// 处理全体禁言命令
fn handle_whole_ban_command(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>, captures: regex::Captures<'_>) {
    let group_id = match e.group_id {
        Some(group_id) => group_id,
        None => {
            eprintln!("未找到群号，无法执行全体禁言操作！");
            return;
        }
    };

    let enable_ban = captures.get(1).unwrap().as_str() == "on";

    // 调用全体禁言方法
    bot.set_group_whole_ban(group_id, enable_ban);

    // 发送成功消息
    bot.send_group_msg(
        group_id,
        if enable_ban {
            "已开启全体禁言。"
        } else {
            "已关闭全体禁言。"
        },
    );
}

// 处理设置专属头衔命令
fn handle_special_title_command(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>, captures: regex::Captures<'_>) {
    let qq_id: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
    let title = captures.get(2).unwrap().as_str();

    let group_id = match e.group_id {
        Some(group_id) => group_id,
        None => {
            eprintln!("未找到群号，无法设置专属头衔！");
            return;
        }
    };

    // 调用设置专属头衔方法
    bot.set_group_special_title(group_id, qq_id, title.to_string());

    // 发送成功消息
    bot.send_group_msg(
        group_id,
        format!("已为用户 {} 设置专属头衔：{}", qq_id, title),
    );
}

// 处理设置管理员命令
fn handle_admin_command(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>, captures: regex::Captures<'_>) {
    let qq_id: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
    let make_admin = captures.get(2).unwrap().as_str() == "on";

    let group_id = match e.group_id {
        Some(group_id) => group_id,
        None => {
            eprintln!("未找到群号，无法设置管理员！");
            return;
        }
    };

    // 调用设置管理员方法
    bot.set_group_admin(group_id, qq_id, make_admin);

    // 发送成功消息
    bot.send_group_msg(
        group_id,
        if make_admin {
            format!("已将用户 {} 设置为管理员。", qq_id)
        } else {
            format!("已取消用户 {} 的管理员权限。", qq_id)
        },
    );
}
