fn main() {
    let v = vec![1, 2, 3];

    // We can create iterators from different data structures
    // that implement the Iterator trait.
    for entry in v.iter() {
        println!("{}", entry);
    }
}

#[test]
fn it_iterates() {
    let v = vec![1, 2, 3];
    // Here we make the iterator mutable, because calling .next
    // mutates internal state of the iterator ("consumes" a value
    // from the iterator).
    let mut v_iter = v.iter();

    // Note that the values produced by .next are immutable references
    // wrapped in the Option type. If we wanted mutable references, we'd
    // call .iter_mut instead of .iter above. If we want the iterator to
    // produce owned values, we'd called .into_inter, like below.
    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);

    let mut v_iter_owned = v.into_iter();

    assert_eq!(v_iter_owned.next(), Some(1));
    assert_eq!(v_iter_owned.next(), Some(2));
    assert_eq!(v_iter_owned.next(), Some(3));
    assert_eq!(v_iter_owned.next(), None);
}

#[test]
fn iterator_sum() {
    let v = vec![1, 2, 3];

    // The sum method is an example of a _consuming adaptor_. Internally,
    // it calls .next to sum up the values in the iterator.
    let sum = v.iter().sum();

    assert_eq!(6, sum);
}

#[test]
fn iterator_collect() {
    let v = vec![1, 2, 3];

    // Iterators also have _iterator adaptors_, which allow you to alter iterators.
    // In the example below, we use a closure to transform the values of v. Note
    // that iterator adaptors are lazy, so they won't be called until we access the
    // values through a method like collect.
    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4])
}
