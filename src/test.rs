#![cfg(test)]

use midpad::rightpad_in_place;
#[test]
fn test_in_place() {
    let mut test = vec![String::from("")];
    rightpad_in_place(&mut test);
}
