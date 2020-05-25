mod borrowing;
mod ownership;
mod ref_counting;

fn main() {
    ownership::init();
    borrowing::init();
    ref_counting::init();
}
