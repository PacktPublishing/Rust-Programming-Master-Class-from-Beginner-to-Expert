// -------------------------------------------
//	  Dereferencing
// -------------------------------------------
fn main() {
    let mut some_data = 42;
    let ref_1 = &mut some_data;
    let deref_copy = *ref_1;
    *ref_1 = 13;
    println!("some_data is: {some_data}, deref_copy is: {deref_copy}");

    let mut heap_data = vec![5, 6, 7];
    let ref_1 = &heap_data;
    let ref_2 = ref_1;
    let ref_3 = ref_1;
    let deref_copy = ref_1.clone();

    let move_out = ref_1;
    // let move_out_again = ref_1;
}
