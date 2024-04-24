use std::{env::{args, var}, fs::read_to_string, process};

fn main() {
    let v1: Vec<String> = args().collect();

    if v1.len() < 3 {
        println!("not enough arguments");
        process::exit(1);
    }

    let ignore_case = var("IGNORE_CASE").is_ok();

    let content = read_to_string(&v1[1]).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1);
    });
    let mut v2 = vec![];

    if ignore_case{
        for line in content.lines(){
            if line.to_lowercase().contains(&v1[2].to_lowercase()) {
                v2.push(line);
            }
        }
    } else {    
        for line in content.lines(){
            if line.contains(&v1[2]) {
            v2.push(line);
            }
        }
    }

    if v2.len() == 0 {
        println!("no such result!")
    }

    for line in v2 {
        println!("{}", line)
    }
}
