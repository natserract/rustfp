
/** 
 * Lazy Evaluation
 * Lazy evaluation or non-strict evaluation is the process of delaying evaluation 
 * of an expression until it is needed

 Code: 
 x: 4
 
 val.add_or_multiply(true)
 expected output: 
    executing add
    8

 val.add_or_multiply(false)
 expected output: 
    executing multiply
    16
 
**/


struct State {
    x: i32,
}

trait Lazy {
    fn add(&self) -> i32;
    fn multiply(&self) -> i32;
    fn add_or_multiply(&self, add: bool) -> i32;
}

impl Lazy for State {
    fn add(&self) -> i32 {
        println!("executing add");
        &self.x + &self.x
    }

    fn multiply(&self) -> i32 {
        println!("executing multiply");
        &self.x * &self.x
    }

    fn add_or_multiply(&self, add: bool) -> i32 { 
        match add {
            true => self.add(),
            false =>  self.multiply(),
        }
    }
}

pub fn lazy_evaluation() {
    let val: State = State {
        x: 20
    };

    assert_eq!(40, val.add_or_multiply(true));
    assert_eq!(400, val.add_or_multiply(false));

}