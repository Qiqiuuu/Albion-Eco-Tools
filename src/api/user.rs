use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn tauri_invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserData {
    pub specializations: HashMap<String, u32>,
}


pub async fn fetch_player_data() -> UserData {
    let result = tauri_invoke("get_player_data", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).unwrap_or_default()
}


pub async fn send_level_update(name: String, level: u32) {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "name": name,
        "level": level
    })).unwrap_or(JsValue::NULL);

    tauri_invoke("update_level", args).await;
}