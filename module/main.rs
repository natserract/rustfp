
#![feature(unboxed_closures)]
#![feature(type_alias_impl_trait)]

mod closures;
mod currying;
mod recursion;
mod higher_order_function;
mod lazy_evaluation;

fn main() {
    closures::closures();
    currying::currying();
    recursion::recursion();
    higher_order_function::hof();
    lazy_evaluation::lazy_evaluation();
}
