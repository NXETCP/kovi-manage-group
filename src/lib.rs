use kovi::{MsgEvent, PluginBuilder as P, RuntimeBot};
use std::sync::Arc;
use reqwest::Error;
use serde_json::{json, Value};
use kovi::utils::load_json_data;
use std::path::PathBuf;

#[kovi::plugin]
async fn main() {
    let bot = P::get_runtime_bot();
    let data_path = bot.get_data_path();
    let config_path = data_path.join("config.json");
    
    // Initialize config if it doesn't exist
    if !config_path.exists() {
        let default_config = json!({
            "owners": [2570266883] // Default owner (replace with your UIN)
        });
        std::fs::write(config_path, default_config.to_string()).unwrap();
    }
    
    P::on_group_msg(move |e| on_group_msg(e, bot.clone(), config_path.clone()));
}

async fn on_group_msg(e: Arc<MsgEvent>, bot: Arc<RuntimeBot>, config_path: PathBuf) {
    let text = match e.borrow_text() {
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
            "/np manage" => {
                if let Some(owner) = e.sender_uin {
                    if is_owner(&owner, &config_path).await {
                        bot.send_group_msg(group_id, MANAGE);
                    } else {
                        bot.send_group_msg(group_id, "Permission denied. Only owners can use this command.");
                    }
                }
            },
            "/np add_owner [uin]" => {
                if let Some(owner) = e.sender_uin {
                    if is_owner(&owner, &config_path).await {
                        if let Some(target_uin) = extract_uin(&text) {
                            if add_owner(target_uin, &config_path).await {
                                bot.send_group_msg(group_id, format!("Successfully added owner: {}", target_uin));
                            } else {
                                bot.send_group_msg(group_id, "Failed to add owner or UIN already exists.");
                            }
                        } else {
                            bot.send_group_msg(group_id, "Please provide a valid UIN after the command.");
                        }
                    } else {
                        bot.send_group_msg(group_id, "Permission denied. Only owners can use this command.");
                    }
                }
            },
            "/np remove_owner [uin]" => {
                if let Some(owner) = e.sender_uin {
                    if is_owner(&owner, &config_path).await {
                        if let Some(target_uin) = extract_uin(&text) {
                            if remove_owner(target_uin, &config_path).await {
                                bot.send_group_msg(group_id, format!("Successfully removed owner: {}", target_uin));
                            } else {
                                bot.send_group_msg(group_id, "Failed to remove owner or UIN not found.");
                            }
                        } else {
                            bot.send_group_msg(group_id, "Please provide a valid UIN after the command.");
                        }
                    } else {
                        bot.send_group_msg(group_id, "Permission denied. Only owners can use this command.");
                    }
                }
            },
            _ => {}
        }
    }
}

static MENU: &str = "MENU\n--\n -manage\n -images\n -others\n -owner_management";
static IMAGES: &str = "IMAGES\n--\n -cats\n -coser\n\n -search [key]";
static MANAGE: &str = "MANAGE\n--\n -ban [uin] [time]\n -all_ban [off/on]\n -add_owner [uin]\n -remove_owner [uin]";

async fn is_owner(uin: &i64, config_path: &PathBuf) -> bool {
    if let Ok(config_data) = load_json_data(config_path).await {
        if let Some(config) = config_data.as_object() {
            if let Some(owners) = config.get("owners") {
                if let Some(owners_array) = owners.as_array() {
                    return owners_array.iter().any(|o| o.as_i64() == Some(*uin));
                }
            }
        }
    }
    false
}

async fn add_owner(uin: i64, config_path: &PathBuf) -> bool {
    if let Ok(mut config_data) = load_json_data(config_path).await {
        if let Some(config) = config_data.as_object_mut() {
            if let Some(owners) = config.get_mut("owners") {
                if let Some(owners_array) = owners.as_array_mut() {
                    if !owners_array.contains(&json!(uin)) {
                        owners_array.push(json!(uin));
                        return save_config(config_data, config_path).await;
                    }
                } else {
                    config.insert("owners".to_string(), json!([uin]));
                    return save_config(config_data, config_path).await;
                }
            } else {
                config.insert("owners".to_string(), json!([uin]));
                return save_config(config_data, config_path).await;
            }
        }
    }
    false
}

async fn remove_owner(uin: i64, config_path: &PathBuf) -> bool {
    if let Ok(mut config_data) = load_json_data(config_path).await {
        if let Some(config) = config_data.as_object_mut() {
            if let Some(owners) = config.get_mut("owners") {
                if let Some(owners_array) = owners.as_array_mut() {
                    if let Some(pos) = owners_array.iter().position(|o| o.as_i64() == Some(uin)) {
                        owners_array.remove(pos);
                        return save_config(config_data, config_path).await;
                    }
                }
            }
        }
    }
    false
}

async fn save_config(config_data: Value, config_path: &PathBuf) -> bool {
    match serde_json::to_string_pretty(&config_data) {
        Ok(json_str) => {
            if let Ok(_) = std::fs::write(config_path, json_str) {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

fn extract_uin(text: &str) -> Option<i64> {
    let parts: Vec<&str> = text.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }
    
    // Try to parse the UIN from the command
    parts[1].parse::<i64>().ok()
}
