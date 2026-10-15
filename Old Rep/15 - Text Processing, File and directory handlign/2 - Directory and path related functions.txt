// -------------------------------------------
// 			Directory and Path Related Functions
// ------------------------------------------- 
use std::env; 
use std::fs; 
use std::path::{Path, PathBuf};
fn main() {
    // let path = Path::new(r"D:\Rust\Examples\my_file.txt"); 
    // println!("Folder containing the file: {:?}", path.parent().unwrap()); 

    // println!("Name of hte file is {:?}", path.file_stem().unwrap()); 
    // println!("Extention of the file is {:?}", path.extension().unwrap());

    // let mut path = PathBuf::from(r"D:\"); 
    // path.push(r"Rust"); 
    // path.push(r"Examples"); 
    // path.push(r"my_file"); 

    // path.set_extension("txt"); 
    // println!("The path is {:?}", path);

    // let path = [r"D:\", r"Rust", r"Examples", r"my_file.txt"]
    // .iter()
    // .collect::<PathBuf>();
    // println!("The path is {:?}", path);

    // let path = Path::new(r"D:\Pics"); 
    // println!("Is the path directory {:?}", path.is_dir()); 

    // let path = Path::new(r"D:\my_text.txt");
    // println!("Does the file exsits: {:?}", path.is_file());

    // let data = path.metadata().unwrap(); 
    // println!("type {:?}", data.file_type()); 
    // println!("length {:?}", data.len()); 
    // println!("Permissions {:?}", data.permissions()); 
    // println!("Modified {:?}", data.modified()); 
    // println!("Created {:?}", data.created());


    // let path = Path::new(r"D:\"); 
    // for files in path.read_dir().expect("read_dir call failed") {
    //     println!("{:?}", files.unwrap().path());
    // }

//     let mut curr_path = env::current_dir().expect("can't access current directory"); 
//     println!("{:?}", curr_path);

//     println!("Create a new directory: {:?}", fs::create_dir(r"D:\rust1")); 
//     println!("Create a new directory and sub directories: {:?}", 
// fs::create_dir_all(r"D:\rust1\level1\level2"));

println!("Remove a specific directory: {:?}", fs::remove_dir(r"D:\rust1\level1\level2"));
println!("Remove a specific directory when it is not empty {:?}", fs::remove_dir(r"D:\rust1"));
println!("Remove everything from a directory {:?}", fs::remove_dir_all(r"D:\rust1"));
    
println!("Remving a file {:?}", fs::remove_file(r"D:\my_text.txt")); 

println!("Renaming a file {:?}", fs::rename(r"D\prev.txt", r"D:\new.txt")); 

println!("Copying contents from one file to another: {:?}", fs::copy(r"D:\new1.txt", r"D:\new2.txt"));
}