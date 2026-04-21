fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // Cannot borrow s as mutable more than once

    println!("{r1}, {r2}");
}
