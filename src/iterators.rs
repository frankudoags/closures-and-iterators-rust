// The Iterator Trait and the next Method

// The Iterator trait is defined in the standard library as follows:
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// The next method is the only required method in the definition of the Iterator trait.
// The next method is used to iterate through the items of an iterator.

// The Iterator trait also has a number of default methods defined for it.
// These methods are implemented in terms of the next method, and so you get
// them for free when implementing the Iterator trait.

#[test]
fn test_iter() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();
    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);
}

// The next method on an iterator returns an Option<T>.

// Methods that Consume the Iterator

// The sum method defined on the Iterator trait is one example of a method that consumes the iterator.
// The sum method takes ownership of the iterator we call it on.
// This is one way that iterators are similar to consuming adaptors in the iterator pattern.

#[test]
fn iterator_sum() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();
    let total: i32 = v_iter.sum();
    assert_eq!(total, 6);
}

// Methods that Produce Other Iterators

// The Iterator trait has a number of other methods defined for it that are useful in a variety of situations.
// One group of methods defined on the Iterator trait are called iterator adaptors.
// These methods allow you to change iterators into different kinds of iterators.
// You can chain multiple calls to iterator adaptors to perform complex actions in a readable way.
// But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.

// An example of an iterator adaptor method is the map method.

#[test]
fn iterator_map() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();
    let mut v_iter = v_iter.map(|x| x + 1);
    assert_eq!(v_iter.next(), Some(2));
    assert_eq!(v_iter.next(), Some(3));
    assert_eq!(v_iter.next(), Some(4));
    assert_eq!(v_iter.next(), None);
}

#[test]
fn test_map_and_collect() {
    let v_1 = vec![1, 2, 3];
    let v: Vec<_> = v_1.iter().map(|x| x + 1).collect();
    assert_eq!(v, vec![2, 3, 4]);
}

// Using Closures that Capture Their Environment
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn _shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn filter_shoe_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let ans = _shoes_in_size(shoes, 10);

        assert_eq!(
            ans,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
