
/** 
 * Higher-Order Functions
 * Higher-order functions are functions that take other functions as arguments or return functions as their results.
 * Taking an other function as an argument is often referred as a callback function, because it is called back by the higher-order function.

 Code: 
 map(&vec![1, 2, 3], |j| j + 10)
 expected output: [10, 20, 30]
 
**/

pub fn hof() {
    fn map<F>(arr: &[i32], func: F) -> Vec<i32> where F: Fn(&i32) -> i32{
        let mut new_array: Vec<i32> = vec![];
        for i in arr.iter() {
            new_array.push(func(i))
        }
        return new_array
    }

    let list = vec![1, 4, 9, 16];
    let result = map(&list, |i| *i + 2);
    
    assert_eq!(vec![3, 6, 11, 18], result)
}

