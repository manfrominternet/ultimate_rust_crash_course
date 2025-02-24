fn main() {
    println!("Hello, world!");
    // let s1 = String::from("abc");
    // let s2 = s1;
    // println!("{}", s1); breaks! trying to use borrowed value
    // Remedy!
    //  let s1 = String::from("abc");
    //  let s2 = s1.clone();
    //  println!("{}", s1);

    // let s1 = String::from("abc");
    // do_stuff(s1);
    // println!("{}", s1); FAILS  1

    // let mut s1 = String::from("abc");
    // s1 = do_stuff(s1);
    // println!("{}", s1); WORKS 2

    // now the proper way
    let s1 = String::from("abc");
    do_stuff(&s1); 
}
// fn do_stuff(s: String) {
//     // do stuff
// } 1

// fn do_stuff(s: String) -> String {
//     s
// } 2

// now the proper way
fn do_stuff(s: &String) {

}