

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
    fn fmt(prev_str: &str) -> String {
        let mut new_str = String::new();
        
        let closure_annotated = |next_str| -> String {
            new_str.push_str(prev_str);
            new_str.push_str(next_str);
            return new_str;
        };
        
       closure_annotated("dolor sit amet")

    }
   
    let r_txt = "Lorem ipsum ";
    assert_eq!("Lorem ipsum dolor sit amet", fmt(r_txt));
}