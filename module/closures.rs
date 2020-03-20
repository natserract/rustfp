

/** 
 * Closures
 * A closure is the combination of a function bundled together (enclosed) with references to its surrounding state (the lexical environment). In other words, a closure gives you access to an outer function’s scope from an inner function.

 Code: 
 fmt("Hello World", |j| j.to_string())
 expected output: "Hello World"

**/

pub fn closures() {
    fn fmt<A>(str1: &str, callback: A) -> String where A: Fn(String) -> String {
        let mut empty_str = String::new();
            {
                let str2: &str = "dolor sit amet";
                empty_str.push_str(str1);
                empty_str.push_str(str2);
            }
        callback(empty_str)
    }
   
    let r_txt = "Lorem ipsum ";
    assert_eq!("Lorem ipsum dolor sit amet", fmt(r_txt, |txt| { 
        txt.to_string()
    }))
}