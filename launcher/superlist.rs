#[link(name = "superlist")]
extern "C" {
    fn superlist_main();
}

fn main() {
    unsafe {
        superlist_main();
    }
}
