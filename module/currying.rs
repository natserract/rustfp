

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


struct State {
    x: i32,
    y: i32,
    z: i32
}

trait Currying {
    fn add(&self) -> i32;
    fn remove(&self) -> i32;
}

impl Currying for State {
    fn add(&self) -> i32 {
        (self.x + self.y) * self.z
    }
    fn remove(&self) -> i32 {
        (self.x + self.y) / self.z
    }
}

pub fn currying() {
    let result: State = State {
        x: 100,
        y: 100,
        z: 2
    };

    assert_eq!(400, result.add());
    assert_eq!(100, result.remove());
}