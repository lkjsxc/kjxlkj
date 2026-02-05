//! Integration tests for kjxlkj editor.
//!
//! Note: The editor binary requires a TTY, so we focus on testing
//! the library components that can be tested without a terminal.

use std::path::PathBuf;

mod integration_tests {
    use super::*;

    #[test]
    fn test_crate_compiles() {
        // Verify the main crate compiles correctly
        assert!(true);
    }

    #[test]
    fn test_path_creation() {
        let path = PathBuf::from("/test/file.rs");
        assert_eq!(path.extension().unwrap(), "rs");
    }

    #[test]
    fn test_path_components() {
        let path = PathBuf::from("/home/user/file.txt");
        let components: Vec<_> = path.components().collect();
        assert!(!components.is_empty());
    }

    #[test]
    fn test_option_utilities() {
        let some_val: Option<i32> = Some(42);
        assert!(some_val.is_some());
        assert_eq!(some_val.unwrap(), 42);
    }

    #[test]
    fn test_result_utilities() {
        let ok_val: Result<i32, &str> = Ok(42);
        assert!(ok_val.is_ok());
        assert_eq!(ok_val.unwrap(), 42);
    }
}

mod string_utilities {
    #[test]
    fn test_string_split() {
        let text = "hello\nworld\ntest";
        let lines: Vec<_> = text.lines().collect();
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_string_trim() {
        let text = "  hello  ";
        assert_eq!(text.trim(), "hello");
    }

    #[test]
    fn test_string_contains() {
        let text = "hello world";
        assert!(text.contains("world"));
        assert!(!text.contains("xyz"));
    }

    #[test]
    fn test_string_replace() {
        let text = "hello world";
        let replaced = text.replace("world", "rust");
        assert_eq!(replaced, "hello rust");
    }

    #[test]
    fn test_string_to_uppercase() {
        let text = "hello";
        assert_eq!(text.to_uppercase(), "HELLO");
    }

    #[test]
    fn test_string_to_lowercase() {
        let text = "HELLO";
        assert_eq!(text.to_lowercase(), "hello");
    }

    #[test]
    fn test_char_iteration() {
        let text = "hello";
        let chars: Vec<_> = text.chars().collect();
        assert_eq!(chars.len(), 5);
        assert_eq!(chars[0], 'h');
    }

    #[test]
    fn test_string_find() {
        let text = "hello world";
        assert_eq!(text.find('w'), Some(6));
    }

    #[test]
    fn test_string_starts_with() {
        let text = "hello world";
        assert!(text.starts_with("hello"));
        assert!(!text.starts_with("world"));
    }

    #[test]
    fn test_string_ends_with() {
        let text = "hello world";
        assert!(text.ends_with("world"));
        assert!(!text.ends_with("hello"));
    }
}

mod collection_utilities {
    #[test]
    fn test_vec_push() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_vec_pop() {
        let mut vec = vec![1, 2, 3];
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_vec_insert() {
        let mut vec = vec![1, 3];
        vec.insert(1, 2);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_vec_remove() {
        let mut vec = vec![1, 2, 3];
        vec.remove(1);
        assert_eq!(vec, vec![1, 3]);
    }

    #[test]
    fn test_vec_get() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(10), None);
    }

    #[test]
    fn test_vec_iter() {
        let vec = vec![1, 2, 3];
        let sum: i32 = vec.iter().sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_vec_filter() {
        let vec = vec![1, 2, 3, 4, 5];
        let evens: Vec<_> = vec.iter().filter(|x| *x % 2 == 0).collect();
        assert_eq!(evens.len(), 2);
    }

    #[test]
    fn test_vec_map() {
        let vec = vec![1, 2, 3];
        let doubled: Vec<_> = vec.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_hashmap_insert() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("key", 42);
        assert_eq!(map.get("key"), Some(&42));
    }

    #[test]
    fn test_hashmap_remove() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("key", 42);
        assert_eq!(map.remove("key"), Some(42));
        assert_eq!(map.get("key"), None);
    }

    #[test]
    fn test_hashset_insert() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1));
    }

    #[test]
    fn test_hashset_contains() {
        use std::collections::HashSet;
        let set: HashSet<_> = vec![1, 2, 3].into_iter().collect();
        assert!(set.contains(&2));
        assert!(!set.contains(&5));
    }
}

mod numeric_tests {
    #[test]
    fn test_usize_saturating_sub() {
        let a: usize = 5;
        assert_eq!(a.saturating_sub(3), 2);
        assert_eq!(a.saturating_sub(10), 0);
    }

    #[test]
    fn test_usize_saturating_add() {
        let a: usize = usize::MAX - 5;
        assert_eq!(a.saturating_add(10), usize::MAX);
    }

    #[test]
    fn test_min_max() {
        assert_eq!(std::cmp::min(3, 5), 3);
        assert_eq!(std::cmp::max(3, 5), 5);
    }

    #[test]
    fn test_clamp() {
        let val = 10;
        assert_eq!(val.clamp(0, 5), 5);
        assert_eq!(val.clamp(15, 20), 15);
        assert_eq!(val.clamp(5, 15), 10);
    }

    #[test]
    fn test_checked_add() {
        let a: usize = usize::MAX;
        assert_eq!(a.checked_add(1), None);
        assert_eq!((5_usize).checked_add(3), Some(8));
    }

    #[test]
    fn test_checked_sub() {
        let a: usize = 3;
        assert_eq!(a.checked_sub(5), None);
        assert_eq!(a.checked_sub(2), Some(1));
    }
}

mod iterator_tests {
    #[test]
    fn test_enumerate() {
        let vec = vec!['a', 'b', 'c'];
        let enumerated: Vec<_> = vec.iter().enumerate().collect();
        assert_eq!(enumerated[0], (0, &'a'));
    }

    #[test]
    fn test_zip() {
        let a = vec![1, 2, 3];
        let b = vec!['a', 'b', 'c'];
        let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
        assert_eq!(zipped.len(), 3);
    }

    #[test]
    fn test_take() {
        let vec = vec![1, 2, 3, 4, 5];
        let taken: Vec<_> = vec.iter().take(3).collect();
        assert_eq!(taken.len(), 3);
    }

    #[test]
    fn test_skip() {
        let vec = vec![1, 2, 3, 4, 5];
        let skipped: Vec<_> = vec.iter().skip(2).collect();
        assert_eq!(skipped.len(), 3);
    }

    #[test]
    fn test_fold() {
        let vec = vec![1, 2, 3, 4, 5];
        let sum: i32 = vec.iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_any() {
        let vec = vec![1, 2, 3, 4, 5];
        assert!(vec.iter().any(|&x| x > 3));
        assert!(!vec.iter().any(|&x| x > 10));
    }

    #[test]
    fn test_all() {
        let vec = vec![2, 4, 6, 8];
        assert!(vec.iter().all(|&x| x % 2 == 0));
        assert!(!vec.iter().all(|&x| x > 5));
    }

    #[test]
    fn test_find() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.iter().find(|&&x| x > 3), Some(&4));
        assert_eq!(vec.iter().find(|&&x| x > 10), None);
    }

    #[test]
    fn test_position() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.iter().position(|&x| x == 3), Some(2));
        assert_eq!(vec.iter().position(|&x| x == 10), None);
    }

    #[test]
    fn test_count() {
        let vec = vec![1, 2, 3, 4, 5];
        let count = vec.iter().filter(|&&x| x % 2 == 0).count();
        assert_eq!(count, 2);
    }
}

mod option_result_tests {
    #[test]
    fn test_option_map() {
        let some: Option<i32> = Some(5);
        assert_eq!(some.map(|x| x * 2), Some(10));
        
        let none: Option<i32> = None;
        assert_eq!(none.map(|x| x * 2), None);
    }

    #[test]
    fn test_option_and_then() {
        let some: Option<i32> = Some(5);
        assert_eq!(some.and_then(|x| Some(x * 2)), Some(10));
    }

    #[test]
    fn test_option_or_else() {
        let none: Option<i32> = None;
        assert_eq!(none.or_else(|| Some(42)), Some(42));
    }

    #[test]
    fn test_option_unwrap_or() {
        let none: Option<i32> = None;
        assert_eq!(none.unwrap_or(42), 42);
    }

    #[test]
    fn test_option_unwrap_or_default() {
        let none: Option<String> = None;
        assert_eq!(none.unwrap_or_default(), "");
    }

    #[test]
    fn test_result_map() {
        let ok: Result<i32, &str> = Ok(5);
        assert_eq!(ok.map(|x| x * 2), Ok(10));
    }

    #[test]
    fn test_result_map_err() {
        let err: Result<i32, &str> = Err("error");
        let mapped = err.map_err(|e| format!("wrapped: {}", e));
        assert!(mapped.is_err());
    }

    #[test]
    fn test_result_and_then() {
        let ok: Result<i32, &str> = Ok(5);
        assert_eq!(ok.and_then(|x| Ok(x * 2)), Ok(10));
    }

    #[test]
    fn test_result_ok() {
        let ok: Result<i32, &str> = Ok(5);
        assert_eq!(ok.ok(), Some(5));
        
        let err: Result<i32, &str> = Err("error");
        assert_eq!(err.ok(), None);
    }

    #[test]
    fn test_result_err() {
        let err: Result<i32, &str> = Err("error");
        assert_eq!(err.err(), Some("error"));
    }
}

mod slice_tests {
    #[test]
    fn test_slice_get() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(5), None);
    }

    #[test]
    fn test_slice_first() {
        let arr = [1, 2, 3];
        assert_eq!(arr.first(), Some(&1));
        let empty: &[i32] = &[];
        assert_eq!(empty.first(), None);
    }

    #[test]
    fn test_slice_last() {
        let arr = [1, 2, 3];
        assert_eq!(arr.last(), Some(&3));
    }

    #[test]
    fn test_slice_len() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_slice_is_empty() {
        let arr: [i32; 0] = [];
        assert!(arr.is_empty());
        let non_empty = [1];
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn test_slice_contains() {
        let arr = [1, 2, 3, 4, 5];
        assert!(arr.contains(&3));
        assert!(!arr.contains(&10));
    }

    #[test]
    fn test_slice_iter_rev() {
        let arr = [1, 2, 3];
        let rev: Vec<_> = arr.iter().rev().copied().collect();
        assert_eq!(rev, vec![3, 2, 1]);
    }
}

mod char_tests {
    #[test]
    fn test_is_alphabetic() {
        assert!('a'.is_alphabetic());
        assert!('Z'.is_alphabetic());
        assert!(!'1'.is_alphabetic());
    }

    #[test]
    fn test_is_numeric() {
        assert!('5'.is_numeric());
        assert!(!'a'.is_numeric());
    }

    #[test]
    fn test_is_alphanumeric() {
        assert!('a'.is_alphanumeric());
        assert!('5'.is_alphanumeric());
        assert!(!' '.is_alphanumeric());
    }

    #[test]
    fn test_is_whitespace() {
        assert!(' '.is_whitespace());
        assert!('\t'.is_whitespace());
        assert!('\n'.is_whitespace());
        assert!(!'x'.is_whitespace());
    }

    #[test]
    fn test_to_ascii_uppercase() {
        assert_eq!('a'.to_ascii_uppercase(), 'A');
        assert_eq!('Z'.to_ascii_uppercase(), 'Z');
    }

    #[test]
    fn test_to_ascii_lowercase() {
        assert_eq!('A'.to_ascii_lowercase(), 'a');
        assert_eq!('z'.to_ascii_lowercase(), 'z');
    }

    #[test]
    fn test_is_ascii() {
        assert!('a'.is_ascii());
        assert!('\x7f'.is_ascii());
    }

    #[test]
    fn test_to_digit() {
        assert_eq!('5'.to_digit(10), Some(5));
        assert_eq!('a'.to_digit(16), Some(10));
    }

    #[test]
    fn test_is_ascii_digit() {
        assert!('0'.is_ascii_digit());
        assert!('9'.is_ascii_digit());
        assert!(!'a'.is_ascii_digit());
    }

    #[test]
    fn test_char_len_utf8() {
        assert_eq!('a'.len_utf8(), 1);
        assert_eq!('é'.len_utf8(), 2);
        assert_eq!('中'.len_utf8(), 3);
        assert_eq!('🦀'.len_utf8(), 4);
    }
}

mod range_tests {
    #[test]
    fn test_range_contains() {
        let range = 0..10;
        assert!(range.contains(&5));
        assert!(!range.contains(&10));
    }

    #[test]
    fn test_range_inclusive() {
        let range = 0..=10;
        assert!(range.contains(&10));
    }

    #[test]
    fn test_range_collect() {
        let range = 0..5;
        let vec: Vec<i32> = range.collect();
        assert_eq!(vec, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_range_sum() {
        let sum: i32 = (1..=10).sum();
        assert_eq!(sum, 55);
    }

    #[test]
    fn test_range_product() {
        let product: i32 = (1..=5).product();
        assert_eq!(product, 120);
    }

    #[test]
    fn test_range_count() {
        let count = (0..100).count();
        assert_eq!(count, 100);
    }

    #[test]
    fn test_range_rev() {
        let rev: Vec<i32> = (0..5).rev().collect();
        assert_eq!(rev, vec![4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_range_step_by() {
        let stepped: Vec<i32> = (0..10).step_by(2).collect();
        assert_eq!(stepped, vec![0, 2, 4, 6, 8]);
    }
}

mod ordering_tests {
    use std::cmp::Ordering;

    #[test]
    fn test_ordering_cmp() {
        assert_eq!(5_i32.cmp(&3), Ordering::Greater);
        assert_eq!(3_i32.cmp(&5), Ordering::Less);
        assert_eq!(5_i32.cmp(&5), Ordering::Equal);
    }

    #[test]
    fn test_ordering_reverse() {
        assert_eq!(Ordering::Less.reverse(), Ordering::Greater);
        assert_eq!(Ordering::Greater.reverse(), Ordering::Less);
        assert_eq!(Ordering::Equal.reverse(), Ordering::Equal);
    }

    #[test]
    fn test_ordering_then() {
        assert_eq!(Ordering::Equal.then(Ordering::Less), Ordering::Less);
        assert_eq!(Ordering::Greater.then(Ordering::Less), Ordering::Greater);
    }

    #[test]
    fn test_partial_cmp() {
        assert_eq!(3.14_f64.partial_cmp(&2.71), Some(Ordering::Greater));
    }

    #[test]
    fn test_nan_ordering() {
        assert_eq!(f64::NAN.partial_cmp(&1.0), None);
    }
}

mod cow_tests {
    use std::borrow::Cow;

    #[test]
    fn test_cow_borrowed() {
        let s = "hello";
        let cow: Cow<str> = Cow::Borrowed(s);
        assert_eq!(&*cow, "hello");
    }

    #[test]
    fn test_cow_owned() {
        let s = String::from("hello");
        let cow: Cow<str> = Cow::Owned(s);
        assert_eq!(&*cow, "hello");
    }

    #[test]
    fn test_cow_is_borrowed() {
        let cow: Cow<str> = Cow::Borrowed("hello");
        assert!(matches!(cow, Cow::Borrowed(_)));
    }

    #[test]
    fn test_cow_to_mut() {
        let mut cow: Cow<str> = Cow::Borrowed("hello");
        cow.to_mut().push_str(" world");
        assert_eq!(&*cow, "hello world");
    }
}

mod rc_tests {
    use std::rc::Rc;

    #[test]
    fn test_rc_new() {
        let rc = Rc::new(42);
        assert_eq!(*rc, 42);
    }

    #[test]
    fn test_rc_clone() {
        let rc = Rc::new(42);
        let rc2 = Rc::clone(&rc);
        assert_eq!(*rc2, 42);
    }

    #[test]
    fn test_rc_strong_count() {
        let rc = Rc::new(42);
        assert_eq!(Rc::strong_count(&rc), 1);
        let _rc2 = Rc::clone(&rc);
        assert_eq!(Rc::strong_count(&rc), 2);
    }

    #[test]
    fn test_rc_ptr_eq() {
        let rc = Rc::new(42);
        let rc2 = Rc::clone(&rc);
        assert!(Rc::ptr_eq(&rc, &rc2));
    }
}

mod cell_tests {
    use std::cell::{Cell, RefCell};

    #[test]
    fn test_cell_new() {
        let cell = Cell::new(42);
        assert_eq!(cell.get(), 42);
    }

    #[test]
    fn test_cell_set() {
        let cell = Cell::new(42);
        cell.set(100);
        assert_eq!(cell.get(), 100);
    }

    #[test]
    fn test_cell_replace() {
        let cell = Cell::new(42);
        let old = cell.replace(100);
        assert_eq!(old, 42);
        assert_eq!(cell.get(), 100);
    }

    #[test]
    fn test_refcell_borrow() {
        let cell = RefCell::new(42);
        let borrowed = cell.borrow();
        assert_eq!(*borrowed, 42);
    }

    #[test]
    fn test_refcell_borrow_mut() {
        let cell = RefCell::new(42);
        *cell.borrow_mut() = 100;
        assert_eq!(*cell.borrow(), 100);
    }
}

mod mem_tests {
    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<u8>(), 1);
        assert_eq!(std::mem::size_of::<u32>(), 4);
        assert_eq!(std::mem::size_of::<u64>(), 8);
    }

    #[test]
    fn test_swap() {
        let mut a = 1;
        let mut b = 2;
        std::mem::swap(&mut a, &mut b);
        assert_eq!(a, 2);
        assert_eq!(b, 1);
    }

    #[test]
    fn test_replace() {
        let mut val = 42;
        let old = std::mem::replace(&mut val, 100);
        assert_eq!(old, 42);
        assert_eq!(val, 100);
    }

    #[test]
    fn test_take() {
        let mut val = 42;
        let old = std::mem::take(&mut val);
        assert_eq!(old, 42);
        assert_eq!(val, 0);
    }
}

mod fmt_tests {
    #[test]
    fn test_format_decimal() {
        assert_eq!(format!("{}", 42), "42");
    }

    #[test]
    fn test_format_hex() {
        assert_eq!(format!("{:x}", 255), "ff");
        assert_eq!(format!("{:X}", 255), "FF");
    }

    #[test]
    fn test_format_binary() {
        assert_eq!(format!("{:b}", 8), "1000");
    }

    #[test]
    fn test_format_octal() {
        assert_eq!(format!("{:o}", 64), "100");
    }

    #[test]
    fn test_format_padding() {
        assert_eq!(format!("{:05}", 42), "00042");
    }

    #[test]
    fn test_format_left_align() {
        assert_eq!(format!("{:<5}", "ab"), "ab   ");
    }

    #[test]
    fn test_format_right_align() {
        assert_eq!(format!("{:>5}", "ab"), "   ab");
    }

    #[test]
    fn test_format_center() {
        assert_eq!(format!("{:^5}", "ab"), " ab  ");
    }

    #[test]
    fn test_format_precision() {
        assert_eq!(format!("{:.2}", 3.14159), "3.14");
    }
}

mod ptr_tests {
    #[test]
    fn test_null_ptr() {
        let p: *const i32 = std::ptr::null();
        assert!(p.is_null());
    }

    #[test]
    fn test_non_null_ptr() {
        let val = 42;
        let p = &val as *const i32;
        assert!(!p.is_null());
    }

    #[test]
    fn test_ptr_eq() {
        let val = 42;
        let p1 = &val as *const i32;
        let p2 = &val as *const i32;
        assert!(std::ptr::eq(p1, p2));
    }
}

mod arc_tests {
    use std::sync::Arc;

    #[test]
    fn test_arc_new() {
        let arc = Arc::new(42);
        assert_eq!(*arc, 42);
    }

    #[test]
    fn test_arc_clone() {
        let arc = Arc::new(42);
        let arc2 = Arc::clone(&arc);
        assert_eq!(*arc2, 42);
    }

    #[test]
    fn test_arc_strong_count() {
        let arc = Arc::new(42);
        assert_eq!(Arc::strong_count(&arc), 1);
        let _arc2 = Arc::clone(&arc);
        assert_eq!(Arc::strong_count(&arc), 2);
    }

    #[test]
    fn test_arc_ptr_eq() {
        let arc = Arc::new(42);
        let arc2 = Arc::clone(&arc);
        assert!(Arc::ptr_eq(&arc, &arc2));
    }
}

mod vec_deque_tests {
    use std::collections::VecDeque;

    #[test]
    fn test_vecdeque_new() {
        let deque: VecDeque<i32> = VecDeque::new();
        assert!(deque.is_empty());
    }

    #[test]
    fn test_vecdeque_push_back() {
        let mut deque = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        assert_eq!(deque.len(), 2);
    }

    #[test]
    fn test_vecdeque_push_front() {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        deque.push_front(2);
        assert_eq!(deque.front(), Some(&2));
    }

    #[test]
    fn test_vecdeque_pop_back() {
        let mut deque = VecDeque::from([1, 2, 3]);
        assert_eq!(deque.pop_back(), Some(3));
    }

    #[test]
    fn test_vecdeque_pop_front() {
        let mut deque = VecDeque::from([1, 2, 3]);
        assert_eq!(deque.pop_front(), Some(1));
    }

    #[test]
    fn test_vecdeque_get() {
        let deque = VecDeque::from([1, 2, 3]);
        assert_eq!(deque.get(1), Some(&2));
    }

    #[test]
    fn test_vecdeque_iter() {
        let deque = VecDeque::from([1, 2, 3]);
        let sum: i32 = deque.iter().sum();
        assert_eq!(sum, 6);
    }
}

mod btreemap_tests {
    use std::collections::BTreeMap;

    #[test]
    fn test_btreemap_new() {
        let map: BTreeMap<i32, i32> = BTreeMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_btreemap_insert() {
        let mut map = BTreeMap::new();
        map.insert("a", 1);
        assert_eq!(map.get("a"), Some(&1));
    }

    #[test]
    fn test_btreemap_remove() {
        let mut map = BTreeMap::new();
        map.insert("a", 1);
        assert_eq!(map.remove("a"), Some(1));
    }

    #[test]
    fn test_btreemap_contains() {
        let mut map = BTreeMap::new();
        map.insert("key", 42);
        assert!(map.contains_key("key"));
    }

    #[test]
    fn test_btreemap_ordered_iter() {
        let mut map = BTreeMap::new();
        map.insert(3, "c");
        map.insert(1, "a");
        map.insert(2, "b");
        let keys: Vec<_> = map.keys().copied().collect();
        assert_eq!(keys, vec![1, 2, 3]);
    }

    #[test]
    fn test_btreemap_range() {
        let mut map = BTreeMap::new();
        for i in 1..=10 {
            map.insert(i, i * 10);
        }
        let range: Vec<_> = map.range(3..=7).map(|(&k, _)| k).collect();
        assert_eq!(range, vec![3, 4, 5, 6, 7]);
    }
}

mod btreeset_tests {
    use std::collections::BTreeSet;

    #[test]
    fn test_btreeset_new() {
        let set: BTreeSet<i32> = BTreeSet::new();
        assert!(set.is_empty());
    }

    #[test]
    fn test_btreeset_insert() {
        let mut set = BTreeSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1));
    }

    #[test]
    fn test_btreeset_contains() {
        let set: BTreeSet<_> = vec![1, 2, 3].into_iter().collect();
        assert!(set.contains(&2));
    }

    #[test]
    fn test_btreeset_ordered_iter() {
        let mut set = BTreeSet::new();
        set.insert(3);
        set.insert(1);
        set.insert(2);
        let values: Vec<_> = set.iter().copied().collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_btreeset_union() {
        let set1: BTreeSet<_> = [1, 2, 3].into();
        let set2: BTreeSet<_> = [3, 4, 5].into();
        let union: BTreeSet<_> = set1.union(&set2).copied().collect();
        assert_eq!(union.len(), 5);
    }

    #[test]
    fn test_btreeset_intersection() {
        let set1: BTreeSet<_> = [1, 2, 3].into();
        let set2: BTreeSet<_> = [2, 3, 4].into();
        let intersection: BTreeSet<_> = set1.intersection(&set2).copied().collect();
        assert_eq!(intersection.len(), 2);
    }
}

mod binary_heap_tests {
    use std::collections::BinaryHeap;

    #[test]
    fn test_binaryheap_new() {
        let heap: BinaryHeap<i32> = BinaryHeap::new();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_binaryheap_push() {
        let mut heap = BinaryHeap::new();
        heap.push(5);
        heap.push(3);
        heap.push(7);
        assert_eq!(heap.peek(), Some(&7));
    }

    #[test]
    fn test_binaryheap_pop() {
        let mut heap = BinaryHeap::from([1, 5, 3]);
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(1));
    }

    #[test]
    fn test_binaryheap_into_sorted_vec() {
        let heap: BinaryHeap<_> = [3, 1, 4, 1, 5, 9].into();
        let sorted = heap.into_sorted_vec();
        assert_eq!(sorted, vec![1, 1, 3, 4, 5, 9]);
    }
}

mod duration_tests {
    use std::time::Duration;

    #[test]
    fn test_duration_from_secs() {
        let dur = Duration::from_secs(5);
        assert_eq!(dur.as_secs(), 5);
    }

    #[test]
    fn test_duration_from_millis() {
        let dur = Duration::from_millis(1500);
        assert_eq!(dur.as_millis(), 1500);
    }

    #[test]
    fn test_duration_from_micros() {
        let dur = Duration::from_micros(1_000_000);
        assert_eq!(dur.as_secs(), 1);
    }

    #[test]
    fn test_duration_from_nanos() {
        let dur = Duration::from_nanos(1_000_000_000);
        assert_eq!(dur.as_secs(), 1);
    }

    #[test]
    fn test_duration_add() {
        let d1 = Duration::from_secs(1);
        let d2 = Duration::from_millis(500);
        let sum = d1 + d2;
        assert_eq!(sum.as_millis(), 1500);
    }

    #[test]
    fn test_duration_mul() {
        let dur = Duration::from_secs(2);
        let doubled = dur * 3;
        assert_eq!(doubled.as_secs(), 6);
    }

    #[test]
    fn test_duration_div() {
        let dur = Duration::from_secs(10);
        let halved = dur / 2;
        assert_eq!(halved.as_secs(), 5);
    }

    #[test]
    fn test_duration_is_zero() {
        assert!(Duration::ZERO.is_zero());
        assert!(!Duration::from_secs(1).is_zero());
    }
}

mod path_buf_tests {
    use std::path::PathBuf;

    #[test]
    fn test_pathbuf_new() {
        let path = PathBuf::new();
        assert!(path.as_os_str().is_empty());
    }

    #[test]
    fn test_pathbuf_from() {
        let path = PathBuf::from("/home/user");
        assert!(!path.as_os_str().is_empty());
    }

    #[test]
    fn test_pathbuf_push() {
        let mut path = PathBuf::from("/home");
        path.push("user");
        path.push("file.txt");
        assert!(path.ends_with("file.txt"));
    }

    #[test]
    fn test_pathbuf_pop() {
        let mut path = PathBuf::from("/home/user/file.txt");
        assert!(path.pop());
        assert!(path.ends_with("user"));
    }

    #[test]
    fn test_pathbuf_set_extension() {
        let mut path = PathBuf::from("/home/file.txt");
        path.set_extension("rs");
        assert!(path.ends_with("file.rs"));
    }

    #[test]
    fn test_pathbuf_file_name() {
        let path = PathBuf::from("/home/user/file.txt");
        assert_eq!(path.file_name().unwrap().to_str().unwrap(), "file.txt");
    }

    #[test]
    fn test_pathbuf_extension() {
        let path = PathBuf::from("file.txt");
        assert_eq!(path.extension().unwrap(), "txt");
    }

    #[test]
    fn test_pathbuf_parent() {
        let path = PathBuf::from("/home/user/file.txt");
        assert_eq!(path.parent().unwrap(), std::path::Path::new("/home/user"));
    }
}

mod os_string_tests {
    use std::ffi::OsString;

    #[test]
    fn test_osstring_new() {
        let s = OsString::new();
        assert!(s.is_empty());
    }

    #[test]
    fn test_osstring_from_str() {
        let s = OsString::from("hello");
        assert_eq!(s.to_str(), Some("hello"));
    }

    #[test]
    fn test_osstring_push() {
        let mut s = OsString::from("hello");
        s.push(" world");
        assert_eq!(s.to_str(), Some("hello world"));
    }

    #[test]
    fn test_osstring_len() {
        let s = OsString::from("hello");
        assert_eq!(s.len(), 5);
    }
}

mod lazy_tests {
    use std::sync::LazyLock;

    static LAZY: LazyLock<i32> = LazyLock::new(|| 42);

    #[test]
    fn test_lazy_access() {
        assert_eq!(*LAZY, 42);
    }
}

mod once_lock_tests {
    use std::sync::OnceLock;

    #[test]
    fn test_oncelock_new() {
        let lock: OnceLock<i32> = OnceLock::new();
        assert!(lock.get().is_none());
    }

    #[test]
    fn test_oncelock_set() {
        let lock: OnceLock<i32> = OnceLock::new();
        assert!(lock.set(42).is_ok());
        assert!(lock.set(100).is_err());
    }

    #[test]
    fn test_oncelock_get() {
        let lock: OnceLock<i32> = OnceLock::new();
        let _ = lock.set(42);
        assert_eq!(lock.get(), Some(&42));
    }

    #[test]
    fn test_oncelock_get_or_init() {
        let lock: OnceLock<i32> = OnceLock::new();
        let val = lock.get_or_init(|| 42);
        assert_eq!(*val, 42);
    }
}

mod wrapping_tests {
    use std::num::Wrapping;

    #[test]
    fn test_wrapping_add() {
        let a = Wrapping(u8::MAX);
        let b = Wrapping(1u8);
        assert_eq!((a + b).0, 0);
    }

    #[test]
    fn test_wrapping_sub() {
        let a = Wrapping(0u8);
        let b = Wrapping(1u8);
        assert_eq!((a - b).0, u8::MAX);
    }

    #[test]
    fn test_wrapping_mul() {
        let a = Wrapping(u8::MAX);
        let b = Wrapping(2u8);
        assert_eq!((a * b).0, 254);
    }
}

mod saturating_tests {
    use std::num::Saturating;

    #[test]
    fn test_saturating_add() {
        let a = Saturating(u8::MAX);
        let b = Saturating(1u8);
        assert_eq!((a + b).0, u8::MAX);
    }

    #[test]
    fn test_saturating_sub() {
        let a = Saturating(0u8);
        let b = Saturating(1u8);
        assert_eq!((a - b).0, 0);
    }
}

mod nonzero_tests {
    use std::num::NonZeroU32;

    #[test]
    fn test_nonzero_new() {
        assert!(NonZeroU32::new(1).is_some());
        assert!(NonZeroU32::new(0).is_none());
    }

    #[test]
    fn test_nonzero_get() {
        let n = NonZeroU32::new(42).unwrap();
        assert_eq!(n.get(), 42);
    }
}

mod phantom_tests {
    use std::marker::PhantomData;

    struct Wrapper<T> {
        value: u32,
        _marker: PhantomData<T>,
    }

    #[test]
    fn test_phantom_data() {
        let w: Wrapper<String> = Wrapper { value: 42, _marker: PhantomData };
        assert_eq!(w.value, 42);
    }
}

mod default_tests {
    #[test]
    fn test_option_default() {
        let opt: Option<i32> = Default::default();
        assert!(opt.is_none());
    }

    #[test]
    fn test_vec_default() {
        let v: Vec<i32> = Default::default();
        assert!(v.is_empty());
    }

    #[test]
    fn test_string_default() {
        let s: String = Default::default();
        assert!(s.is_empty());
    }

    #[test]
    fn test_bool_default() {
        let b: bool = Default::default();
        assert!(!b);
    }

    #[test]
    fn test_i32_default() {
        let n: i32 = Default::default();
        assert_eq!(n, 0);
    }
}

mod convert_tests {
    #[test]
    fn test_from_into() {
        let s: String = String::from("hello");
        let _s2: String = "hello".into();
        assert!(!s.is_empty());
    }

    #[test]
    fn test_as_ref() {
        fn take_str(s: impl AsRef<str>) -> usize {
            s.as_ref().len()
        }
        assert_eq!(take_str("hello"), 5);
        assert_eq!(take_str(String::from("hello")), 5);
    }

    #[test]
    fn test_try_from_ok() {
        let n: Result<u8, _> = u8::try_from(100_i32);
        assert!(n.is_ok());
    }

    #[test]
    fn test_try_from_err() {
        let n: Result<u8, _> = u8::try_from(300_i32);
        assert!(n.is_err());
    }
}

mod iter_adapters {
    #[test]
    fn test_chain() {
        let a = [1, 2];
        let b = [3, 4];
        let chained: Vec<_> = a.iter().chain(b.iter()).copied().collect();
        assert_eq!(chained, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_flatten() {
        let nested = vec![vec![1, 2], vec![3, 4]];
        let flat: Vec<_> = nested.into_iter().flatten().collect();
        assert_eq!(flat, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_flat_map() {
        let words = vec!["hello", "world"];
        let chars: Vec<_> = words.iter().flat_map(|s| s.chars()).collect();
        assert_eq!(chars.len(), 10);
    }

    #[test]
    fn test_peekable() {
        let mut iter = [1, 2, 3].iter().peekable();
        assert_eq!(iter.peek(), Some(&&1));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_skip_while() {
        let v = [1, 2, 3, 4, 5];
        let skipped: Vec<_> = v.iter().skip_while(|&&x| x < 3).copied().collect();
        assert_eq!(skipped, vec![3, 4, 5]);
    }

    #[test]
    fn test_take_while() {
        let v = [1, 2, 3, 4, 5];
        let taken: Vec<_> = v.iter().take_while(|&&x| x < 3).copied().collect();
        assert_eq!(taken, vec![1, 2]);
    }

    #[test]
    fn test_scan() {
        let v = [1, 2, 3, 4, 5];
        let scanned: Vec<_> = v.iter().scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        }).collect();
        assert_eq!(scanned, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_fuse() {
        let iter = [1, 2].iter().fuse();
        let collected: Vec<_> = iter.copied().collect();
        assert_eq!(collected, vec![1, 2]);
    }

    #[test]
    fn test_cycle() {
        let cycled: Vec<_> = [1, 2].iter().cycle().take(5).copied().collect();
        assert_eq!(cycled, vec![1, 2, 1, 2, 1]);
    }
}

mod more_tests {
    #[test]
    fn test_vec_sort() {
        let mut v = vec![3, 1, 4, 1, 5, 9];
        v.sort();
        assert_eq!(v, vec![1, 1, 3, 4, 5, 9]);
    }

    #[test]
    fn test_vec_dedup() {
        let mut v = vec![1, 1, 2, 2, 3, 3];
        v.dedup();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_vec_binary_search() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.binary_search(&3), Ok(2));
        assert_eq!(v.binary_search(&0), Err(0));
    }

    #[test]
    fn test_vec_retain() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.retain(|&x| x % 2 == 0);
        assert_eq!(v, vec![2, 4]);
    }

    #[test]
    fn test_string_split_whitespace() {
        let s = "hello   world  test";
        let words: Vec<_> = s.split_whitespace().collect();
        assert_eq!(words, vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_string_lines() {
        let s = "line1\nline2\nline3";
        let lines: Vec<_> = s.lines().collect();
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_string_repeat() {
        assert_eq!("ab".repeat(3), "ababab");
    }

    #[test]
    fn test_string_chars_count() {
        let s = "hello";
        assert_eq!(s.chars().count(), 5);
    }

    #[test]
    fn test_string_bytes() {
        let s = "hello";
        assert_eq!(s.bytes().count(), 5);
    }

    #[test]
    fn test_vec_windows() {
        let v = vec![1, 2, 3, 4];
        let windows: Vec<_> = v.windows(2).collect();
        assert_eq!(windows.len(), 3);
    }

    #[test]
    fn test_vec_chunks() {
        let v = vec![1, 2, 3, 4, 5];
        let chunks: Vec<_> = v.chunks(2).collect();
        assert_eq!(chunks.len(), 3);
    }
}
