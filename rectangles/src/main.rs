fn main() {
  let width1 = 30;
  let height1 = 50;

  let rect1 = (30, 50);

  let rect2 = Rectangle {
    width: dbg!(30 * 2),
    height: 50,
  };

  println!("rect2 is {:#?}", rect2);
  dbg!(&rect2);

  println!("The area of the rectangle is {} square pixels", area(width1, height1));
  println!("The area of the rectangle is {} square pixels", area1(rect1));
  println!("The area of the rectangle is {} square pixels", rect2.area());
  println!("The area of the rectangle is {} square pixels", area2(rect2));


  let m = Message::Write(String::from("hello"));
  m.call();
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn area2(rect: Rectangle) -> u32 {
  rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

struct QuitMessage;

// 类单元结构体
struct MoveMessage {
  x: i32,
  y: i32,
}

// 元组结构体
struct WriteMessage(String);

// 元组结构体
struct ChangeColorMessage(i32, i32, i32);

impl Message {
  fn call(&self) {
    // 在这里定义方法体
    println!("ok");
  }
}