extern crate chrono;

use std::env;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Level(pub &'static str, pub u32);

impl Level {
  pub fn name(&self) -> &'static str {self.0}
  pub fn id(&self) -> u32 {self.1}
}

const ALL: Level = Level("ALL", 0);
const DEBUG: Level = Level("DEBUG", 1);
const VERBOSE: Level = Level("VERBOSE", 2);
const INFO: Level = Level("INFO", 3);
const WARN: Level = Level("WARN", 4);
const ERROR: Level = Level("ERROR", 5);
const FATAL: Level = Level("FATAL", 6);

// This allows us to fetch from environment variable
#[derive(Debug, Clone)]
pub struct Levels {
  pub levels: HashMap<&'static str, Level>,
  pub current: Level
}

impl Levels {
  pub fn new() -> Levels {
    let levels = HashMap::new();
    let mut levels = Levels { levels, current: ALL };

    levels.add_defaults();
    levels.current = levels.from_env();

    levels
  }

  pub fn from_env(&self) -> Level {
    let level = env::var("LOG_LEVEL").unwrap_or_default();
    let level = self.levels.get(level.to_uppercase().as_str()).unwrap_or(&ALL);

    level.clone()
  }

  fn add_defaults(&mut self) {
    let default_levels = [DEBUG, VERBOSE, INFO, WARN, ERROR, FATAL, ALL];

    for level in default_levels.iter() {
      self.add(&level);
    }
  }

  pub fn add(&mut self, level: &Level) {
    self.levels.insert(level.name(), level.clone());
    self.current = self.from_env();
  }

  pub fn is_in_level(&self, msg_level: Level) -> bool {
    msg_level.id() >= self.current.id()
  }
}

pub struct Message {
  pub text: String,
  pub level: Level,
  pub timestamp: chrono::DateTime<chrono::Local>
}

pub struct Logger {
  pub logs: Vec<Message>,
  pub levels: Levels,
}

impl Logger {
  pub fn new() -> Logger {
    let logs: Vec<Message> = Vec::new();
    let levels = Levels::new();

    Logger { logs, levels }
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

    if self.levels.is_in_level(level) {
      self.console(&log);
      self.logs.push(log);
    }
  }

  pub fn console(&self, log: &Message) {
    let timestamp = log.timestamp.format("%Y-%m-%d %H:%M:%S");
    println!("[{}] ({}) {}", log.level.name(), timestamp, log.text)
  }

  pub fn debug(&mut self, msg: &str) {
    self.log(msg, DEBUG)
  }

  pub fn verbose(&mut self, msg: &str) {
    self.log(msg, VERBOSE)
  }

  pub fn info(&mut self, msg: &str) {
    self.log(msg, INFO)
  }

  pub fn warn(&mut self, msg: &str) {
    self.log(msg, WARN)
  }

  pub fn error(&mut self, msg: &str) {
    self.log(msg, ERROR)
  }

  pub fn fatal(&mut self, msg: &str) {
    self.log(msg, FATAL)
  }
}
