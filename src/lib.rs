use horizon_mod_sdk::{
    Mod, Registration,
    input::{self, Event},
    render::{self, Point, QuadShape, Shape, TextShape},
};

const FADEOUT_DURATION: f32 = 0.2;
const TEXT_COLOR: u32 = 0xFFFFFF;

struct KeyBtn {
    id: &'static str,
    desc: &'static str,
    default_key: &'static str,
    label: &'static str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    pressed: bool,
    brightness: f32,
}

impl KeyBtn {
    const fn new(
        id: &'static str,
        desc: &'static str,
        default_key: &'static str,
        label: &'static str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Self {
        Self {
            id,
            desc,
            default_key,
            label,
            x,
            y,
            w,
            h,
            pressed: false,
            brightness: 0.0,
        }
    }

    fn bg_color(&self) -> u32 {
        let c = (self.brightness.clamp(0.0, 1.0) * 255.0) as u32;
        0x8000_0000 | (c << 16) | (c << 8) | c
    }
}

struct KeystrokesMod {
    buttons: [KeyBtn; 8],
}

impl Mod for KeystrokesMod {
    fn init() -> Result<(Self, Registration), String> {
        let buttons = [
            KeyBtn::new("ks_w", "Keystrokes W", "key.keyboard.w", "W", 27.0, 5.0, 20.0, 20.0),
            KeyBtn::new("ks_a", "Keystrokes A", "key.keyboard.a", "A", 5.0, 27.0, 20.0, 20.0),
            KeyBtn::new("ks_s", "Keystrokes S", "key.keyboard.s", "S", 27.0, 27.0, 20.0, 20.0),
            KeyBtn::new("ks_d", "Keystrokes D", "key.keyboard.d", "D", 49.0, 27.0, 20.0, 20.0),
            KeyBtn::new("ks_space", "Keystrokes Space", "key.keyboard.space", "Space", 5.0, 52.0, 42.0, 20.0),
            KeyBtn::new("ks_sft", "Keystrokes Shift", "key.keyboard.left.shift", "SFT", 49.0, 52.0, 20.0, 20.0),
            KeyBtn::new("ks_lmb", "Keystrokes LMB", "key.mouse.left", "LMB", 5.0, 77.0, 31.0, 20.0),
            KeyBtn::new("ks_rmb", "Keystrokes RMB", "key.mouse.right", "RMB", 38.0, 77.0, 31.0, 20.0),
        ];

        for b in &buttons {
            input::register_key(b.id, b.desc, b.default_key)?;
        }

        Ok((Self { buttons }, Registration::new().on_tick().on_frame()))
    }

    fn tick(&mut self, _tick: u64) {
        for event in input::poll() {
            match event {
                Event::KeyDown(id) => {
                    if let Some(b) = self.buttons.iter_mut().find(|b| b.id == id) {
                        b.pressed = true;
                        b.brightness = 1.0;
                    }
                }
                Event::KeyUp(id) => {
                    if let Some(b) = self.buttons.iter_mut().find(|b| b.id == id) {
                        b.pressed = false;
                    }
                }
                _ => {}
            }
        }
    }

    fn frame(&mut self, seconds: f32) {
        let dt = seconds.clamp(0.0, 0.25);
        let mut shapes = Vec::with_capacity(self.buttons.len() * 2);

        for b in &mut self.buttons {
            if b.pressed {
                b.brightness = 1.0;
            } else {
                b.brightness = (b.brightness - dt / FADEOUT_DURATION).max(0.0);
            }

            let x0 = b.x;
            let y0 = b.y;
            let x1 = b.x + b.w;
            let y1 = b.y + b.h;

            shapes.push(Shape::Quad(QuadShape {
                corners: (
                    Point { x: x0, y: y0 },
                    Point { x: x1, y: y0 },
                    Point { x: x1, y: y1 },
                    Point { x: x0, y: y1 },
                ),
                color: b.bg_color(),
            }));

            shapes.push(Shape::Text(TextShape {
                text: b.label.to_string(),
                at: Point {
                    x: x0 + b.w * 0.5,
                    y: y0 + (b.h - 8.0) * 0.5,
                },
                color: TEXT_COLOR,
                centered: true,
                shadow: true,
            }));
        }

        render::set_hud(&shapes);
    }
}

horizon_mod_sdk::export_mod!(KeystrokesMod);
