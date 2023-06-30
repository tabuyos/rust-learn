fn main() {
  println!("Hello, world!");

  another_function();

  lift();
}

fn another_function() {
  println!("Another function.");
}

fn lift() {
  for number in (1..=4).rev() {
    println!("{number}!");
  }
  println!("LIFTOFF!!!");
}

