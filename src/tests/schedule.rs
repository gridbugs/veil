use util::schedule::*;

#[test]
fn insert_drain() {
    let mut s = Schedule::new();

    s.insert("second", 1);
    s.insert("fourth", 2);
    s.insert("first", 0);
    s.insert("third", 1);

    assert_eq!(s.next_value(), Some("first"));
    assert_eq!(s.next_value(), Some("second"));
    assert_eq!(s.next_value(), Some("third"));
    assert_eq!(s.next_value(), Some("fourth"));
    assert_eq!(s.next_value(), None);
    assert_eq!(s.next_value(), None);
}

#[test]
fn insert_remove() {
    let mut s = Schedule::new();

    let second = s.insert("second", 1);
    let fourth = s.insert("fourth", 2);

    assert!(s.remove(second));

    let first = s.insert("first", 0);

    assert!(!s.remove(second));

    assert_eq!(s.next_value(), Some("first"));

    s.insert("third", 1);

    assert!(!s.remove(first));

    assert_eq!(s.next_value(), Some("third"));
    assert_eq!(s.next_value(), Some("fourth"));
    assert_eq!(s.next_value(), None);

    assert!(!s.remove(fourth));
}

#[test]
fn time_calculation() {
    let mut s = Schedule::new();

    s.insert("first", 1);
    let entry = s.next().unwrap();
    assert_eq!(entry.duration, 1);
    assert_eq!(entry.release_time, 1);

    s.insert("second", 2);
    let entry = s.next().unwrap();
    assert_eq!(entry.duration, 2);
    assert_eq!(entry.release_time, 3);

    assert_eq!(s.absolute_time(), 3);

    let third = s.insert("third", 1);
    let fourth = s.insert("fourth", 2);
    s.remove(third);
    s.remove(fourth);
    assert_eq!(s.next(), None);

    assert_eq!(s.absolute_time(), 3);

    s.insert("fifth", 0);
    s.insert("seventh", 1);
    s.insert("sixth", 0);
    assert_eq!(s.next().unwrap().release_time, 3);
    assert_eq!(s.next().unwrap().release_time, 3);
    assert_eq!(s.next().unwrap().release_time, 4);

    assert_eq!(s.absolute_time(), 4);
}
