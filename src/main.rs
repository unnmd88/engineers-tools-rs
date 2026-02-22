#![cfg_attr(not(debug_assertions), windows_subsystem = "console")]

mod models;
mod handlers;
mod server;
mod docs;

use std::thread;
use server::run_server;

/// Открывает браузер после запуска сервера
fn open_browser() {
    // Даём серверу секунду на запуск
    thread::sleep(std::time::Duration::from_secs(1));
    
    println!("🌐 Открываю браузер...");
    let url = "http://localhost:3000/swagger-ui";
    
    if let Err(e) = open::that(url) {
        eprintln!("⚠️  Не удалось автоматически открыть браузер: {}", e);
        println!("🔗 Пожалуйста, откройте вручную: {}", url);
    } else {
        println!("✅ Браузер открыт");
    }
}

fn print_instructions() {
    println!("\n{}", "═".repeat(60));
    println!("📋 ИНСТРУКЦИЯ ПО ИСПОЛЬЗОВАНИЮ:");
    println!("{}", "─".repeat(60));
    println!("🌐 Swagger UI:  http://localhost:3000/swagger-ui");
    println!("📊 Проверка:    http://localhost:3000/health");
    println!("ℹ️  Инфо:        http://localhost:3000/info");
    println!("🔧 Генерация:   POST http://localhost:3000/generate");
    println!("🔧 Парсинг:     POST http://localhost:3000/parse");
    println!("{}", "─".repeat(60));
    println!("🛑 Для остановки сервера нажмите Ctrl+C");
    println!("{}", "═".repeat(60));
    println!(); // Пустая строка для красоты
}

#[tokio::main]
async fn main() {
    println!("╔════════════════════════════════════╗");
    println!("║     Traffic Core API v1.0.0        ║");
    println!("╚════════════════════════════════════╝");
    
    print_instructions();
    
    // Запускаем открытие браузера в отдельном потоке
    thread::spawn(|| {
        open_browser();
    });
    
    // Запускаем сервер
    run_server().await;
}