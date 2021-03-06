

/** 
 * Recursion
 * Recursion is a technique that allows us to break down a problem into smaller pieces. 
 * This technique allows us to remove some local side effects that we perform while writing looping structures 
 * and also makes our code more expressive and readable.

 Code: 
 Factor::factorial(3)
 expected output: 6
 
**/


#[allow(non_camel_case_types)] 
type i64_t = i64;

trait Factor {
    fn factorial_tail_rec(val: i64_t) -> Self;
    fn factorial(num: i64_t) -> Self;
}

impl Factor for i64_t {
    fn factorial_tail_rec(val: i64_t) -> Self {
        val
    }

    fn factorial(num: i64_t) -> Self {
        match num {
            0 => 1,
            _ => num * Self::factorial_tail_rec(num - 1)
        }
    }
}

pub fn recursion() {
    let result: i64_t = Factor::factorial(3); 
    assert_eq!(6, result);
}