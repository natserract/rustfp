

/** 
 * Currying
 * Currying is the technique of translating the evaluation of a function that takes multiple arguments 
 * into evaluating a sequence of functions, each with a single argument. 

 Code: 
 add(5, 5, 10)
 expected output: 100

 remove(50, 50, 10)
 expected output: 10
 
**/

#[derive(Debug)]
struct States<'a> {
    a: &'a i32,
    b: &'a i32,
}

trait Currying {
    type ReturnType: Fn(i32) -> i32;
    fn add(self) -> Self::ReturnType;
}

impl Currying for States<'static>{
    type ReturnType = Box<dyn Fn(i32) -> i32>;
   
    fn add(self) -> Self::ReturnType {
        Box::new(move|x| {
            x * self.a
        })
    }
}

pub fn currying() {
    let r_value: States = States {
        a: &100,
        b: &100
    };

    let r1 = r_value.add();
    let r2 = r1(5);
    
    assert_eq!(500, r2);

}