fn main() {
  println!("Hello, world!");
  let mut s = String::from("hello");
  change(&mut s);

  println!("s: {}", s);

  let s = "hello world";

  println!("{}", first_word(s));
}

fn change(some_string: &mut String) {
  some_string.push_str(", world")
}

fn first_word(s: &str) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  s.len()
}
