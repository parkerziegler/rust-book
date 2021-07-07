use core::slice;

// Unsafe operations in Rust occur in an unsafe block.
// You can perform five types of operations in unsafe Rust:
//
// 1. Dereference a raw pointer
// 2. Call an unsafe function or method
// 3. Access or modify a mutable static variable
// 4. Implement an unsafe trait
// 5. Access fields of union S
//
// Note that this does not turn off Rust's other safety checks
// or the borrow checker; it just allows these specific operations.
fn main() {
    let mut num = 5;

    // Here we create _raw pointers_ to the num binding. Raw pointers
    // are allowed to have mutable and immutable pointers, or multiple
    // mutable pointers to the same location. They aren't guaranteed to
    // point to valid memory, are allowed to be null, and don't implement
    // any automatic cleanup.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // You could even use raw pointers to access an arbitrary memory register.
    let address = 0x012345usize;
    let _r = address as *const i32;

    // We can create raw pointers in safe Rust, but we can't dereference them.
    // To do that, we need an unsafe block.
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (fst, snd) = split_at_mut(&mut v, 2);
    println!("fst is: {:?} and snd is: {:?}", fst, snd);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    add_to_count(3);
    add_to_count(4);

    unsafe {
        // Access the mutable static variable COUNTER. All code that reads from or
        // writes to COUNTER must be in an unsafe block.
        println!("COUNTER: {}", COUNTER);
    }
}

// To indicare that a function performs unsafe logic, preface it with the unsaf keyword.
unsafe fn dangerous() {
    let x = 10;
    let y = &x as *const i32;

    // We don't need an additional unsafe block to dereference a raw pointer in an unsafe function.
    println!("y is: {}", *y);
}

// split_at_mut is an example of creating a safe abstraction over unsafe code.
// Safe Rust code can call this function, and all unsafety is encapsulated in it.
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    // Access the mutable raw pointer of the slice in memory.
    let ptr = slice.as_mut_ptr();

    assert!(mid < len);

    unsafe {
        (
            // Create a slice from a raw pointer and the length of the splitting index.
            slice::from_raw_parts_mut(ptr, mid),
            // For the second slice, start add the mid point and take len - mid items.
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Any FFI is considered unsafe, i.e. interoperating with C code.
// The "C" here defines which application binary interface (ABI) we want to call
extern "C" {
    fn abs(input: i32) -> i32;
}

// We can also make Rust functions accessible from C.
// The #[no_mangle] attribute prevents the Rust compiler from mangling
// the function name so the calling C code can more easily identify it.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Calling some Rust code from C!");
}

// Static variables have a fixed address in memory.
// Static variables can also be immutable, unlike constants declared with the const keyword.
// However, accessing and modifying mutable static variables is unsafe.
static _HELLO_WORLD: &str = "Hello World";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Unsafe traits are those with at least one method with an invariant that cannot
// be verified by Rust. An example of where this can be useful is implementing
// Sync or Send for a type that the compiler cannot automatically implement these
// traits for, such as raw pointers.
unsafe trait Foo {}

unsafe impl Foo for i32 {}
