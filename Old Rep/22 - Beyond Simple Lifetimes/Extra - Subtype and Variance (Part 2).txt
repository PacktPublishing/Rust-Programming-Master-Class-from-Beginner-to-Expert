// Subtype: Subtyping in Rust refers to the relationship where one type can be used in place of another without causing type errors.
// if T: U -> U can be used
// e.g.
// In Rust, subtyping primarily applies to lifetimes. If 'b is a subtype of 'a, then 'b can be used wherever 'a is expected. Another way of wording this is to say that b outlives a.
// but it goes beyond that Example

// &Dog :< &dyn Animal
// trait Animal {
//     fn make_sound(&self);
// }

// struct Dog;

// impl Animal for Dog {
//     fn make_sound(&self) {
//         println!("Woof!");
//     }
// }

// fn main() {
//     let dog = Dog;
//     let animal: &dyn Animal = &dog; // `&Dog` is a subtype of `&dyn Animal`

//     animal.make_sound(); // This will print "Woof!"
// }

// fn main() {
//     let some_str = String::from("Nouman");
//     let mut non_static_str = &*some_str;
//     let static_str: &'static str = "Hello world";
//     non_static_str = static_str;
// }

// fn shortner_lifetime<'a>(s: &'static str) -> &'a str {
//     s
// }

// covaraince
// You can provide any type which is the subtype of the provided argument.
// 'static :< 'a
// fn covaraince_fn(s: &str) {} // the argument type is basically covaraint

// fn main() {
//     let str_static: &'static str = "Hello";
//     {
//         let str_not_static = "World";
//         covaraince_fn(str_not_static);
//     }
//     covaraince_fn(str_static);

//     // The same can also happen in case of variables. for instance
//     let some_str = String::from("Nouman");
//     let mut non_static_str = &*some_str;
//     let static_str: &'static str = "Hello world";
//     non_static_str = static_str;
// }

// contravaraint
// fn some_fn(bar: fn(&'a str) -> ())
// let x: fn(&'a str) -> ();

// foo(fn(&static str) {})

// example of covraint
// struct MyCoType<Mixed> {
//     k1: fn() -> Mixed, // covariant over Mixed
// }



// fn main() {}
// 'static :< 'a
// covariance
// &'static T <: &'a T

// covariant
// fn(&'a T) <: fn(&'static T)

// invariance

fn assign<T>(input: &mut T, val: T) {
    *input = val;
}

fn main() {
    let mut hello: &'static str = "hello";

    {
        let world = String::from("world");
        assign(&mut hello, &world);
    }
    //println!("{hello}"); // use after free 😿
}
