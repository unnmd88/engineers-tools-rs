#![cfg_attr(not(debug_assertions), windows_subsystem = "console")]

mod models;
mod handlers;
mod server;
mod docs;

use webbrowser; 

use std::thread;
use server::run_server;
use std::process;  // <-- Этого импорта не хватало
use single_instance::SingleInstance;


/// Проверяет, что приложение запущено в единственном экземпляре
fn ensure_single_instance() -> SingleInstance {
    const APP_ID: &str = "traffic-core-api-v1";
    
    // SingleInstance::new возвращает Result, обрабатываем его правильно
    let instance = match SingleInstance::new(APP_ID) {
        Ok(inst) => inst,
        Err(e) => {
            eprintln!("❌ FATAL: Не удалось создать системную блокировку");
            eprintln!("   Ошибка: {}", e);
            eprintln!("   Приложение будет остановлено.");
            process::exit(1);
        }
    };
    
    if !instance.is_single() {
        eprintln!("❌ FATAL: Обнаружен другой запущенный экземпляр!");
        eprintln!("   Traffic Core API уже работает в системе.");
        eprintln!();
        eprintln!("💡 Решения:");
        eprintln!("   1. Найдите и закройте существующий процесс");
        eprintln!("   2. Если процесс не найден, удалите файл блокировки:");
        eprintln!("      /tmp/single-instance-{}", APP_ID);
        eprintln!("   3. Перезапустите приложение");
        
        process::exit(1);
    }
    
    println!("🔒 Single instance check: ✓ PASSED");
    instance
}


/// Открывает браузер после запуска сервера
fn open_browser() {
    // Даём серверу секунду на запуск
    thread::sleep(std::time::Duration::from_secs(1));
    
    println!("🌐 Открываю браузер...");
    let url = "http://localhost:3000/swagger-ui";
    
    if let Err(e) = webbrowser::open(url) {
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
    

    // Проверяем единственный экземпляр
    let _guard = ensure_single_instance();
    println!();


    print_instructions();

    // Запускаем открытие браузера в отдельном потоке
    thread::spawn(|| {
        open_browser();
    });
    
    // Запускаем сервер
    run_server().await;
}