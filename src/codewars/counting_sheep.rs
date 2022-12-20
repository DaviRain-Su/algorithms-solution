//! # Counting sheep
//!
//! ## DESCRIPTION:
//! Consider an array/list of sheep where some sheep may be missing from their place. We need a function that counts the number of sheep present in the array (true means present).
//!
//! For example,
//! ```none
//! &[true,  true,  true,  false,
//!  true,  true,  true,  true ,
//!  true,  false, true,  false,
//!  true,  false, false, true ,
//!  true,  true,  true,  true ,
//!  false, false, true,  true];
//! ```
//! The correct answer would be 17.
//! Hint: Don't forget to check for bad values like null/undefined
//!
//!
pub fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|&&x| x).count() as u8
}

// another way to reslove this question
// pub fn count_sheep1(sheep: &[bool]) -> u8 {
//     sheep.iter().fold(0, |acc, x| if *x == true {acc + 1} else { acc })
// }

#[test]
fn returns_correct_sheep_count() {
    assert_eq!(count_sheep(&[]), 0);
    assert_eq!(count_sheep(&[false]), 0);
    assert_eq!(count_sheep(&[true]), 1);
    assert_eq!(count_sheep(&[true, false]), 1);
    assert_eq!(count_sheep(&[true, true]), 2);
    assert_eq!(count_sheep(&[false, false]), 0);
}
