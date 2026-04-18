use std::fs;
use std::path::PathBuf;
use aet_shared::models::user::UserData;

pub fn run() -> anyhow::Result<()> {
    let data = UserData::default();
    let home = std::env::var("HOME")?;
    let path = PathBuf::from(home).join(".config/Albion Economy Tools/user.json");

    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, serde_json::to_string_pretty(&data)?)?;

    println!("Done seeding user data");
    Ok(())
}