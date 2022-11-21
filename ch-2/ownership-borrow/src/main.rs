fn main() {
    let s = String::from("test");
    heap_example(&s);
}

fn heap_example(input: &String) {
    let mystr = input;
    let _otherstr = mystr;
    println!("{}", mystr);
}
