use gpui::{prelude::*, *};
use gpui_component::{
    Disableable,
    button::{Button, ButtonCustomVariant, ButtonVariants},
};
use std::time::Instant;

#[derive(Clone, Copy)]
pub enum Kind {
    Minimize,
    Close,
    Folder,
    Download,
    Settings,
}

impl Kind {
    fn engraved(self) -> bool {
        matches!(self, Self::Minimize | Self::Close | Self::Settings)
    }
    fn geometry(self) -> (f32, f32, f32) {
        match self {
            Self::Minimize | Self::Close | Self::Settings => (22., 11., 1.5),
            Self::Folder => (32., 8., 1.5),
            Self::Download => (144., 72., 3.),
        }
    }
}

struct Motion {
    from: f32,
    target: f32,
    started: Instant,
}

impl Motion {
    fn new() -> Self {
        Self {
            from: 0.,
            target: 0.,
            started: Instant::now(),
        }
    }
    fn sample(&self) -> (f32, bool) {
        let duration = if self.target > 0. { 0.08 } else { 0.15 };
        let t = (self.started.elapsed().as_secs_f32() / duration).min(1.);
        let eased = 1. - (1. - t).powi(3);
        (
            self.from + (self.target - self.from) * eased,
            t < 1. && self.from != self.target,
        )
    }
    fn set(&mut self, pressed: bool) {
        let target = if pressed { 1. } else { 0. };
        if self.target != target {
            self.from = self.sample().0;
            self.target = target;
            self.started = Instant::now();
        }
    }
}

pub struct Buttons([Entity<Motion>; 5]);

impl Buttons {
    pub fn new(cx: &mut App) -> Self {
        Self(std::array::from_fn(|_| cx.new(|_| Motion::new())))
    }
    pub fn wrap(&self, kind: Kind, button: Button, disabled: bool) -> PhysicalButton {
        PhysicalButton {
            kind,
            button,
            disabled,
            motion: self.0[kind as usize].clone(),
        }
    }
}

#[derive(IntoElement)]
pub struct PhysicalButton {
    kind: Kind,
    button: Button,
    disabled: bool,
    motion: Entity<Motion>,
}

impl RenderOnce for PhysicalButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (depth, animating) = self.motion.read(cx).sample();
        if animating {
            window.request_animation_frame();
        }
        let (size, radius, travel) = self.kind.geometry();
        let engraved = self.kind.engraved();
        let shade = (48. - depth * 12.).round() as u32;
        let down = self.motion.clone();
        let up = self.motion.clone();
        let outside = self.motion.clone();
        let hover = self.motion;
        div()
            .id(("button-socket", self.kind as usize))
            .relative()
            .flex_none()
            .size(px(size))
            .rounded(px(radius))
            .bg(if engraved {
                transparent_black()
            } else {
                rgb(0x141516).into()
            })
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                if !self.disabled {
                    down.update(cx, |motion, _| motion.set(true));
                    window.refresh();
                }
            })
            .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                up.update(cx, |motion, _| motion.set(false));
                window.refresh();
            })
            .on_mouse_up_out(MouseButton::Left, move |_, window, cx| {
                if outside.read(cx).target != 0. {
                    outside.update(cx, |motion, _| motion.set(false));
                    window.refresh();
                }
            })
            .on_hover(move |hovered, window, cx| {
                if !hovered && hover.read(cx).target != 0. {
                    hover.update(cx, |motion, _| motion.set(false));
                    window.refresh();
                }
            })
            .child(
                self.button
                    .when(engraved, |button| {
                        button
                            .custom(
                                ButtonCustomVariant::new(cx)
                                    .color(transparent_black())
                                    .border(transparent_black())
                                    .foreground(rgb(0xaaa9a2).into())
                                    .hover(transparent_black())
                                    .active(transparent_black()),
                            )
                            .border_0()
                    })
                    .disabled(self.disabled)
                    .w(px(size))
                    .h(px(size))
                    .rounded(px(radius))
                    .relative()
                    .top(px(travel * depth))
                    .when(depth > 0. && !engraved, |button| {
                        button.bg(rgb((shade << 16) | ((shade + 1) << 8) | (shade + 2)))
                    })
                    .when(engraved, |button| button.opacity(0.85 - depth * 0.2))
                    .shadow(if engraved {
                        vec![]
                    } else {
                        vec![BoxShadow {
                            color: rgba(0x00000070).into(),
                            offset: point(px(0.), px(2. * (1. - depth))),
                            blur_radius: px(3. * (1. - depth)),
                            spread_radius: px(0.),
                        }]
                    }),
            )
    }
}
