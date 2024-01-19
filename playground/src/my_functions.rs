pub fn add_five(num:u32) -> u32 {
  num + 5
}

pub fn add_one(num:u32) -> u32 {
  num + 1
}

pub fn display_address(num_addr: &i32) {
  println!("Address of x is {:p}", num_addr);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_five() {
    assert_eq!(add_five(5), 10);
  }

  #[test]
  fn test_add_one() {
    assert_eq!(add_one(5), 6);
  }
}