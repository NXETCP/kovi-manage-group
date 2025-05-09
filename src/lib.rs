use kovi::{NoticeEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();
    P::on_all_notice(move |e| on_group_msg(e, bot.clone()));
}

async fn on_group_msg(e: Arc<NoticeEvent>, bot: Arc<RuntimeBot>) {
    match e.notice_type.as_str() {
        "group_decrease" => println!("{}", e.original_json)
    }
}
