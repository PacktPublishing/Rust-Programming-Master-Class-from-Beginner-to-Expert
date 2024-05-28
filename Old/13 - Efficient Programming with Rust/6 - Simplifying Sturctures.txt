// -------------------------------------------
//           	- Simplifying structures
// ------------------------------------------- 


struct A {
    f1: u32,
    f2: u32,
    f3: u32,
}

fn fn1(a: &mut A) -> &u32 { &a.f2 }
fn fn2(a: &mut A) -> u32 { a.f1 + a.f3 }

fn fn3(a: &mut A) {
    let x = fn1(a);
    let y = fn2(a); 
    println!("{}", x);
}

struct A {
    b: B,
    c: C,
}
struct B {
    f2: u32,
}
struct C {
    f1: u32,
    f3: u32,
}

// These functions take a B or C, rather than A.
fn fn1(b: &mut B) -> &u32 { &b.f2 }
fn fn2(c: &mut C) -> u32 { c.f1 + c.f3 }

fn fn3(a: &mut A) {
    let x = fn1(&mut a.b);
    let y = fn2(&mut a.c);
    println!("{}", x);
}

fn main() {}



