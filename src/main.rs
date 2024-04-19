use std::{
    fs::File,
    io::{self, Read, Write},
};

use cairo_vm::vm::runners::cairo_pie::CairoPie;

fn deserialize_pie() -> std::io::Result<Vec<u8>> {
    let mut file = File::open("avg_balance.pie")?;
    println!("File opened:{:?}", file);
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn serialize_pie(pie_bytes: Vec<u8>) -> std::io::Result<()> {
    // Write to a .pie file
    let mut file = File::create("output.pie")?;
    file.write_all(&pie_bytes)?;
    Ok(())
}

fn files_are_identical() -> io::Result<bool> {
    let mut f1 = File::open("avg_balance.pie")?;
    let mut f2 = File::open("output.pie")?;

    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();

    f1.read_to_end(&mut buf1)?;
    f2.read_to_end(&mut buf2)?;

    Ok(buf1 == buf2)
}

fn main() -> io::Result<()> {
    let bytes = deserialize_pie()?;
    serialize_pie(bytes)?;

    match files_are_identical() {
        Ok(true) => println!("Files are identical."),
        Ok(false) => println!("Files are not identical."),
        Err(e) => println!("Error occurred: {}", e),
    }

    Ok(())
}
