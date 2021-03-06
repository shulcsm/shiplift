extern crate shiplift;

use shiplift::Docker;
use std::env;
use std::io::prelude::*;
use std::io::copy;
use std::fs::OpenOptions;

fn main() {
    let docker = Docker::new();
    if let Some(id) = env::args().nth(1) {
        let mut export = OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{}.tgz", &id))
            .unwrap();
        let images = docker.images();
        let mut exported = images.get(&id).export().unwrap();
        println!("copying");
        copy(&mut exported, &mut export).unwrap();
        println!("copied");
    }
}
