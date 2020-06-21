use std::{thread, time};
use rand::Rng;

#[test]
fn sample_test() {
  let mut rng = rand::thread_rng();
  let seconds = time::Duration::new(rng.gen_range(0,120),0);

  thread::sleep(seconds);
  assert!(true);
}
