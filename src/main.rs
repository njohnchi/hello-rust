fn main() {
    let closure = return_closure();
    call_twice(closure)
}

fn return_closure() -> impl Fn() {
    let s = String::from("a closure");
    move || println!("{s}")
}

fn call_twice(f: impl Fn()) {
    f();
    f();
}
