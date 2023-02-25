mod component;

use component::{Component, Dir, File};

fn main() {
    let file1 = File::new("file 1");
    let file2 = File::new("path_2");
    let file3 = File::new("file 3");

    let mut folder1 = Dir::new("dir 1");
    folder1.add(file1);

    let mut folder2 = Dir::new("dir 2");
    folder2.add(file2);
    folder2.add(file3);
    folder2.add(folder1);

    folder2.search("path");
}
