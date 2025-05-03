use kovi::{AllMsgEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();
    P::on_group_msg(move |e| on_group_msg(e, bot.clone()));
}

async fn on_group_msg(e: Arc<AllMsgEvent>, bot: Arc<RuntimeBot>) {
    let text = match e.borrow_text() {
        Some(v) => v,
        None => return,
    };

    if !text.starts_with("/nxetcp") {
        return;
    }

    let title = text.trim_start_matches("nxetcp -ban [0-9] [0-9]").trim().to_string(); //禁言操作 第一个是qq 第2个是时间 

    bot.set_group_ban(e.group_id.unwrap(), title[0], title[1]);  //https://kovi.thricecola.com/api/onebot_api.html#bot-set-group-ban-%E7%BE%A4%E7%BB%84%E5%8D%95%E4%BA%BA%E7%A6%81%E8%A8%80
}
