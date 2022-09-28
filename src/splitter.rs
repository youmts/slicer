use num::Num;
use std::cmp::Ordering;
use std::iter::*;

pub fn split_item<T, V>(item: T, split_x: V) -> (T, T)
where
    T: SplitItem<V> + Clone + std::fmt::Debug,
    V: Num + Copy,
{
    let item_x = item.get_key();

    let mut left = item.clone();
    *left.get_mut_key() = split_x;
    for value in left.get_mut_values().into_iter() {
        *value = *value * split_x / item_x;
    }

    let left_values = left.get_values();
    let mut left_values_iter = left_values.iter();
    let mut right = item;
    *right.get_mut_key() = item_x - split_x;
    for value in right.get_mut_values().into_iter() {
        let left = *left_values_iter.next().unwrap();
        *value = *value - left;
    }

    (left, right)
}

macro_rules! break_none {
    ($res:expr) => {
        match $res {
            Some(val) => val,
            None => break,
        }
    };
}

pub fn split_all<S, D, V>(src: Vec<S>, dest: Vec<D>) -> Vec<(S, D)>
where
    S: SplitItem<V> + Clone + std::fmt::Debug,
    D: SplitItem<V> + Clone + std::fmt::Debug,
    V: Num + Copy + Ord,
{
    let mut src_iter = src.into_iter();
    let mut dest_iter = dest.into_iter();

    let mut src_item = src_iter.next().expect("Src must not be empty array.");
    let mut dest_item = dest_iter.next().expect("Dest must not be empty array.");

    let mut key_acc = V::zero();

    let mut result = Vec::new();
    loop {
        let src_key = src_item.get_key();
        let dest_key = dest_item.get_key();
        let src_key_acc_next = key_acc + src_key;
        let dest_key_acc_next = key_acc + dest_key;

        match src_key_acc_next.cmp(&dest_key_acc_next) {
            Ordering::Greater => {
                let (left, right) = split_item(src_item, dest_key);
                result.push((left, dest_item));

                src_item = right;
                dest_item = break_none!(dest_iter.next());
                key_acc = dest_key_acc_next;
            }
            Ordering::Less => {
                let (left, right) = split_item(dest_item, src_key);
                result.push((src_item, left));

                src_item = break_none!(src_iter.next());
                dest_item = right;
                key_acc = src_key_acc_next;
            }
            Ordering::Equal => {
                result.push((src_item, dest_item));

                src_item = break_none!(src_iter.next());
                dest_item = break_none!(dest_iter.next());
                key_acc = src_key_acc_next;
            }
        }
    }

    result
}

// TODO: 任意のNum traitを実装する型をKeyやValueの各要素に使えるようにしたい！
pub trait SplitItem<T> {
    fn get_key(&self) -> T;
    fn get_mut_key(&mut self) -> &mut T;
    fn get_values(&self) -> Vec<T>;
    fn get_mut_values(&mut self) -> Vec<&mut T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct Src<'a> {
        key: &'a String,
        qty: i32,
        price: i32,
    }

    impl SplitItem<i32> for Src<'_> {
        fn get_key(&self) -> i32 {
            self.qty
        }
        fn get_mut_key(&mut self) -> &mut i32 {
            &mut self.qty
        }
        fn get_values(&self) -> Vec<i32> {
            vec![self.price]
        }
        fn get_mut_values(&mut self) -> Vec<&mut i32> {
            vec![&mut self.price]
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    struct Dest<'a> {
        key: &'a String,
        qty: i32,
    }

    impl SplitItem<i32> for Dest<'_> {
        fn get_key(&self) -> i32 {
            self.qty
        }
        fn get_mut_key(&mut self) -> &mut i32 {
            &mut self.qty
        }
        fn get_values(&self) -> Vec<i32> {
            vec![]
        }
        fn get_mut_values(&mut self) -> Vec<&mut i32> {
            vec![]
        }
    }

    #[test]
    /// target:
    /// |       qty  : 2      |
    /// |       price: 3      |
    /// 
    /// #1 split at qty:0
    /// | qty  : 0 | qty  : 2 |
    /// | price: 0 | price: 0 |
    /// 
    /// #2 split at qty:1
    /// | qty  : 1 | qty  : 1 |
    /// | price: 1 | price: 2 |
    /// 
    /// #3 split at qty:2
    /// | qty  : 2 | qty  : 0 |
    /// | price: 3 | price: 0 |
    /// 
    fn test_split_item() {
        let item = Src {
            key: &"a".to_owned(),
            qty: 2,
            price: 3,
        };

        // #1
        assert_eq!(
            (
                Src {
                    key: &"a".to_owned(),
                    qty: 0,
                    price: 0
                },
                Src {
                    key: &"a".to_owned(),
                    qty: 2,
                    price: 3
                },
            ),
            split_item(item.clone(), 0)
        );

        assert_eq!(
            (
                Src {
                    key: &"a".to_owned(),
                    qty: 1,
                    price: 1
                },
                Src {
                    key: &"a".to_owned(),
                    qty: 1,
                    price: 2
                },
            ),
            split_item(item.clone(), 1)
        );

        assert_eq!(
            (
                Src {
                    key: &"a".to_owned(),
                    qty: 2,
                    price: 3
                },
                Src {
                    key: &"a".to_owned(),
                    qty: 0,
                    price: 0
                },
            ),
            split_item(item.clone(), 2)
        );
    }

    /// src:
    /// |       qty  :  5          |       qty  :   5         |
    /// |       price: 51          |       price: 101         |
    ///
    /// dest:
    /// |   qty  :  3  |       qty  :  4        |   qty:  3   |
    /// 
    /// result:
    /// |   qty  :  3  | qty  :  2 |   qty:  2  |   qty:  3   |
    /// |   price: 30  | price: 21 | price: 40  | price: 61   |
    /// 
    #[test]
    fn test_split_all_same_range() {
        let key_a = &"a".to_owned();
        let key_b = &"b".to_owned();
        let key_x = &"x".to_owned();
        let key_y = &"y".to_owned();
        let key_z = &"z".to_owned();

        let src = vec![
            Src {
                key: key_a,
                qty: 5,
                price: 51,
            },
            Src {
                key: key_b,
                qty: 5,
                price: 101,
            },
        ];

        let dest = vec![
            Dest { key: key_x, qty: 3 },
            Dest { key: key_y, qty: 4 },
            Dest { key: key_z, qty: 3 },
        ];

        let result = split_all(src, dest);

        assert_eq!(
            vec![
                (
                    Src {
                        key: key_a,
                        qty: 3,
                        price: 30
                    },
                    Dest { key: key_x, qty: 3 }
                ),
                (
                    Src {
                        key: key_a,
                        qty: 2,
                        price: 21
                    },
                    Dest { key: key_y, qty: 2 }
                ),
                (
                    Src {
                        key: key_b,
                        qty: 2,
                        price: 40
                    },
                    Dest { key: key_y, qty: 2 }
                ),
                (
                    Src {
                        key: key_b,
                        qty: 3,
                        price: 61
                    },
                    Dest { key: key_z, qty: 3 }
                ),
            ],
            result
        );
    }
}
