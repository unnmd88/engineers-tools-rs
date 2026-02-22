//! Защита от множественного запуска приложения

use std::fs::File;
use std::io;
use std::path::PathBuf;

pub struct SingleInstance {
    lock_file: PathBuf,
    _file: File, // Держим файл открытым, пока приложение живо
}

impl SingleInstance {
    /// Пытается создать единственный экземпляр приложения
    pub fn try_new(app_name: &str) -> io::Result<Self> {
        let mut lock_path = std::env::temp_dir();
        lock_path.push(format!("{}.lock", app_name));
        
        match File::create_new(&lock_path) {
            Ok(file) => {
                println!("🔒 Первый экземпляр приложения");
                Ok(Self {
                    lock_file: lock_path,
                    _file: file,
                })
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "Приложение уже запущено"
                ))
            }
            Err(e) => Err(e),
        }
    }
}

impl Drop for SingleInstance {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.lock_file);
        println!("🔓 Блокировка снята");
    }
}