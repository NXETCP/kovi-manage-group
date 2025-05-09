use kovi::{NoticeEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;
//use regex::Regex;

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();
    P::on_all_notice(move |e| on_group_msg(e, bot.clone()));
}

async fn on_group_msg(e: Arc<NoticeEvent>, bot: Arc<RuntimeBot>) {
    
    if let Some(group_id) = e.group_id {
        bot.send_group_msg(group_id, e.notice_type);
    }
    /*let text = match e.borrow_text() {
        Some(v) => v,
        None => return,
    };
    
    if !text.starts_with("/np") {
        return;
    }
    
    if let Some(group_id) = e.group_id {
        match text {
            "/np menu" => bot.send_group_msg(group_id, MENU),
            "/np images" => bot.send_group_msg(group_id, IMAGES),
            "/np manage" => bot.send_group_msg(group_id, MANAGE),
            _ => {}
        }
    }*/
}
/*
static MENU: &str = "MENU\n--\n -manage\n -images\n -others";
static IMAGES: &str = "IMAGES\n--\n -cats\n -coser\n\n -search [key]";
static MANAGE: &str = "MANAGE\n--\n -ban [uin] [time]\n -all_ban [off/on]";*/
