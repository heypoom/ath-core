extern crate ath;

use ath::logger::Logger;

fn main() {
  let mut log = Logger::new();
  log.debug("Hello, World!");
  log.info("Bootstrapping Aethereal v1.1.2");
  log.verbose("Hello, world!");

  log.print_logs();
}
