use kovi::{MsgEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;
use regex::Regex;

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
    
    match text {
        "/np menu" => bot.send_group_msg(e.group_id, MENU),
        "/np images" => bot.send_group_msg(e.group_id, IMAGES),
        "/np manage" => bot.send_group_msg(e.group_id, MANAGE),
        _ => bot.send_group_msg(e.group_id, UNCMD)
    }
}

//菜单列表
static MENU: &str = "MENU\n--\n -manage\n -images\n -others";

//图片菜单
static IMAGES: &str = "IMAGES\n--\n -cats\n -coser\n\n -search [key]";

//群管菜单
static MANAGE: &str = "MANAGE\n--\n -ban [uin] [time]\n -all_ban [off/on]";

//无效指令
static UNCMD: &str = "好像没有这个指令哦喵～";
