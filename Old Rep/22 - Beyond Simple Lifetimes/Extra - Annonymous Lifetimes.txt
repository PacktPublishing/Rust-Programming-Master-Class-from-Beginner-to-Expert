
cover it from the https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1 

Example 1: 
So anonymous lifetimes are places where you tell the compiler,
guess what lifetime. And that only works when there's only one possible guess. 

impl foo {
fn get_ref(&self) -> &str{} // here if we put 
fn get_ref(&self) -> &'_ str{}  // here's only one other lifetime here, and that's the lifetime to self. And so the compiler can guess what this type is, as you don't need to give this. 
// you dont need to give this 

fn get_ref<'a>(&'a self) -> &'a str{} // The compiler understands with underscore that it must be the lifetime of self. So this is one example where the compiler will guess the lifetime. 

key difference between 
'a and '_ 

But it's something that you can use when you don't want to specify the lifetime and you think the compiler can figure it out.

Tick underscore is telling the compiler you guessed the lifetime. It sort of means it's sort of the same as underscore for types.
But it's something that you can use when you don't want to specify the lifetime and you think the compiler can figure it out.
}


