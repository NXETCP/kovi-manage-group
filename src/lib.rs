use kovi::{NoticeEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();
    
    // 注册通知处理器
    P::on_all_notice(move |e| {
        on_notice_event(e, bot.clone()).await;
    });
}

/// 处理通知事件
async fn on_notice_event(e: Arc<NoticeEvent>, bot: Arc<RuntimeBot>) {
    match e.notice_type.as_str() {
        "group_decrease" => handle_group_decrease(e, bot).await,
        "group_increase" => handle_group_increase(e, bot).await,
        "group_admin" => handle_group_admin_change(e, bot).await,
        _ => println!("Received unknown notice type: {}", e.notice_type),
    }
}

/// 处理群成员减少事件
async fn handle_group_decrease(e: Arc<NoticeEvent>, bot: Arc<RuntimeBot>) {
    println!("群成员减少: {}", e.original_json);
    
    // 这里可以添加更多处理逻辑，比如：
    // - 记录日志
    // - 发送通知消息
    // - 更新数据库等
    
    // 示例：如果需要发送消息（需要取消注释相关代码）
    // if let Some(group_id) = e.group_id {
    //     if let Err(e) = bot.send_group_msg(group_id, "有人离开了群聊").await {
    //         eprintln!("发送消息失败: {}", e);
    //     }
    // }
}

/// 处理群成员增加事件
async fn handle_group_increase(e: Arc<NoticeEvent>, bot: Arc<RuntimeBot>) {
    println!("群成员增加: {}", e.original_json);
    // 可以添加欢迎消息等逻辑
}

/// 处理群管理员变更事件
async fn handle_group_admin_change(e: Arc<NoticeEvent>, bot: Arc<RuntimeBot>) {
    println!("群管理员变更: {}", e.original_json);
    // 可以添加相应处理逻辑
}
