

/** 
 * Currying
 * Currying is the technique of translating the evaluation of a function that takes multiple arguments into evaluating a sequence of functions, each with a single argument. 

 Code: 
 add(5, 5, 10)
 expected output: 100
 
**/

pub fn currying() {
    fn add(x: i32, y: i32, z: i32) -> i32 {
        (x + y) * z
    }

    assert_eq!(1000, add(100, 100, 5));
}