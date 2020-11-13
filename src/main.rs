
extern crate libc;

extern {
    fn func_in_cpp(input: libc::c_int) -> libc::c_int;
}

#[no_mangle]
pub extern fn func_in_rust(input: i32) -> i32 {
    println!("called by C++. In Rust now...");
    input * 2
}

fn main() {
    let input = 4;
    let output = unsafe { func_in_cpp(input) };
    println!("{} * 3 = {}", input, output);
}