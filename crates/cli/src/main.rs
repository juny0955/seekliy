use std::{env, error::Error, path::Path};
use core::scanner::Scanner;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| ".".to_string());

    let scanner = Scanner;
    let files = scanner.scan(Path::new(&path))?;

    println!("검색된 파일 수: {}", files.len());

    for file in files.iter().take(20) {
        println!("{}", file.path.display());
    }

    Ok(())
}
