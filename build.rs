use std::{
    io::{self, Write},
    process,
};

fn main() {
    println!("cargo:rerun-if-changed=templates/index.html");
    println!("cargo:rerun-if-changed=assets/style.css");

    match process::Command::new("sh")
        .arg("-c")
        .arg("npx @tailwindcss/cli -i assets/style.css -o assets/output.css")
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                let _ = io::stdout().write_all(&output.stdout);
                let _ = io::stdout().write_all(&output.stderr);
                panic!("Tailwind error");
            }
        }
        Err(e) => panic!("Tailwind error: {:?}", e),
    };
}