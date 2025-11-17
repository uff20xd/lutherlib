mod redoxri;
use redoxri::*;

fn main() {
    let redoxri = Redoxri::new(&[

    ]);

    let main = Mcule::new("lutherlib", "lutherlib.rlib")
        .with(&["lib.rs".into()])
        .add_step(&["rustc", "lib.rs", "-o", "$out", "--crate-type=lib"])
        .compile();
}
