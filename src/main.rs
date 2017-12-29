extern crate ath;

use ath::logger::{Logger, Level};

const ELITE: Level = Level("ELITE", 44);
const OWNER: Level = Level("OWNER", 112);

fn main() {
  let mut log = Logger::new();
  log.levels.add(&ELITE);
  log.levels.add(&OWNER);

  println!("Log Level: {}", log.levels.current.name());

  log.debug("Hello, World!");
  log.info("Bootstrapping Aethereal v1.1.2");
  log.verbose("Hello, world!");

  log.log("Elites and Above!", ELITE);
  log.log("Hey, Owner Only!", OWNER);
}
