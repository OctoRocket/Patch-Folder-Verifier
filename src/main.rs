use std::{env, fs};

fn main() {
    let mut arg: Vec<String> = env::args().collect();
    if arg.len() != 2 {
        println!("There are is not one arguement input, please use one argument.");
        return;
    }
    arg[1] = fs::canonicalize(arg[1].clone()).unwrap().into_os_string().into_string().unwrap();
    let mut patchs = Vec::new();
    for f in fs::read_dir(arg[1].clone()).unwrap() {
        if f.as_ref().unwrap().file_name().into_string().unwrap().ends_with(".patch") {
            patchs.push(f.unwrap().file_name().into_string().unwrap());
        }
    }
    if patchs.len() == 0 {
        println!("There are no patch files!");
        return;
    }
    patchs.sort();
    let mut used = Vec::new();
    for i in patchs {
        used.push(i[..4].parse::<i32>().unwrap());
    }
    for i in 1..used.len() {
        if used[i-1] + 1 != used[i] && used[i-1] + 2 == used[i] {
            println!("Patch {} is missing!", used[i] - 1);
        } else if used[i-1] +1 != used[i] && used[i-1] + 2 == used[i] - 1 {
            println!("Patches {} and {} are missing!", used[i-1] + 1, used[i] - 1)
        } else if used[i-1] + 1 != used[i] {
            println!("Patches {} through {} are missing!", used[i-1] + 1, used[i] - 1)
        }
    }
}
