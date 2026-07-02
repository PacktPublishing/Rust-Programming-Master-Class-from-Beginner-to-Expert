/*
// Start from this very basic lifetime example
fn some_fn(s: &str) {}
fn main() {
    let s_1 = String::from("");
    let s_2 = "";
    some_fn(s_2);
    some_fn(&*s_1);
}

*/ 


pub fn strtok<'a>(s: &'a mut &'a str, delimiter: char) {
    //-> &'a str {
    //-> &'static str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + 1)..];
        *s = suffix;
        //suffix   // comment later on for problem 3 and then uncomment for problem 4
        //""   // add for problem 3
    } else {
        let prefix = *s;
        *s = "";
        //prefix //  comment later on for problem 3 and then uncomment for problem 4
        // "" // add for problem 3
    }
}
fn main() {
    // Prblem 1:
    // let mut x = "hello world";
    // let hello = strtok(&mut x, ' ');
    // assert_eq!(x, "world");

    /*  Hello is going to have the same lifetime as the mutable reference that's given in. And therefore, as long as hello lives, x continues to be mutably borrowed.
       So here, hello still exists, and therefore the compiler gets sad. Because we're trying to use x while it's still mutably borrowed in hello.
    */

    // Problem 2:
    /* let us drop hello to see if it solves out the problem 16:00*/
    // {
    //     let hello = strtok(&mut x, ' ');
    // }
    // assert_eq!(x, "world");

    // Problem 3: it is also something which is not related ot the output lifetime. let us change the outgput to static
    // Problem 4: it is also not related to the output at all.

    // problem 5:
    // let us simplify the code a bit for explaination
    let mut x = "hello world";
    strtok(&mut x, ' ');
    // assert_eq!(x, "world");

    // function signature
    // &'a  mut          &'a str        // expected
    // &    mut          &'static str   // provided
    // &'static  mut     &'static str        // Compiler inference

    // Problem 6:  at 20:00
    {
        let mut x = "hello world";
        strtok(&mut x, ' ');
        // assert_eq!(x, "world");

        // why this work: becuase the compiler knows that it can shorten the lifetime of x to any other shorter lifetime.
    }

    // &'x  mut     &'x str
    // becuase of variance: 'static :< 'x  // anywhere i am expecting a longer lifetime, it is ok if you provide me with a shorter lifetime.

    // fn(&'a T) you can provide fn(&'static T) -> this is becuase of variance
    // why this work: becuase the compiler knows that it can shorten the lifetime of x to any other shorter lifetime.

    let s = String::from("Nouman");
    let x: &'static str = "Hello world";
    let mut y = &*s;
    y = x;
}

// Problem 7
fn shortner_lifetime<'a>(s: &'static str) -> &'a str {
    //fn shortner_lifetime<'a>(s: &'a str) -> &'static str {
    s
}

// This means that we have some sort of relationship between lifetimes. This is captured through the idea of the outlives relationship. If 'b outlives 'a, it is written as 'b: 'a

// covaraince:
