pub use solana_program::{log, pubkey};
use std::str;
use numtoa::NumToA;
#[allow(unused_imports)]
use solana_program::log::sol_log;

pub const KEY_VAL_DELIM: char = '`';
pub const FIELD_DELIM: char = '^';

pub trait FastDisplay {
    fn fast_fmt(&self, buffer: &mut String);
}

pub struct KeyValue<'s, T: FastDisplay> {
    key: &'s str,
    value: T,
}

pub fn kv<T: FastDisplay>(key: &str, value: T) -> KeyValue<T> {
    KeyValue{ key, value }
}

impl<T: FastDisplay> FastDisplay for KeyValue<'_, T> {
    fn fast_fmt(&self, buffer: &mut String) {
        buffer.push_str(self.key);
        buffer.push(KEY_VAL_DELIM);
        self.value.fast_fmt(buffer);
    }
}

impl FastDisplay for pubkey::Pubkey {
    fn fast_fmt(&self, buffer: &mut String) {
        let bytes = self.to_bytes();
        buffer.push_str(bs58::encode(bytes.as_slice()).into_string().as_str());
    }
}

impl FastDisplay for &str {
    fn fast_fmt(&self, buffer: &mut String) {
        buffer.push_str(self);
    }
}

impl FastDisplay for String {
    fn fast_fmt(&self, buffer: &mut String) {
        buffer.push_str(self.as_str());
    }
}

impl FastDisplay for bool {
    fn fast_fmt(&self, buffer: &mut String) {
        buffer.push_str(if *self { "b1" } else { "b0" });
    }
}

macro_rules! gen_fast_display_number {
    ($type:tt, $max_digits:expr) => {
        impl FastDisplay for $type {
            fn fast_fmt(&self, dest: &mut String) {
                // Create a non heap allocated string and parse the int into that string.
                // This is more memory efficient than having each string be heap allocated.
                // Allocate 1 more than is actually required for the negative sign.
                let mut buffer = [0u8; $max_digits + 1];
                let str = self.numtoa_str(10, &mut buffer);
                dest.push_str(str);
            }
        }
    }
}

gen_fast_display_number!(i8, 3);
gen_fast_display_number!(i16, 5);
gen_fast_display_number!(i32, 10);
gen_fast_display_number!(i64, 20);
gen_fast_display_number!(u8, 3);
gen_fast_display_number!(u16, 5);
gen_fast_display_number!(u32, 10);
gen_fast_display_number!(u64, 20);

#[allow(unused_macros)]
#[macro_export]
macro_rules! fast_fmt {
    ($($arg:expr),+) => {
        {
            let mut buffer = String::new();
            $($arg.fast_fmt(&mut buffer); buffer.push($crate::FIELD_DELIM);)*
            buffer
        }
    }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! log {
    ($($arg:expr),+) => ($crate::log::sol_log(&$crate::fast_fmt!($($arg),*)));
}
