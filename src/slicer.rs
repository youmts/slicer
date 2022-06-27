use std::collections::LinkedList;
use std::iter::*;

pub fn slice<S, D>(src: Vec<S>, dest: Vec<D>) -> Vec<(S, D)>
where
    S: SliceItem<i32> + Copy + std::fmt::Debug,
    D: SliceItem<i32> + Copy + std::fmt::Debug,
{
    let mut src_ll = LinkedList::new();
    for s in src {
        src_ll.push_back(s)
    }
    let mut dest_ll = LinkedList::new();
    for d in dest {
        dest_ll.push_back(d)
    }

    // TODO: a little slow algorithm
    let mut x = 0i32;
    for d in dest_ll.iter() {
        x += d.get_key();
        src_ll = slice_list(src_ll, x);
    }

    let mut x: i32 = 0i32;
    for s in src_ll.iter() {
        x += s.get_key();
        dest_ll = slice_list(dest_ll, x);
    }

    zip(src_ll.into_iter(), dest_ll.into_iter()).collect::<Vec<(S, D)>>()
}

pub fn slice_list<T>(list: LinkedList<T>, at: i32) -> LinkedList<T>
where
    T: SliceItem<i32> + Copy + std::fmt::Debug,
{
    let mut x: i32 = 0i32;
    let mut result = LinkedList::new();
    for item in list {
        let v = item.get_key();
        if x < at && at < x + v {
            // NOTE: split element
            let mut a = item;
            *a.get_mut_key() = at - x;
            result.push_back(a);

            let mut b = item;
            *b.get_mut_key() = x + v - at;
            result.push_back(b);
        } else {
            // NOTE: no split element
            result.push_back(item);
        }
        x += item.get_key();
    }

    result
}

pub trait SliceItem<K> {
    fn get_key(&self) -> K;
    fn get_mut_key(&mut self) -> &mut K;
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

    impl SliceItem<i32> for Src<'_> {
        fn get_key(&self) -> i32 {
            self.qty
        }
        fn get_mut_key(&mut self) -> &mut i32 {
            &mut self.qty
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Dest<'a> {
        key: &'a String,
        qty: i32,
    }

    impl SliceItem<i32> for Dest<'_> {
        fn get_key(&self) -> i32 {
            self.qty
        }
        fn get_mut_key(&mut self) -> &mut i32 {
            &mut self.qty
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

        // TODO: fix price(sliced)
        assert_eq!(
            vec![
                (
                    Src {
                        key: key_a,
                        qty: 3,
                        price: 50
                    },
                    Dest { key: key_x, qty: 3 }
                ),
                (
                    Src {
                        key: key_a,
                        qty: 2,
                        price: 50
                    },
                    Dest { key: key_y, qty: 2 }
                ),
                (
                    Src {
                        key: key_b,
                        qty: 2,
                        price: 100
                    },
                    Dest { key: key_y, qty: 2 }
                ),
                (
                    Src {
                        key: key_b,
                        qty: 3,
                        price: 100
                    },
                    Dest { key: key_z, qty: 3 }
                ),
            ],
            result
        );
    }
}
