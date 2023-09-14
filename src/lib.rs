#[repr(C)]
pub struct Padded {
    little: u16,
    medium: u32,
    byte: u8,
    big: u64
}

impl Padded {
    pub fn new() -> Padded {
        Padded {
            little: 1,
            medium: 2,
            byte: 3,
            big: 4
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::mem;

    #[test]
    fn test_size() {
        assert_eq!(24, mem::size_of::<Padded>())
    }

    #[test]
    fn test_values() {
        let p = Padded::new();
        assert_eq!(1, p.little);
        assert_eq!(2, p.medium);
        assert_eq!(3, p.byte);
        assert_eq!(4, p.big);
    }

    #[test]
    fn test_bytes_le() {
        let bs: [u8; 24] = unsafe { mem::transmute(Padded::new()) };
        assert_eq!(1, bs[0]);
        assert_eq!(2, bs[4]);
        assert_eq!(3, bs[8]);
        assert_eq!(4, bs[16]);

        let bs_slice = &bs[..];

        assert_eq!(1, bs_slice[0]);
        assert_eq!(2, bs_slice[4]);
        assert_eq!(3, bs_slice[8]);
        assert_eq!(4, bs_slice[16]);
    }

    #[test]
    fn test_bytes() {
        let bs: [u8; 24] = unsafe { mem::transmute(Padded::new()) };
        let bs_slice = &bs[..];

        assert!(bs_slice.iter().all(|&x| x < 10 || x == 10 || x > 10));
    }

    #[test]
    fn test_from_bytes() {
        let bs: [u8; 24] = unsafe { mem::transmute(Padded::new()) };

        let p: Padded = unsafe { mem::transmute(bs) };

        assert_eq!(1, p.little);
        assert_eq!(2, p.medium);
        assert_eq!(3, p.byte);
        assert_eq!(4, p.big);
    }
}