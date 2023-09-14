use std::mem;
use undefined::Padded;

fn main() {
    let bs: [u8; 24] = unsafe { mem::transmute(Padded::new()) };
    let bs_slice = &bs[..];

    assert!(bs_slice.iter().all(|&x| x < 10 || x == 10 || x > 10));

    let s = bs_slice.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    dbg!(&s);
}
