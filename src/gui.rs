use core::cell::Cell;
use tinystr::{TinyStr4, Error};

pub mod pokemon;
pub mod health;
pub mod party;
pub mod bag;

pub const LEVEL_PREFIX: &str = "Lv";

#[derive(Debug, Default, Clone)]
pub struct IntegerStr4(Cell<Option<TinyStr4>>);

impl IntegerStr4 {
    pub fn new(integer: u16) -> Result<Self, Error> {
        Ok(Self(Cell::new(Some(to_ascii4(integer)?))))
    }
    pub fn update(&self, integer: u16) -> Result<(), Error> {
        Ok(self.0.set(Some(to_ascii4(integer)?)))
    }
    pub fn update_or_default(&self, integer: u16) {
        self.0.set(to_ascii4(integer).ok())
    }
    pub fn clear(&self) {
        self.0.set(None)
    }
    pub fn get(&self) -> &str {
        cellref(&self.0).as_deref().unwrap_or("0")
    }
}

/// maximum 4 digits
fn to_ascii4(num: u16) -> Result<TinyStr4, Error> {
    const SIZE: usize = 4;
    let mut num = num;
    let mut string = [0u8; SIZE];
    let mut place = 0;
    while num > 0 && place < SIZE - 1 {
        let digit = num % 10;
        num /= 10;
        string[3 - place] = digit as u8 + 48;
        place += 1;
    }
    let mut start = 0;
    while string[start] == 0 {
        start += 1;
    }
    TinyStr4::from_bytes(&string[start..])
}

fn cellref<'a, T>(cell: &'a Cell<T>) -> &'a T {
    unsafe { &*cell.as_ptr() }
}