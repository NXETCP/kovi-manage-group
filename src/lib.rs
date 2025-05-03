use kovi::{MsgEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;
use regex::Regex;
use std::io::{self};
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)] // 添加 Clone trait
struct Config {
    owner: Option<i64>,
}

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();
    let data_path = bot.get_data_path();
    let config_path = data_path.join("config.json");

    // 初始化主人（从文件读取或提示输入）
    let mut config = initialize_owner(config_path).await;

    // 使用异步闭包修复 `()` is not a future 的问题
    P::on_group_msg(move |e| {
        let bot_clone = bot.clone();
        async move {
            on_group_msg(e, bot_clone, config.clone(), config_path.clone()).await;
        }
    });
}

async fn initialize_owner(config_path: std::path::PathBuf) -> Config {
    if let Ok(config_str) = fs::read_to_string(&config_path) {
        // 如果文件存在且可读取，尝试反序列化
        if let Ok(config) = serde_json::from_str::<Config>(&config_str) {
            return config;
        }
    }

    // 如果没有配置文件，提示用户输入
    println!("请输入默认主人的 QQ 号：");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let qq_id: i64 = input.trim().parse().expect("请输入有效的 QQ 号！");
    
    let new_config = Config { owner: Some(qq_id) };

    // 保存新的主人到配置文件
    save_config(config_path, &new_config).await;

    println!("默认主人已设置为：{}", qq_id);
    new_config
}

async fn save_config(path: std::path::PathBuf, config: &Config) {
    let config_str = serde_json::to_string(config).unwrap();
    fs::write(path, config_str).unwrap();
}

async fn on_group_msg(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>, config: Config, config_path: std::path::PathBuf) {
    let text = match e.borrow_text() {
        Some(v) => v,
        None => return,
    };

    if !text.starts_with("/nxetcp") {
        return;
    }

    if let Some(captures) = Regex::new(r"^/nxetcp\s+-setowner\s+(\d+)$").unwrap().captures(text) {
        handle_set_owner_command(e.clone(), bot.clone(), captures, &config, config_path.clone()).await;
    } else if let Some(captures) = Regex::new(r"^/nxetcp\s+-ban\s+(\d+)\s+(\d+)$").unwrap().captures(text) {
        handle_ban_command(e.clone(), bot.clone(), captures).await;
    } else if let Some(group_id) = e.group_id {
        bot.send_group_msg(
            group_id,
            "命令格式错误！支持的命令有：\n/nxetcp -setowner [QQ号]\n/nxetcp -ban [QQ号] [禁言时间(秒)]\n/nxetcp -wholeban [on/off]",
        );
    }
}

fn is_owner(config: &Config, user_id: i64) -> bool {
    if let Some(owner_id) = config.owner {
        return owner_id == user_id;
    }
    false
}

async fn handle_set_owner_command(
    e: Arc<MsgEvent>, 
    bot: Arc<RuntimeBot>, 
    captures: regex::Captures<'_>, 
    config: &Config, 
    config_path: std::path::PathBuf
) {
    let group_id = match e.group_id {
        Some(group_id) => group_id,
        None => {
            eprintln!("未找到群号，无法执行操作！");
            return;
        }
    };

    let user_id = e.user_id;
    if !is_owner(config, user_id) {
        bot.send_group_msg(group_id, "只有当前主人可以设置新的主人！");
        return;
    }

    let new_owner: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
    let mut new_config = config.clone(); // 克隆 config
    new_config.owner = Some(new_owner);  // 修改 owner

    // 保存新的主人到配置文件
    save_config(config_path, &new_config).await;

    bot.send_group_msg(group_id, format!("新的主人已设置为：{}", new_owner));
}
