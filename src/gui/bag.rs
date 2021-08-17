use core::cell::Cell;

use engine::{
    graphics::{draw_cursor, draw_o, draw_text_left, position},
    gui::Panel,
    input::{pressed, Control},
    tetra::graphics::Texture,
    text::TextColor,
    util::HEIGHT,
    EngineContext,
};

use pokedex::item::{ItemRefStack, ItemRef};

use crate::context::PokedexClientContext;

// const WORLD_OPTIONS: &[&'static str] = &[
//     "Use",
//     "Give",
//     "Toss",
// ];

type TextOption = &'static [&'static str];

const BATTLE_OPTIONS: TextOption = &["Use"];

pub struct BagGui {
    alive: Cell<bool>,
    background: Texture,

    offset: Cell<usize>,
    cursor: Cell<usize>,

    selecting: Cell<bool>,
    select_cursor: Cell<usize>,
    // select_text: Cell<Option<TextOption>>,
    // items: [Cell<Option<TinyStr4>>; 12],

    selected: Cell<Option<usize>>,
}

impl BagGui {
    pub fn new<U>(ctx: &PokedexClientContext<U>) -> Self {
        Self {
            alive: Default::default(),
            background: ctx.bag_background.clone(),
            offset: Default::default(),
            cursor: Default::default(),
            selecting: Default::default(),
            select_cursor: Default::default(),
            // items: Default::default(),
            selected: Default::default(),
        }
    }

    pub fn input<'d>(&self, ctx: &EngineContext, items: &mut [ItemRefStack<'d>]) {
        match self.selecting.get() {
            true => {
                // match self.select_text {
                // Some(text) => {
                let cursor = self.cursor.get();
                if pressed(ctx, Control::B) {
                    self.selecting.set(false);
                }
                if pressed(ctx, Control::Up) && cursor > 0 {
                    self.select_cursor
                        .set(self.select_cursor.get() - 1);
                }
                if pressed(ctx, Control::Down) && cursor < BATTLE_OPTIONS.len() {
                    self.select_cursor
                        .set(self.select_cursor.get() + 1);
                }
                if pressed(ctx, Control::A) {
                    match cursor {
                        0 => {
                            self.selected.set(Some(cursor));
                        }
                        1 => (), // cancel
                        _ => unreachable!("Selected an option that is not use/cancel"),
                    }
                    self.selecting.set(false);
                }

                // }
                //     None => self.selecting = false,
                // }
            }
            false => {
                if pressed(ctx, Control::B) {
                    self.despawn();
                }
                let cursor = self.cursor.get();
                if pressed(ctx, Control::A) {
                    if cursor < items.len() {
                        self.spawn_select();
                    } else {
                        self.despawn();
                    }
                }
                if pressed(ctx, Control::Up) && cursor > 0 {
                    self.cursor.set(cursor - 1);
                }
                if pressed(ctx, Control::Down) {
                    if cursor < items.len() {
                        self.cursor.set(cursor + 1);
                    }
                }
            }
        }
    }

    pub fn draw<'d, U>(&self, ctx: &mut EngineContext, dex: &PokedexClientContext<U>, items: &[ItemRefStack<'d>]) {
        self.background.draw(ctx, position(0.0, 0.0));
        let cursor = self.cursor.get();
        for (index, stack) in items.iter().enumerate() {
            let y = 11.0 + (index << 4) as f32;
            draw_text_left(ctx, &1, &stack.item.name, TextColor::Black, 98.0, y);
            draw_text_left(ctx, &1, "x", TextColor::Black, 200.0, y);
            // if let Some(ref count) = self.items.get(index - self.offset.get()).map(|cell| cell.get()).flatten() {
            //     draw_text_left(ctx, &1, &count, TextColor::Black, 208.0, y);
            // }
        }
        draw_text_left(
            ctx,
            &1,
            "Cancel",
            TextColor::Black,
            98.0,
            11.0 + (items.len() << 4) as f32,
        );
        if let Some(stack) = items.get(cursor) {
            draw_o(ctx, dex.item_textures.try_get(&stack.item.id), 8.0, 125.0);
            for (index, line) in stack.item.description.iter().enumerate() {
                draw_text_left(
                    ctx,
                    &1,
                    line,
                    TextColor::White,
                    41.0,
                    117.0 + (index * 14) as f32,
                );
            }
        }
        draw_cursor(ctx, 91.0, 13.0 + (cursor << 4) as f32);
        if self.selecting.get() {
            // if let Some(text) = self.select_text {
            Panel::draw_text(
                ctx,
                146.0,
                HEIGHT,
                94.0,
                &BATTLE_OPTIONS,
                self.select_cursor.get(),
                true,
                true,
            )
            // }
        }
    }

    fn spawn_select(&self) {
        self.selecting.set(true);
        self.select_cursor.set(0);
    }

    // fn set_cell<'d>(&self, index: usize, stack: Option<&ItemRefStack<'d>>) {
    //     if let Some(cell) = self.items.get(index) {
    //         cell.set(stack.map(|stack| to_ascii4(stack.count).ok()).flatten())
    //     }
    // }

    pub fn take_selected_despawn<'d>(&self, items: &mut [ItemRefStack<'d>]) -> Option<ItemRef<'d>> {
        let selected = self.selected.get();
        selected
            .map(|selected| {
                self.selected.set(None);
                let item = items[selected]
                    .decrement()
                    .then(|| items[selected].item);
                self.despawn();
                item
            })
            .flatten()
    }

    pub fn spawn(&self) {
        self.alive.set(true);
        // self.select_text.set(Some(BATTLE_OPTIONS));
    }

    pub fn despawn(&self) {
        self.alive.set(false);
        // self.items.clear()
    }

    pub fn alive(&self) -> bool {
        self.alive.get()
    }
}
