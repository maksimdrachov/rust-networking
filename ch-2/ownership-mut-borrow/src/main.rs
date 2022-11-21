fn main() {
    let mut s = String::from("test");
    heap_example(&mut s);
}

fn heap_example(input: &mut String) {
    let mystr = input;
    let _otherstr = mystr;
    println!("{}", mystr);
}
