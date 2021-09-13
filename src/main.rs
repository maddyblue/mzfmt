use mzfmt::pretty_str;

fn main() {
    println!("{:?}", pretty_str("select 1,2", 60));
}
