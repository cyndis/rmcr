fn main() {
  let a = 1; let b = 2; let c = 3;
  let xs = [a, b, c].map(|&v| v*v);
  io::println(fmt!("%?", xs));
}

