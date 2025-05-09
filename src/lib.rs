use kovi::{NoticeEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;

#[kovi::plugin]
fn main() {
    let bot = P::get_runtime_bot_sync(); // 假设有同步版本的获取 bot 方法
    
    // 注册通知处理器（假设 P::on_all_notice 有同步版本）
    P::on_all_notice_sync(move |e| {
        on_notice_event(e, bot.clone());
    });
}

/// 处理通知事件
fn on_notice_event(e: Arc<NoticeEvent>, bot: Arc<RuntimeBot>) {
    match e.notice_type.as_str() {
        "group_decrease" => handle_group_decrease(e, bot),
        "group_increase" => handle_group_increase(e, bot),
        "group_admin" => handle_group_admin_change(e, bot),
        _ => println!("Received unknown notice type: {}", e.notice_type),
    }
}

/// 处理群成员减少事件
fn handle_group_decrease(e: Arc<NoticeEvent>, bot: Arc<RuntimeBot>) {
    println!("群成员减少: {}", e.original_json);
    
    // 如果需要发送消息，需要确保 bot.send_group_msg 是同步的
    // if let Some(group_id) = e.group_id {
    //     if let Err(e) = bot.send_group_msg_sync(group_id, "有人离开了群聊") {
    //         eprintln!("发送消息失败: {}", e);
    //     }
    // }
}

/// 处理群成员增加事件
fn handle_group_increase(e: Arc<NoticeEvent>, bot: Arc<RuntimeBot>) {
    println!("群成员增加: {}", e.original_json);
    // 可以添加欢迎消息等逻辑
}

/// 处理群管理员变更事件
fn handle_group_admin_change(e: Arc<NoticeEvent>, bot: Arc<RuntimeBot>) {
    println!("群管理员变更: {}", e.original_json);
    // 可以添加相应处理逻辑
}
