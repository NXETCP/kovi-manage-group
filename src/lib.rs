use kovi::{MsgEvent, PluginBuilder as P, RuntimeBot};
use std::sync::{Arc, Mutex};
use regex::Regex;
use std::io::{self, Write};

// 全局存储主人 QQ 号
lazy_static::lazy_static! {
    static ref OWNER: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(None));
}

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();

    // 初始化主人
    initialize_owner();

    // 监听群聊消息
    P::on_group_msg(move |e| on_group_msg(e, bot.clone()));
}

// 初始化主人 QQ 号
fn initialize_owner() {
    let mut owner = OWNER.lock().unwrap();
    if owner.is_none() {
        println!("请输入默认主人的 QQ 号：");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let qq_id: i64 = input.trim().parse().expect("请输入有效的 QQ 号！");
        *owner = Some(qq_id);
        println!("默认主人已设置为：{}", qq_id);
    }
}

async fn on_group_msg(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>) {
    let text = match e.borrow_text() {
        Some(v) => v,
        None => return,
    };

    if !text.starts_with("/nxetcp") {
        return;
    }

    if let Some(captures) = Regex::new(r"^/nxetcp\s+-setowner\s+(\d+)$").unwrap().captures(text) {
        handle_set_owner_command(e.clone(), bot.clone(), captures).await;
    } else if let Some(captures) = Regex::new(r"^/nxetcp\s+-ban\s+(\d+)\s+(\d+)$").unwrap().captures(text) {
        handle_ban_command(e.clone(), bot.clone(), captures);
    } else if let Some(captures) = Regex::new(r"^/nxetcp\s+-wholeban\s+(on|off)$").unwrap().captures(text) {
        handle_whole_ban_command(e.clone(), bot.clone(), captures);
    } else if let Some(group_id) = e.group_id {
        bot.send_group_msg(group_id, "命令格式错误！支持的命令有：\n/nxetcp -setowner [QQ号]\n/nxetcp -ban [QQ号] [禁言时间(秒)]\n/nxetcp -wholeban [on/off]").await;
    }
}

// 检查是否是主人
fn is_owner(user_id: i64) -> bool {
    let owner = OWNER.lock().unwrap();
    match *owner {
        Some(qq_id) => qq_id == user_id,
        None => false,
    }
}

// 处理设置主人命令
async fn handle_set_owner_command(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>, captures: regex::Captures<'_>) {
    let group_id = match e.group_id {
        Some(group_id) => group_id,
        None => {
            eprintln!("未找到群号，无法执行操作！");
            return;
        }
    };

    let user_id = e.user_id.unwrap_or(0);
    if !is_owner(user_id) {
        bot.send_group_msg(group_id, "只有当前主人可以设置新的主人！").await;
        return;
    }

    let new_owner: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
    {
        let mut owner = OWNER.lock().unwrap();
        *owner = Some(new_owner);
    }

    bot.send_group_msg(group_id, format!("新的主人已设置为：{}", new_owner)).await;
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

    let user_id = e.user_id.unwrap_or(0);
    if !is_owner(user_id) {
        bot.send_group_msg(group_id, "只有主人可以执行禁言操作！");
        return;
    }

    bot.set_group_ban(group_id, qq_id, ban_duration);
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

    let user_id = e.user_id.unwrap_or(0);
    if !is_owner(user_id) {
        bot.send_group_msg(group_id, "只有主人可以执行全体禁言操作！");
        return;
    }

    bot.set_group_whole_ban(group_id, enable_ban);
    bot.send_group_msg(
        group_id,
        if enable_ban {
            "已开启全体禁言。"
        } else {
            "已关闭全体禁言。"
        },
    );
}
