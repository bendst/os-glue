fn update_readme() {
    use std::process::Command;
    use std::fs::File;
    use std::io::Write;

    let output = Command::new("cargo").arg("readme").output().unwrap();
    
    let readme = String::from_utf8_lossy(&output.stdout);

    File::create("README.md").as_mut().map(|file| {
        file.write(readme.as_bytes()).unwrap();
    }).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    update_readme();
}
