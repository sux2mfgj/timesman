extern "C" {
    fn info(num: i32);
}

#[no_mangle]
extern "C" fn init() {
    // unsafe { info(123); }
}
