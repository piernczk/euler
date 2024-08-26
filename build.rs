use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("solutions.rs");

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

    let out_file = File::create(out_path).unwrap();
    let mut out = BufWriter::new(out_file);

    out.write_all(b"pub const SOLUTIONS: [Option<&(dyn Fn() -> String + Send + 'static)>; ")
        .unwrap();
    out.write_all(max.as_bytes()).unwrap();
    out.write_all(b"] = [").unwrap();

    let mut last = 0;
    for number in numbers {
        let parsed: usize = number.parse().unwrap();
        for _ in (last + 1)..parsed {
            out.write_all(b"None,").unwrap();
        }
        last = parsed;

        out.write_all(b"Some(&p").unwrap();
        out.write_all(number.as_bytes()).unwrap();
        out.write_all(b"::solution),").unwrap();
    }

    out.write_all(b"];").unwrap();
    out.flush().unwrap();
}
