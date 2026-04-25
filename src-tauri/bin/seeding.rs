use anyhow::Result;
mod seed_items;
mod seed_prices;
mod seed_user;

fn main() -> Result<()> {
    println!("🚀 Rozpoczynam proces seedowania danych...");

    println!("📦 Generowanie przedmiotów...");
    seed_items::run()?;
    //
    println!("💰 Generowanie cen...");
    seed_prices::run()?;
    // 
    // println!("👤 Generowanie danych użytkownika...");
    // seed_user::run()?;

    println!("✅ Sukces! Wszystkie dane zostały pomyślnie zapisane.");

    Ok(())
}