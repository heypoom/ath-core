extern crate chrono;

use std::env;

#[derive(Clone, Debug)]
pub enum Level {
  Debug,
  Verbose,
  Info,
  Warn,
  Error,
  Fatal,
  All
}

impl Level {
  pub fn from_env() -> Level {
    let level = match env::var("LOG_LEVEL") {
      Ok(l) => l,
      Err(_) => String::from("")
    };

    let level = match level.to_uppercase().as_str() {
      "DEBUG" => Level::Debug,
      "VERBOSE" => Level::Verbose,
      "INFO" => Level::Info,
      "WARN" => Level::Warn,
      "ERROR" => Level::Error,
      "FATAL" => Level::Fatal,
      _ => Level::All
    };

    level
  }

  pub fn as_num(level: Level) -> u32 {
    match level {
      Level::Debug => 0,
      Level::Verbose => 1,
      Level::Info => 2,
      Level::Warn => 3,
      Level::Error => 4,
      Level::Fatal => 5,
      Level::All => 6
    }
  }

  pub fn is_in_level(msg_level: Level, logger_level: Level) -> bool {
    Level::as_num(msg_level) >= Level::as_num(logger_level)
  }
}

pub struct Message {
  text: String,
  level: Level,
  timestamp: chrono::DateTime<chrono::Local>
}

pub struct Logger {
  logs: Vec<Message>,
  level: Level
}

impl Logger {
  pub fn new() -> Logger {
    let logs: Vec<Message> = Vec::new();
    let level = Level::from_env();

    println!("Log Level: {:?}", level);

    Logger { logs, level }
  }

  pub fn print_logs(&self) {
    println!("Here are all logs:");

    for log in &self.logs {
      self.console(&log);
    }
  }

  pub fn log(&mut self, msg: &str, level: Level) {
    let log = Message {
      text: String::from(msg),
      level: level.clone(),
      timestamp: chrono::Local::now()
    };

    if Level::is_in_level(level, self.level.clone()) {
      self.console(&log);
      self.logs.push(log);
    }
  }

  pub fn console(&self, log: &Message) {
    let timestamp = log.timestamp.format("%Y-%m-%d %H:%M:%S");
    println!("[{:?}] ({}) {}", log.level, timestamp, log.text)
  }

  pub fn debug(&mut self, msg: &str) {
    self.log(msg, Level::Debug)
  }

  pub fn verbose(&mut self, msg: &str) {
    self.log(msg, Level::Verbose)
  }

  pub fn info(&mut self, msg: &str) {
    self.log(msg, Level::Info)
  }

  pub fn warn(&mut self, msg: &str) {
    self.log(msg, Level::Warn)
  }

  pub fn error(&mut self, msg: &str) {
    self.log(msg, Level::Error)
  }

  pub fn fatal(&mut self, msg: &str) {
    self.log(msg, Level::Fatal)
  }
}
