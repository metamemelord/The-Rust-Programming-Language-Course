#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

mod closures;
mod functions;
mod higher_order_functions;
mod methods;

fn main() {
    functions::init();
    methods::init();
    closures::init();
    higher_order_functions::init();
}
