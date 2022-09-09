#[macro_export]
macro_rules! item_value_tests (
    ( $x:ty, $y:ty ) => {
	use std::rc::Rc;
	use xrust::value::Value;
	use xrust::item::{Sequence, SequenceTrait, Item};

	#[test]
	fn item_value_string_empty_to_bool() {
	    assert_eq!(Item::<$x,$y>::Value(Value::from("")).to_bool(), false)
	}
	#[test]
	fn item_value_string_nonempty_to_bool() {
	    assert_eq!(Item::<$x,$y>::Value(Value::from("false")).to_bool(), true)
	}
	#[test]
	fn item_value_int_zero_to_bool() {
	    assert_eq!(Item::<$x,$y>::Value(Value::from(0)).to_bool(), false)
	}
	#[test]
	fn item_value_int_nonzero_to_bool() {
	    assert_eq!(Item::<$x,$y>::Value(Value::from(42)).to_bool(), true)
	}

	#[test]
	fn item_value_string_to_int() {
	    match (Item::<$x,$y>::Value(Value::from("1")).to_int()) {
		Ok(i) => assert_eq!(i, 1),
		Err(_) => {
		    panic!("to_int() failed")
		}
	    }
	}
	#[test]
	fn item_value_string_to_double() {
	    assert_eq!(Item::<$x,$y>::Value(Value::from("2.0")).to_double(), 2.0)
	}

	#[test]
	fn sequence() {
            let _s = Sequence::<$x,$y>::new();
            assert!(true)
	}
	#[test]
	fn sequence_one() {
            let mut s = Sequence::<$x,$y>::new();
	    s.push_value(Value::from("one"));
	    let mut t = Sequence::<$x,$y>::new();
	    t.push_item(&s[0]);
	    assert!(Rc::ptr_eq(&s[0], &t[0]))
	}
    }
);
