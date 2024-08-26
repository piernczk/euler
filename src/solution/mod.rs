// this file has to be here in order for rust-analyzer to not go kaput

automod::dir!("src/solution");
include!(concat!(env!("OUT_DIR"), "/solutions.rs"));
