use std::fs;
use rustop::opts;


fn main() -> std::io::Result<()> {
    let (args, _rest) = opts! {
        synopsis "Rot13 encryption/decryption.";
        param filename:String, desc:"Filename to encrypt/decrypt.";
    }.parse_or_exit();

    let contents = fs::read_to_string(args.filename.clone())
        .expect("Something went wrong reading the file");

    let mut result = String::with_capacity(contents.len());
    for c in contents.chars() {
        if c.is_alphabetic() {
            if (c as u32 >= 78 && c as u32 <= 90) || (c as u32 >= 110 && c as u32 <= 122) {
                result.push((c as u8 - 13) as char);
            } else if (c as u32 >= 65 && c as u32 <= 77) || (c as u32 >= 97 && c as u32 <= 109) {
                result.push((c as u8 + 13) as char);
            }
        } else {
            result.push(c);
        }
    }

    let mut filename_out = args.filename;
    filename_out.push_str(".rot13");
    fs::write(filename_out, result)
        .expect("Error writing the file.");

    Ok(())
}
