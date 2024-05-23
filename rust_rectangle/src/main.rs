mod rectangle; 

  fn main() {
      let rect = rectangle::Rectangle { width: 10, height: 20 };
      let area = rect.area();
  
      println!("The area of the rectangle is: {}", area);
  }
    