use kovi::{NoticeEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();
    if let Err(e) = P::on_all_notice(move |e| {
        if let Err(err) = on_group_msg(e, bot.clone()) {
            eprintln!("Error handling notice event: {}", err);
        }
    }) {
        eprintln!("Failed to register notice handler: {}", e);
    }
}

async fn on_group_msg(e: Arc<NoticeEvent>, bot: Arc<RuntimeBot>) -> Result<(), String> {
    match e.notice_type.as_str() {
        "group_decrease" => {
            println!("{}", e.original_json);
            Ok(())
        }
        _ => Ok(()),
    }
}
