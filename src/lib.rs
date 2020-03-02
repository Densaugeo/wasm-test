#[no_mangle]
pub extern fn add_one(x: u8) -> usize {
  let mut array = [x, x + 1, x + 2, x + 3];
  
  array.as_mut_ptr() as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
