// NOTE: Unimplemented
use std::iter::*;

pub fn slice<S, D>(src: Vec<S>, dest: Vec<D>) -> Vec<(i32, (S, D))>
where
    S: SliceItem<i32> + std::fmt::Debug,
    D: SliceItem<i32> + std::fmt::Debug,
{
    let srcs = src.iter().scan(0, |s, x| {
        let key = x.get_key();
        *s = *s + key;
        Some((*s, x))
    });
    let mut dests = dest.iter().map(|x| (x.get_key(), x));

    // println!("{:?}", srcs.collect::<(i32, &S)>());
    // println!("{:?}", dests.collect::<(i32, D)>());

    // let result = vec!();
    // let before_sum_key = 0;
    // let now_sum_key = 0;
    // let now_dest = None;
    // for (src_key, src_value) in srcs {
    //     let (dest_key, dest_value) = match now_dest {
    //         None => match dests.next() {
    //             Some(d) => d,
    //             None => { // TODO: dest side end.. take rest of src side
    //                 break;
    //             }
    //         },
    //         Some(x) => x,
    //     };

    //     if dest_key < src_key {
    //         now_sum_key += dest_key;
    //     }

    //     before_sum_key = now_sum_key;
    // }

    vec![]
}

pub trait SliceItem<K> {
    fn get_key(&self) -> K;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Src {
        qty: i32,
        price: i32,
    }

    impl SliceItem<i32> for Src {
        fn get_key(&self) -> i32 {
            self.qty
        }
    }

    #[derive(Debug, PartialEq)]
    struct Dest {
        qty: i32,
    }

    impl SliceItem<i32> for Dest {
        fn get_key(&self) -> i32 {
            self.qty
        }
    }

    #[test]
    #[ignore]
    fn slice_same_range() {
        let src = vec![Src { qty: 5, price: 50 }, Src { qty: 5, price: 100 }];

        let dest = vec![Dest { qty: 3 }, Dest { qty: 4 }, Dest { qty: 3 }];

        let result = slice(src, dest);

        assert_eq!(
            vec![
                (3, (Src { qty: 3, price: 30 }, Dest { qty: 3 })),
                (2, (Src { qty: 2, price: 20 }, Dest { qty: 2 })),
                (2, (Src { qty: 2, price: 40 }, Dest { qty: 2 })),
                (5, (Src { qty: 3, price: 60 }, Dest { qty: 3 })),
            ],
            result
        );
    }
}
