
mod closures;
mod currying;
mod recursion;
mod higher_order_function;
mod lazyevaluation;

fn main() {
    closures::closures();
    currying::currying();
    recursion::recursion();
    higher_order_function::hof();
    lazyevaluation::lazy_evaluation();
}
