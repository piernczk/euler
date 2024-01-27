mod solution {
    automod::dir!("src/solution");
    include!(concat!(env!("OUT_DIR"), "/array.rs"));
}

use std::{fs::File, io::Read, path::Path};

use solution::SOLUTIONS;

pub fn get_solution(number: usize) -> Option<&'static (dyn Fn() -> String + Send + 'static)> {
    unsafe { SOLUTIONS.get(number - 1).and_then(|p| Some(&*(*p)?)) }
}

pub fn get_data(number: usize) -> Option<String> {
    let file_name = format!("p{:03}.txt", number);
    let mut buf = String::new();
    File::open(Path::new("./data/").join(file_name))
        .ok()?
        .read_to_string(&mut buf)
        .ok()?;
    Some(buf)
}
