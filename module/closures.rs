

/** 
 * Closures
 * A closure is the combination of a function bundled together (enclosed) with references to its surrounding state 
 * (the lexical environment). In other words, a closure gives you access to an outer function’s scope 
 * from an inner function.

 Code: 
 fmt("Hello World", |j| j.to_string())
 expected output: "Hello World"

**/

pub fn closures() {
    fn fmt<A>(prev_str: &str, func: A) -> String where A: Fn(String) -> String {
        let mut new_str = String::new();
            {
                let prev_str2: &str = "dolor sit amet";
                new_str.push_str(prev_str);
                new_str.push_str(prev_str2);
            }
        func(new_str)
    }
   
    let r_txt = "Lorem ipsum ";
    assert_eq!("Lorem ipsum dolor sit amet", fmt(r_txt, |txt| txt.to_string()))
}