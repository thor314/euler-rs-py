#[timings_proc_macro::timings]
fn time() {}

#[test]
fn works() {
    time();
    //assert!(false);
}
