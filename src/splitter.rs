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

    let mut src_x = V::zero();
    let mut dest_x = V::zero();

    let mut ret = Vec::new();
    loop {
        let src_key = src_item.get_key();
        let dest_key = dest_item.get_key();
        let src_x_next = src_x + src_key;
        let dest_x_next = dest_x + dest_key;

        match src_x_next.cmp(&dest_x_next) {
            Ordering::Greater => {
                let (left, right) = split_item(src_item, dest_key);

                ret.push((left, dest_item));

                src_item = right;
                dest_item = match dest_iter.next() {
                    Some(item) => item,
                    None => break,
                };

                src_x = dest_x_next;
                dest_x = dest_x_next;
            }
            Ordering::Less => {
                let (left, right) = split_item(dest_item, src_key);

                ret.push((src_item, left));

                src_item = match src_iter.next() {
                    Some(item) => item,
                    None => break,
                };
                dest_item = right;

                src_x = src_x_next;
                dest_x = src_x_next;
            }
            Ordering::Equal => {
                ret.push((src_item, dest_item));

                src_item = match src_iter.next() {
                    Some(item) => item,
                    None => break,
                };
                dest_item = match dest_iter.next() {
                    Some(item) => item,
                    None => break,
                };

                src_x = src_x_next;
                dest_x = dest_x_next;
            }
        }
    }

    ret
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
    fn split_item_test() {
        let item = Src {
            key: &"a".to_owned(),
            qty: 2,
            price: 3,
        };

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

    #[test]
    fn split_same_range() {
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
