// use core::slice;
use unsafe_example::{abs, add_to_count, call_from_c, dangerous, split_at_mut, COUNTER};

fn main() {
    let mut num = -5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("\nr1 is {}", *r1);
        println!("r2 is {}", *r2);

        dangerous()
    }

    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
    println!(
        "\nSplitting Vector:\nLeft slice -> {:?}\nRight slice -> {:?}",
        left, right
    );

    { // FIX: Very Unsafe
         // let address = 0x01234usize;
         // let r = address as *mut i32;
         // let values = unsafe { slice::from_raw_parts_mut(r, 10000) };
         // println!("{:?}\n", values);
    }

    unsafe { println!("\nAbsolute value of {num} according to C: {}\n", abs(num)) }
    call_from_c();

    add_to_count(3);
    unsafe { println!("\nCounter: {:?}", COUNTER) }

    println!()
}
