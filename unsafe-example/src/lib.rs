use std::slice;

pub unsafe fn dangerous() {}

///////////////////////////////////////////////////////////////////////////////////

pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // (&mut values[..mid], &mut values[mid..])
    // BUG: cannot borrow `*values` as mutable more than once at a time

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/////////////////////////////////////

extern "C" {
    pub fn abs(input: i32) -> i32;
}

////////////////////////////////////////////////////

pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!")
}

//////////////////////////////////

pub static mut COUNTER: u32 = 0;

pub fn add_to_count(inc: u32) {
    unsafe { COUNTER += inc }
}

/////////////////////////////////////////

#[allow(unused)]
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
