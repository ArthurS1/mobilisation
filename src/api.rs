pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn internal() {
    println!("It works !");
    let result = add(1, 1);
    assert_eq!(result, 2);
  }

}
