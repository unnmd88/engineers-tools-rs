#![cfg_attr(not(debug_assertions), windows_subsystem = "console")]

mod server;
mod docs;
mod shared;
mod features;

// use utoipa_swagger_ui::Config;
use webbrowser; 

use std::thread;
use server::run_server;
use single_instance::SingleInstance;

use crate::server::Config;

/// Открывает браузер после запуска сервера
fn open_browser(url: String) {

    // Даём серверу секунду на запуск
    thread::sleep(std::time::Duration::from_secs(1));
    
    println!("🌐 Открываю браузер...");

    if let Err(e) = webbrowser::open(&url) {
        eprintln!("⚠️  Не удалось автоматически открыть браузер: {}", e);
        println!("🔗 Пожалуйста, откройте вручную: {}", url);
    } else {
        println!("✅ Браузер открыт");
    }
}


fn print_instructions(config: &Config) {  // ПЕРЕДАЁМ config
    println!("\n{}", "═".repeat(60));
    println!("📋 ИНСТРУКЦИЯ ПО ИСПОЛЬЗОВАНИЮ:");
    println!("{}", "─".repeat(60));

    println!("🌐 Swagger UI:  http://{}:{}/swagger-ui", config.host, config.port);
    println!("📊 Проверка:    http://{}:{}/health", config.host, config.port);
    println!("ℹ️  Инфо:        http://{}:{}/info", config.host, config.port);  // исправил двойной слеш

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
    
    let config = Config::from_env();
    
    let instance = SingleInstance::new("traffic-core-api").unwrap();
    if !instance.is_single() {
        eprintln!("❌ Программа уже запущена!");
        eprintln!("   Можно запустить только один экземпляр");
        std::process::exit(1);
    }
    println!();
    print_instructions(&config);

    let url = format!("http://{}:{}", config.host, config.port);

    // Запускаем открытие браузера в отдельном потоке
    thread::spawn(|| {
        open_browser(url);
    });
    
    // Запускаем сервер
    run_server(config).await;
}