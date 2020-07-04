fn main() {
  // Variables and Mutability
  let mut x1 = 5;
  println!("The value of x is: {}", x1);
  x1 = 6;
  println!("The value of x is: {}", x1);

  // Differences Between Variables and Constants
  const MAX_POINTS: u32 = 100_000;
  println!("The value of MAX_POINTS is: {}", MAX_POINTS);

  // Shadowing
  let x2 = 5;
  let x2 = x2 + 1;
  let x2 = x2 * 2;
  println!("The value of x is: {}", x2);

  let spaces = "   ";
  let spaces = spaces.len();
  println!("The value of spaces is: {}", spaces);
}