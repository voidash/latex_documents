#! /usr/bin/env run-cargo-script

use std::fs;
use std::io::prelude::*;

fn create_new_index(folders: Vec<String>) {
    let path = "./index.html";

    let mut output =  fs::OpenOptions::new().write(true).append(false).truncate(true).open(path).unwrap(); 

    let mut string_to_write = folders.iter().fold(String::from("<ul>"),|a,i| {
        format!("{0}\n<li><a href=\"{1}\">{1}</a></li>",a.as_str(),i.as_str())
    });
    string_to_write = format!("{}</ul>",string_to_write);


    if let Err(e) = writeln!(output,"{}",string_to_write.as_str()) {
        eprintln!("couldn't write to file");
    }

    }



pub fn main() {

    let paths = fs::read_dir("./").unwrap();
    let mut folders_to_include = vec![];
    find_paths(paths,&mut folders_to_include);
    println!("{:?}",folders_to_include);
    create_new_index(folders_to_include);
}

pub fn find_paths(paths: std::fs::ReadDir,folders_to_include:&mut Vec<String>) {
    for path in paths {
        let rpath = path.unwrap();
        println!("Name: {}",rpath.path().display());
        if rpath.metadata().unwrap().is_dir() {
            if !rpath.path().to_str().unwrap().starts_with("./.") {
                let new_path = fs::read_dir(rpath.path()).unwrap();
                find_paths(new_path,folders_to_include);
            }
        }
            let test_path = String::from(rpath.path().to_str().unwrap());
            if test_path.ends_with(".pdf"){
                println!("found {}", test_path);
                folders_to_include.push(test_path);
        }
    }
}
