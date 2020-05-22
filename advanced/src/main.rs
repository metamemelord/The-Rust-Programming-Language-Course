#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

mod closures;
mod dynamic_dispatch;
mod functions;
mod higher_order_functions;
mod methods;
mod static_dispatch;
mod traits;
mod generic_vectors;

fn main() {
    functions::init();
    methods::init();
    closures::init();
    higher_order_functions::init();
    traits::init();
    static_dispatch::init();
    dynamic_dispatch::init();
    generic_vectors::init();
}
