use core::panic;
use std::cmp::Ordering;
use std::iter::*;

pub fn slice_item<T>(item: T, slice_x: i32) -> (T, T)
where
    T: SliceItem<i32, i32> + Copy + std::fmt::Debug,
{
    let item_x = item.get_key();

    let mut left = item;
    *left.get_mut_key() = slice_x;
    for value in left.get_mut_values().into_iter() {
        *value = *value * slice_x / item_x;
    }

    let mut right = item;
    *right.get_mut_key() = item_x - slice_x;
    for value in right.get_mut_values().into_iter() {
        *value = *value * (item_x - slice_x) / item_x;
    }

    (left, right)
}

pub fn slice<S, D>(src: Vec<S>, dest: Vec<D>) -> Vec<(S, D)>
where
    S: SliceItem<i32, i32> + Copy + std::fmt::Debug,
    D: SliceItem<i32, i32> + Copy + std::fmt::Debug,
{
    let mut src_ret = Vec::new();
    let mut dest_ret = Vec::new();

    let mut src_iter = src.iter();
    let mut dest_iter = dest.iter();

    let src_item = src_iter.next();
    let dest_item = dest_iter.next();

    if src_item.is_none() {
        panic!("Src must not be empty array.")
    }
    if dest_item.is_none() {
        panic!("Dest must not be empty array.")
    }

    let mut src_item = Box::new(*src_item.unwrap());
    let mut dest_item = Box::new(*dest_item.unwrap());

    let mut src_x = 0;
    let mut dest_x = 0;

    loop {
        let src_key = src_item.get_key();
        let dest_key = dest_item.get_key();
        let src_x_next = src_x + src_key;
        let dest_x_next = dest_x + dest_key;

        match src_x_next.cmp(&dest_x_next) {
            Ordering::Greater => {
                dest_ret.push(*dest_item);

                let (left, right) = slice_item(*src_item, dest_key);
                src_ret.push(left);
                src_item = Box::new(right);

                src_x = dest_x_next;
                dest_x = dest_x_next;

                dest_item = match dest_iter.next() {
                    Some(item) => Box::new(*item),
                    None => break,
                };
            }
            Ordering::Less => {
                src_ret.push(*src_item);

                let (left, right) = slice_item(*dest_item, src_key);
                dest_ret.push(left);
                dest_item = Box::new(right);

                src_x = src_x_next;
                dest_x = src_x_next;

                src_item = match src_iter.next() {
                    Some(item) => Box::new(*item),
                    None => break,
                };
            }
            Ordering::Equal => {
                src_ret.push(*src_item);
                dest_ret.push(*dest_item);

                src_item = match src_iter.next() {
                    Some(item) => Box::new(*item),
                    None => break,
                };

                dest_item = match dest_iter.next() {
                    Some(item) => Box::new(*item),
                    None => break,
                };
            }
        }
    }

    zip(src_ret.into_iter(), dest_ret.into_iter()).collect::<Vec<(S, D)>>()
}

// TODO: change to lambda
pub trait SliceItem<K, V> {
    fn get_key(&self) -> K;
    fn get_mut_key(&mut self) -> &mut K;
    fn get_mut_values(&mut self) -> Vec<&mut V>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Src<'a> {
        key: &'a String,
        qty: i32,
        price: i32,
    }

    impl SliceItem<i32, i32> for Src<'_> {
        fn get_key(&self) -> i32 {
            self.qty
        }
        fn get_mut_key(&mut self) -> &mut i32 {
            &mut self.qty
        }
        fn get_mut_values(&mut self) -> Vec<&mut i32> {
            vec![&mut self.price]
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Dest<'a> {
        key: &'a String,
        qty: i32,
    }

    impl SliceItem<i32, i32> for Dest<'_> {
        fn get_key(&self) -> i32 {
            self.qty
        }
        fn get_mut_key(&mut self) -> &mut i32 {
            &mut self.qty
        }
        fn get_mut_values(&mut self) -> Vec<&mut i32> {
            vec![]
        }
    }

    #[test]
    fn slice_same_range() {
        let key_a = &"a".to_owned();
        let key_b = &"b".to_owned();
        let key_x = &"x".to_owned();
        let key_y = &"y".to_owned();
        let key_z = &"z".to_owned();

        let src = vec![
            Src {
                key: key_a,
                qty: 5,
                price: 50,
            },
            Src {
                key: key_b,
                qty: 5,
                price: 100,
            },
        ];

        let dest = vec![
            Dest { key: key_x, qty: 3 },
            Dest { key: key_y, qty: 4 },
            Dest { key: key_z, qty: 3 },
        ];

        let result = slice(src, dest);

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
                        price: 20
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
                        price: 60
                    },
                    Dest { key: key_z, qty: 3 }
                ),
            ],
            result
        );
    }
}
