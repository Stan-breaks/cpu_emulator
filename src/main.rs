fn main() {
    let mut i: u16 = 65532;
    loop {
        {
            println!("{} ,{:016b}..", i, i);
            i += 1;
        }
    }
}
