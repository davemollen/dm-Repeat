mod utils;
use repeat::Repeat;
use utils::generate_signal;

fn main() {
  let mut repeat = Repeat::new(44100.);

  loop {
    let input = generate_signal();
    repeat.process(input, 100., 16, 1., -0.25);
  }
}
