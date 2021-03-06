use engine::{
    graphics::draw_rectangle,
    gui::ProgressBar,
    tetra::{
        graphics::{Color, DrawParams, Texture},
        math::Vec2,
    },
    EngineContext,
};

use pokedex::pokemon::Health;

use crate::context::PokedexClientContext;

pub struct HealthBar {
    background: Texture,
    bar: ProgressBar,
}

pub struct HealthBarColor {
    upper: Color,
    lower: Color,
}

impl HealthBar {
    pub const WIDTH: f32 = 48.0;

    pub const GREEN: &'static HealthBarColor = &HealthBarColor {
        upper: Color::rgb(88.0 / 255.0, 208.0 / 255.0, 128.0 / 255.0),
        lower: Color::rgb(112.0 / 255.0, 248.0 / 255.0, 168.0 / 255.0),
    };

    pub const YELLOW: &'static HealthBarColor = &HealthBarColor {
        upper: Color::rgb(200.0 / 255.0, 168.0 / 255.0, 8.0 / 255.0),
        lower: Color::rgb(248.0 / 255.0, 224.0 / 255.0, 56.0 / 255.0),
    };

    pub const RED: &'static HealthBarColor = &HealthBarColor {
        upper: Color::rgb(168.0 / 255.0, 64.0 / 255.0, 72.0 / 255.0),
        lower: Color::rgb(248.0 / 255.0, 88.0 / 255.0, 56.0 / 255.0),
    };

    pub fn new(ctx: &PokedexClientContext) -> Self {
        Self {
            background: Self::texture(ctx).clone(),
            bar: ProgressBar::new(Self::WIDTH),
        }
    }

    pub fn with_size(ctx: &PokedexClientContext, width: f32) -> Self {
        Self {
            background: Self::texture(ctx).clone(),
            bar: ProgressBar::new(width),
        }
    }

    pub fn texture<'d, 'c>(ctx: &'c PokedexClientContext<'d>) -> &'c Texture {
        &ctx.health_bar
    }

    pub fn width(current: Health, max: Health) -> f32 {
        current as f32 * Self::WIDTH / max as f32
    }

    pub fn resize_hp(&mut self, current: Health, max: Health, reset: bool) {
        self.resize(current as f32 / max as f32, reset)
    }

    pub fn resize(&mut self, percent: f32, reset: bool) {
        self.bar.resize(Self::WIDTH * percent, reset);
    }

    pub fn is_moving(&self) -> bool {
        self.bar.moving()
    }

    pub fn update(&mut self, delta: f32) {
        self.bar.update(delta)
    }

    pub fn draw(&self, ctx: &mut EngineContext, origin: Vec2<f32>) {
        self.draw_width(ctx, origin, self.bar.width().ceil());
    }

    pub fn draw_width(&self, ctx: &mut EngineContext, origin: Vec2<f32>, width: f32) {
        self.background
            .draw(ctx, DrawParams::position(DrawParams::default(), origin));
        let x = origin.x + 15.0;
        let color = if width < Self::WIDTH / 8.0 {
            Self::RED
        } else if width < Self::WIDTH / 2.0 {
            Self::YELLOW
        } else {
            Self::GREEN
        };
        let width = width.clamp(0.0, Self::WIDTH);
        draw_rectangle(ctx, x, origin.y + 2.0, width, 1.0, color.upper);
        draw_rectangle(ctx, x, origin.y + 3.0, width, 2.0, color.lower);
    }
}
