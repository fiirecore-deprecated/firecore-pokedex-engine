use pokedex::item::{ItemStack, StackSize};

#[derive(Debug)]
pub struct ItemStackInstance {
    pub stack: *mut ItemStack,              // we do a little trolling
    count: ([u8; 4], Option<StackSize>),    // i think this is fine
}

impl ItemStackInstance {
    pub fn stack(&self) -> &mut ItemStack {
        unsafe { self.stack.as_mut().unwrap() }
    }

    pub fn count(&mut self) -> &str {
        let count = self.stack().count;
        if self.count.1 != Some(count) {
            itoa::write(self.count.0.as_mut(), count).unwrap();
            self.count.1 = Some(count);
        }
        unsafe { core::str::from_utf8_unchecked(&self.count.0) }
    }
}

impl From<&mut ItemStack> for ItemStackInstance {
    fn from(stack: &mut ItemStack) -> Self {
        Self {
            stack: stack as *mut ItemStack,
            count: Default::default(),
        }
    }
}
