use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mods_path = Path::new(&out_dir).join("mods.rs");
    let array_path = Path::new(&out_dir).join("array.rs");

    let files = fs::read_dir("./src/solution/").unwrap();
    let mut numbers: Vec<_> = files
        .filter_map(|e| {
            e.unwrap()
                .file_name()
                .to_str()
                .and_then(|s| s.strip_suffix(".rs"))
                .and_then(|s| s.strip_prefix('p'))
                .map(|s| s.to_owned())
        })
        .collect();
    numbers.sort();

    let mut mods = File::create(mods_path).unwrap();
    let mut array = File::create(array_path).unwrap();
    let max = numbers.last().unwrap();
    let max = if let Some(stripped1) = max.strip_prefix('0') {
        if let Some(stripped2) = stripped1.strip_prefix('0') {
            stripped2
        } else {
            stripped1
        }
    } else {
        max
    };

    array
        .write_all(b"pub const SOLUTIONS: [Option<*const (dyn Fn() -> String + Send + 'static)>; ")
        .unwrap();
    array.write_all(max.as_bytes()).unwrap();
    array.write_all(b"] = [").unwrap();

    let mut last = 0;
    for number in numbers {
        mods.write_all(b"mod p").unwrap();
        mods.write_all(number.as_bytes()).unwrap();
        mods.write_all(b";").unwrap();

        let parsed: usize = number.parse().unwrap();
        for _ in (last + 1)..parsed {
            array.write_all(b"None,").unwrap();
        }
        last = parsed;

        array.write_all(b"Some(&p").unwrap();
        array.write_all(number.as_bytes()).unwrap();
        array.write_all(b"::solution as *const _),").unwrap();
    }

    mods.flush().unwrap();
    array.write_all(b"];").unwrap();
    array.flush().unwrap();
}
