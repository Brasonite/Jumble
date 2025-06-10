use std::{collections::HashMap, sync::LazyLock};

static LOCALES: LazyLock<HashMap<String, HashMap<String, String>>> = LazyLock::new(|| {
    HashMap::from_iter([
        (
            "en".to_string(),
            serde_json::from_str(include_str!("lang/en.json")).unwrap(),
        ),
        (
            "pt-BR".to_string(),
            serde_json::from_str(include_str!("lang/pt-BR.json")).unwrap(),
        ),
    ])
});

#[tauri::command]
pub fn native_localize(key: &str) -> String {
    let locale = tauri_plugin_os::locale().unwrap_or("en".to_string());
    let locale = match LOCALES.contains_key(&locale) {
        true => locale,
        false => "en".to_string(),
    };

    LOCALES[&locale]
        .get(key)
        .map(|x| x.clone())
        .unwrap_or("Lang key doesn't exist".to_string())
}
