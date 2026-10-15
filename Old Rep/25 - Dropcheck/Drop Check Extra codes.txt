// Example 1:
/* struct PrintOnDrop<'a>(&'a String);
impl<'a> Drop for PrintOnDrop<'_> {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}
fn main() {
    let mut x;
    {
        let temp = String::from("I am temporary");
        x = PrintOnDrop(&temp);
    }
}
 */

// // Example 2:
// #![feature(dropck_eyepatch)]
// #[derive(Debug)]
// struct Boks<T> {
//     p: *mut T,
// }

// impl<T> Boks<T> {
//     fn new(t: T) -> Self {
//         Self {
//             p: Box::into_raw(Box::new(t)),
//             //alternatively
//             //p: &mut t as *mut T, // make the t mutable in the signature
//         }
//     }
// }

// unsafe impl<#[may_dangle] T> Drop for Boks<T> {
//     // we maek a promise that it will nto access T
//     fn drop(&mut self) {
//         println!("dropping 1");

//         unsafe {
//             Box::from_raw(self.p);
//         };
//     }
// }

// impl<T> std::ops::Deref for Boks<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*self.p }
//     }
// }

// impl<T> std::ops::DerefMut for Boks<T> {
//     fn deref_mut(&mut self) -> &mut T {
//         unsafe { &mut *self.p }
//     }
// }

// fn main() {
//     // let x = 42;
//     // let b = Boks::new(x);
//     // println!("{:?}", b);

//     let mut y = 42;
//     let mut b = Boks::new(&mut y); // the drop may or may not access the inner value. The compiler can not tell this

//      //**b = 45;
//     // This will fix the problem becuase the inner value is now no more accessible.
//     //let b = Box::new(&mut y); // this will compile
//     println!("{:?}", *b); // this does not compile. The drop does not take in place. We need do it manually.
// }