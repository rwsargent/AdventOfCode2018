use std::env;
//use std::alloc::String;

fn main() {
    let input = env::args().skip(1).next().unwrap();
    println!("capthca next: {}", sum_digits_next(&input).unwrap());
    println!("capthca halfway: {}", sum_digits_halfway(&input).unwrap());
}

fn sum_digits_next(input: &String) -> Result<u32, &'static str> {
    let len = input.len();
    let bytes = input.as_bytes();
    if len > 0 {
        fn digit_to_num(digit: u8) -> Result<u32, &'static str> {
            if digit >= 0x30 && digit <= 0x39 {
                Ok((digit - 0x30) as u32)
            } else {
                Err("input has non numeric digit")
            }
        }
        let mut x: u32 = 0;
        for i in 0..len {
            let this = digit_to_num(bytes[i])?;
            let next = digit_to_num(bytes[(i + 1) % len])?;
            if this == next {
                x += this
            }
        }
        Ok(x)
    } else {
        Ok(0)
    }
}

#[test]
fn test_sum_digits_next() {
    assert_eq!(sum_digits_next("1122".to_string()).unwrap(), 3);
    assert_eq!(sum_digits_next("1111".to_string()).unwrap(), 4);
    assert_eq!(sum_digits_next("1234".to_string()).unwrap(), 0);
    assert_eq!(sum_digits_next("91212129".to_string()).unwrap(), 9);
}


fn sum_digits_halfway(input: &String) -> Result<u32, &'static str> {
    let len = input.len();
    let half_len = len / 2;
    let bytes = input.as_bytes();
    if len > 0 {
        fn digit_to_num(digit: u8) -> Result<u32, &'static str> {
            if digit >= 0x30 && digit <= 0x39 {
                Ok((digit - 0x30) as u32)
            } else {
                Err("input has non numeric digit")
            }
        }
        let mut x: u32 = 0;
        for i in 0..len {
            let this = digit_to_num(bytes[i])?;
            let next = digit_to_num(bytes[(i + half_len) % len])?;
            if this == next {
                x += this
            }
        }
        Ok(x)
    } else {
        Ok(0)
    }
}

#[test]
fn test_sum_digits_halfway() {
    assert_eq!(sum_digits_halfway("1212".to_string()).unwrap(), 6);
    assert_eq!(sum_digits_halfway("1221".to_string()).unwrap(), 0);
    assert_eq!(sum_digits_halfway("123425".to_string()).unwrap(), 4);
    assert_eq!(sum_digits_halfway("123123".to_string()).unwrap(), 12);
    assert_eq!(sum_digits_halfway("12131415".to_string()).unwrap(), 4);
}
