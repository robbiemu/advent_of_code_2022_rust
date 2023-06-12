pub fn interpret_command(index: usize, line: String) -> (String, Option<i32>) {
  let mut il = line.split_whitespace();
  let cmd: String = il
    .next()
    .unwrap_or_else(|| panic!("invalid format on line {}: {}", index, line))
    .to_string();
  if !["addx".to_string(), "noop".to_string()].contains(&cmd) {
    panic!("invalid format on line {}: {}", index, line)
  }
  let mut v = None;
  if let Some(x) = il.next() {
    v = Some(x.parse::<i32>().unwrap_or_else(|_| {
      panic!("invalid format on line {}: {}", index, line)
    }));
  }

  (cmd, v)
}
