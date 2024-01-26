mod solution {
    automod::dir!("src/solution");
    include!(concat!(env!("OUT_DIR"), "/array.rs"));
}

use solution::SOLUTIONS;

pub fn get_solution(number: usize) -> Option<&'static (dyn Fn() -> String + Send + 'static)> {
    unsafe { SOLUTIONS.get(number - 1).and_then(|p| Some(&*(*p)?)) }
}
