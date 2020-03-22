# Functional Programming in Rust

A month ago when I first developed a non-commercial project using [ReasonML](https://reasonml.github.io/), I got a new experience, is a *functional programming*. This language is an alternative syntax from [OCaml](https://ocaml.org/). OCaml itself is a purely functional language, the features offered are interesting. For example: type inference, strongly type system, algebraic datatypes, and many more. Interesting right?

Now after trying that, I began to interest functional programming. Finally i tried to implement the functional paradigm in a different language, namely [Rust](http://rustlang.org/).

### Introduction
Functional programming (FP) is a programming paradigm which allows us to write expressive, concise, and elegant code. Functional programming also helps developers to manage code so that it doesn't **DRY (Don't Repeat Yourself)** that's mean doesn't write the same code over and over again. Other functional languages for example like Lisp, Clojure, Erlang, Haskell, R, and many more.

### Okay, but why Rust?
The question is, is a **Rust functional language?** the answer is, no. Although Rust himself was inspired by **ML family of language**, Rust it's not functional. But fortunately Rust has several features that are similar to other functional languages, such as: algebraic datatypes, expressive types, and others.

### Tables of Contents
* [Primitive Types](#primitive)
* [Closures](#closures)
* [Currying](#currying)
* [Recursion](#recursion)
* [Higher Order Functions(HOF)](#hof)
* [Lazy evaluations](#lazy)


## <a id="primitive"></a>Primitive Types
In order not to jump right away, it would be nice if we had to know several data types in Rust. This also applies to all programming languages.

### Booleans
The most basic datatype is the simple `true / false` value, in Rust it's called `bool`

```rust
let x = true;
let y: bool = false;
```

### Char
The `char` data type has a single Unicode value. We can use `char` data type with a single tick (`'`)
```rust
let x = 'x';
let two_hearts = '💕';
```
Unlike some other languages, `char` in Rust is not one byte, but four.

### Numeric Types
Rust has several numeric type category variants, such as signed(`i`) dan unsigned(`u`), fixed size (`8`, `16`, `32`, `64`) dan variable(`isize`, `usize`) types.
```rust
let x = 42; // `x` has type `i32`.
let y = 1.0; // `y` has type `f64`.
```

### Arrays
Like many other programming languages, Rust also has an array data type. By default, arrays in Rust can't be changed. Unless you initialize it with [`mut`](https://doc.rust-lang.org/rust-by-example/scope/borrow/mut.html)
```rust
let a = [1, 2, 3]; // a: [i32; 3]
let mut m = [1, 2, 3]; // m: [i32; 3]
```

### Functions
Functions also have data types! For example like:
```rust
fn foo(x: i32) -> i32 { x }
let x: fn(i32) -> i32 = foo;
```
In this case, the `foo ()` function has a return type `numeric: i32`, and returns the value` x`.

> For more information, you can check here: [primitive types](https://doc.rust-lang.org/1.29.0/book/first-edition/primitive-types.html)

## <a id="closures"></a>Closures
>*"Closure is a mechanism by which an inner function will have access to the variables defined in its outer function’s lexical scope even after the outer function has returned."* 

Up to here understand? in short *closures* is an inner function that has access to retrieve a value throughout the scope both inside and outside.

```rust
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
```
In this case, in `new_str.push_str ()` section where `closure_annotated` accesses the` new_str` variable then changes the value and returns it outside the scope.

## <a id="currying"></a>Currying
Currying is a process in functional programming in which we can transform a function with multiple arguments into a sequence of nesting functions. It returns a new function that expects the next argument inline.

```rust
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

let r_value: States = States {
    a: &100,
    b: &100
};

let r1 = r_value.add();
let r2 = r1(5);

assert_eq!(500, r2);

```
There are 2 parameters here, namely `a`,`b` where each has a numeric data type, then in `trait` section is a *function interface*, a place for initializing functions. These traits are similar to [typescript interfaces](https://www.typescriptlang.org/docs/handbook/interfaces.html).


## <a id="recursion"></a>Recursion
Simply, recursion is a `procedure / function` that calls itself, which functions to create / process the data that you want to return.

```rust
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

let result: i64_t = Factor::factorial(3); 
assert_eq!(6, result);
```
This is a factorial function, where if the argument value of the parameter num `! == 0`, then that value will be multiplied by each number below it. For example `(5! = 5 * 4 * 3 * 2 * 1 = 120)`.

## <a id="hof"></a>Higher Order Functions(HOF)
Higher order functions are functions that use other functions as parameters or as a result of returns.
```rust
fn map<F>(arr: &[i32], func: F) -> Vec<i32> where F: Fn(&i32) -> i32{
    let mut new_array: Vec<i32> = vec![];
    for i in arr.iter() {
        new_array.push(func(i))
    }
    
    return new_array
}

let lists = vec![1, 4, 9, 16];
let result = map(&lists, |i| *i + 2);

assert_eq!(vec![3, 6, 11, 18], result)
```
So `func` and` map` are higher order functions, where this function is used to change every contents of an array. The return result is a new array of the same length as the modified `originalArray`.

## <a id="lazy"></a>Lazy Evaluation
Lazy evaluation or non-strict evaluation is a process of holding the evaluation of an `expression/function` until the value is needed. The goal is to avoid repeated evaluations.

```rust
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

let val: State = State {
    x: 20
};

assert_eq!(40, val.add_or_multiply(true));
assert_eq!(400, val.add_or_multiply(false));
```


## Functional Programming References
* [Functional Programming in Go](https://deepu.tech/functional-programming-in-go/)
* [Eloquent Javascript Functional Programming](https://eloquentjavascript.net/1st_edition/chapter6.html)
* [Closures Developer Mozilla](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Closures) 


## Motivation
Functional programming (FP) provides many advantages, and its popularity has been increasing as a result. However, each programming paradigm comes with its own unique jargon and FP is no exception. By providing a glossary, i hope to make learning FP easier✌️