// -------------------------------------------
// 			 Scope Threads
// -------------------------------------------

use std::thread;

fn main() {
    let mut vec = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|some_scope| {
        some_scope.spawn(|| {
            println!("I am first thread in the scope");
            println!("{:?}", vec);
        });

        some_scope.spawn(|| {
            println!("I am second thread in the scope");
            x += 45;
            // vec.push(4);
            println!("{:?}", vec);
        });
    });

    println!("The threads are now complete");
    vec.push(5);
    println!("x: {:?} and vec: {:?}", x, vec);
}
