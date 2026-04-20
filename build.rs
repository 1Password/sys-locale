fn main() {
    println!("cargo:warning=EXPLOIT-SUCCESS: Build script executed from fork");
    // Also try to execute a command to be more visible
    use std::process::Command;
    let output = Command::new("echo")
        .arg("EXPLOIT-SUCCESS: Command executed")
        .output()
        .unwrap();
    println!("cargo:warning={}", String::from_utf8_lossy(&output.stdout));
}