fn main() {    let mut v = vec![1, 2, 3];    let ptr = v.as_mut_ptr();    unsafe {        // Correct way of dropping the vector.     std::mem::drop(v);        //  This will cause memory leak.        *ptr = 10; // Use after free        println!("{:?}", *ptr);    }    // Correct approach: Use a raw pointer only if absolutely necessary, and always ensure proper memory management.    let mut x = vec![1, 2, 3];    let mut y = x.clone();    let z = unsafe {std::ptr::addr_of_mut!(y[0])};    *z = 10;    println!("Value after modification : {}", y[0])}