use crate::backend;
use crate::i18n::{Language, Text};
use crate::model::Settings;
use crate::session::Session;
use crate::{
    buttons::{Buttons, Kind},
    device,
};
use gpui::{prelude::*, *};
use gpui_component::{
    button::{Button, ButtonCustomVariant, ButtonVariants},
    input::{Input, InputState},
    switch::Switch,
};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Duration,
};

const BODY: u32 = 0x242425;
const SCREEN: u32 = 0x191a1a;
const TEXT: u32 = 0xd1d1c9;
const MUTED: u32 = 0x969990;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Screen {
    #[default]
    Download,
    Settings,
}

pub struct Downloader {
    link: Entity<InputState>,
    destination: Entity<InputState>,
    settings: Settings,
    session: Session,
    cancel: Arc<AtomicBool>,
    buttons: Buttons,
    screen: Screen,
    preferences_error: Option<Text>,
}

impl Drop for Downloader {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

impl Downloader {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut settings = backend::load_settings();
        if settings.destination.is_empty() {
            settings.destination = backend::default_destination()
                .to_string_lossy()
                .into_owned();
        }
        // Resolve the initial/legacy automatic preference to a selected language.
        if settings.language == Language::System {
            settings.language = Language::system();
            let _ = backend::save_settings(&settings);
        }
        let link = cx.new(|cx| {
            InputState::new(window, cx).placeholder(settings.language.text(Text::PasteLink))
        });
        let destination = cx.new(|cx| {
            let mut input = InputState::new(window, cx);
            input.set_value(settings.destination.clone(), window, cx);
            input
        });
        Self {
            link,
            destination,
            settings,
            session: Session::default(),
            cancel: Arc::new(AtomicBool::new(false)),
            buttons: Buttons::new(cx),
            screen: Screen::Download,
            preferences_error: None,
        }
    }

    fn trigger(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.screen == Screen::Settings {
            window.blur();
            self.screen = Screen::Download;
            cx.notify();
            return;
        }
        if self.session.is_busy() {
            self.cancel.store(true, Ordering::Relaxed);
            self.session.cancel();
            cx.notify();
            return;
        }
        let id = match backend::parse_id(self.link.read(cx).value().as_ref()) {
            Ok(id) => id,
            Err(error) => {
                self.session = Session::Failed(crate::i18n::error_key(&error));
                cx.notify();
                return;
            }
        };
        let destination = self.destination.read(cx).value().to_string();
        if !PathBuf::from(&destination).is_absolute() {
            self.session = Session::Failed(Text::AbsoluteFolder);
            cx.notify();
            return;
        }
        self.settings.destination = destination.clone();
        let _ = backend::save_settings(&self.settings);
        self.session.start();
        self.cancel = Arc::new(AtomicBool::new(false));
        let (tx, rx) = mpsc::channel();
        backend::start(id, destination.into(), tx, self.cancel.clone());
        cx.spawn(async move |view, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(120))
                    .await;
                let mut events = Vec::new();
                let disconnected = loop {
                    match rx.try_recv() {
                        Ok(event) => events.push(event),
                        Err(mpsc::TryRecvError::Empty) => break false,
                        Err(mpsc::TryRecvError::Disconnected) => break true,
                    }
                };
                let done = view.update(cx, |this, cx| {
                    for event in events {
                        if let Some(done) = this.session.apply(event) {
                            if this.settings.auto_open_folder {
                                open_folder(&done.folder);
                            }
                            this.settings.history.insert(0, done);
                            this.settings.history.truncate(30);
                            let _ = backend::save_settings(&this.settings);
                        }
                    }
                    if disconnected && this.session.is_busy() {
                        this.session = Session::Failed(Text::Interrupted);
                    }
                    cx.notify();
                    !this.session.is_busy()
                });
                match done {
                    Ok(false) => {}
                    _ => break,
                }
            }
        })
        .detach();
        cx.notify();
    }

    fn choose_folder(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let picker = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some(self.settings.language.text(Text::SaveHere).into()),
        });
        cx.spawn_in(window, async move |view, cx| {
            if let Ok(Ok(Some(paths))) = picker.await
                && let Some(path) = paths.first()
            {
                let path = path.to_string_lossy().to_string();
                let _ = view.update_in(cx, |this, window, cx| {
                    this.destination
                        .update(cx, |input, cx| input.set_value(path.clone(), window, cx));
                    this.settings.destination = path;
                    let _ = backend::save_settings(&this.settings);
                    cx.notify();
                });
            }
        })
        .detach();
    }

    fn persist_preferences(&mut self, cx: &mut Context<Self>) {
        self.preferences_error = backend::save_settings(&self.settings)
            .err()
            .map(|_| Text::SettingsSaveFailed);
        cx.notify();
    }

    fn set_language(&mut self, language: Language, window: &mut Window, cx: &mut Context<Self>) {
        self.settings.language = language;
        self.link.update(cx, |input, cx| {
            input.set_placeholder(language.text(Text::PasteLink), window, cx)
        });
        self.persist_preferences(cx);
    }

    fn settings_screen(&self, cx: &mut Context<Self>) -> Div {
        let lang = self.settings.language;
        let selected = self.settings.language;
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .w_full()
            .h_full()
            .child(caption(lang.text(Text::Settings)))
            .child(div().h(px(1.)).bg(rgb(0x343633)))
            .child(caption(lang.text(Text::Language)))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .children(Language::OPTIONS.into_iter().map(|language| {
                        Button::new(("language", language as usize))
                            .label(language.name(lang))
                            .custom(device_button(cx))
                            .w_full()
                            .h(px(28.))
                            .rounded(px(4.))
                            .text_size(px(12.))
                            .bg(rgb(if language == selected {
                                0x3a4037
                            } else {
                                0x222423
                            }))
                            .border_color(rgb(if language == selected {
                                0x69735d
                            } else {
                                0x323631
                            }))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.set_language(language, window, cx)
                            }))
                    })),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(8.))
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(11.))
                            .child(lang.text(Text::AutoOpen)),
                    )
                    .child(
                        Switch::new("auto-open")
                            .checked(self.settings.auto_open_folder)
                            .tooltip(lang.text(if self.settings.auto_open_folder {
                                Text::Enabled
                            } else {
                                Text::Disabled
                            }))
                            .on_click(cx.listener(|this, enabled, _, cx| {
                                this.settings.auto_open_folder = *enabled;
                                this.persist_preferences(cx);
                            })),
                    ),
            )
            .child(div().flex_1())
            .child(caption(lang.text(
                self.preferences_error.unwrap_or(Text::PreferencesSaved),
            )))
    }
}

fn open_folder(folder: &std::path::Path) {
    let _ = std::process::Command::new("explorer.exe")
        .arg(folder)
        .spawn();
}

fn caption(text: impl Into<SharedString>) -> Div {
    div()
        .font_family("Segoe UI")
        .text_size(px(10.))
        .text_color(rgb(MUTED))
        .child(text.into())
}

fn vent() -> Div {
    div()
        .flex()
        .gap(px(5.))
        .justify_center()
        .children((0..32).map(|_| div().w(px(3.)).h(px(3.)).rounded_full().bg(rgb(0x101011))))
}

fn device_button(cx: &App) -> ButtonCustomVariant {
    ButtonCustomVariant::new(cx)
        .color(rgb(0x303132).into())
        .foreground(rgb(TEXT).into())
        .border(rgb(0x414244).into())
        .hover(rgb(0x3b3d3e).into())
        .active(rgb(0x202122).into())
}

impl Render for Downloader {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let light = if self.session.error(self.settings.language).is_some() {
            0xb35b60
        } else if self.session.is_busy() {
            0xbf8b63
        } else if self.session.saved_folder().is_some() {
            0x88967a
        } else {
            0x74746d
        };
        let lang = self.settings.language;
        let time = self.session.elapsed().unwrap_or(0);
        let content = if self.screen == Screen::Settings {
            self.settings_screen(cx)
        } else {
            div()
                .flex()
                .flex_col()
                .gap(px(16.))
                .w_full()
                .h_full()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(caption(lang.text(Text::Link)))
                        .child(
                            Input::new(&self.link)
                                .appearance(false)
                                .bordered(false)
                                .disabled(self.session.is_busy())
                                .h(px(46.))
                                .text_size(px(12.))
                                .text_color(rgb(TEXT))
                                .font_family("Segoe UI"),
                        )
                        .child(div().h(px(1.)).bg(rgb(0x343633))),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(caption(lang.text(Text::Destination)))
                        .child(
                            div()
                                .flex()
                                .gap(px(8.))
                                .items_center()
                                .child(
                                    div().flex_1().min_w(px(0.)).child(
                                        Input::new(&self.destination)
                                            .appearance(false)
                                            .bordered(false)
                                            .disabled(self.session.is_busy())
                                            .h(px(46.))
                                            .text_size(px(12.))
                                            .text_color(rgb(TEXT))
                                            .font_family("Segoe UI"),
                                    ),
                                )
                                .child(
                                    self.buttons.wrap(
                                        Kind::Folder,
                                        Button::new("folder")
                                            .label("···")
                                            .custom(device_button(cx))
                                            .compact()
                                            .text_size(px(10.))
                                            .font_family("Segoe UI")
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.choose_folder(window, cx)
                                            })),
                                        self.session.is_busy(),
                                    ),
                                ),
                        )
                        .child(div().h(px(1.)).bg(rgb(0x343633))),
                )
                .when_some(
                    self.session
                        .error(self.settings.language)
                        .map(str::to_owned),
                    |el, error| {
                        el.child(
                            div()
                                .id("screen-error")
                                .w_full()
                                .max_h(px(64.))
                                .flex_shrink_0()
                                .overflow_y_scroll()
                                .text_size(px(11.))
                                .font_family("Segoe UI")
                                .text_color(rgb(0xd5a09a))
                                .child(error),
                        )
                    },
                )
                .child(div().flex_1())
                .when_some(self.session.progress(), |el, fraction| {
                    el.child(
                        div()
                            .w_full()
                            .h(px(2.))
                            .flex_shrink_0()
                            .rounded_full()
                            .bg(rgb(0x32352f))
                            .child(
                                div()
                                    .w(relative(fraction))
                                    .h_full()
                                    .rounded_full()
                                    .bg(rgb(0xadb7a0)),
                            ),
                    )
                })
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .gap(px(8.))
                        .child(
                            div()
                                .id("saved-folder")
                                .cursor_pointer()
                                .child(caption(self.session.status(self.settings.language)))
                                .on_click(cx.listener(|this, _, _, _| {
                                    if let Some(folder) = this.session.saved_folder() {
                                        open_folder(folder);
                                    }
                                })),
                        )
                        .when(self.session.is_busy(), |el| {
                            el.child(caption(format!("{:02}:{:02}", time / 60, time % 60)))
                        }),
                )
        };
        let screen = div()
            .flex()
            .flex_col()
            .p(px(22.))
            .w_full()
            .h(px(360.))
            .bg(rgb(SCREEN))
            .border_2()
            .border_color(rgb(0x0e0e0f))
            .rounded(px(5.))
            .shadow_md()
            .child(content);

        div()
            .id("device-body")
            .size_full()
            .text_color(rgb(TEXT))
            .font_family(
                if lang == Language::Japanese
                    || (lang == Language::System && Language::system() == Language::Japanese)
                {
                    "Yu Gothic UI"
                } else {
                    "Segoe UI"
                },
            )
            .p(px(device::SHADOW_MARGIN))
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p(px(24.))
                    .gap(px(24.))
                    .bg(rgb(BODY))
                    .rounded(px(device::RADIUS))
                    .shadow(vec![BoxShadow {
                        color: rgba(0x00000050).into(),
                        offset: point(px(0.), px(1.)),
                        blur_radius: px(6.),
                        spread_radius: px(0.),
                    }])
                    .child(
                        div()
                            .w_full()
                            .h(px(22.))
                            .flex()
                            .items_center()
                            .gap(px(8.))
                            .child(
                                self.buttons.wrap(
                                    Kind::Settings,
                                    Button::new("settings")
                                        .child(
                                            svg()
                                                .path(if self.screen == Screen::Settings {
                                                    "icons/back.svg"
                                                } else {
                                                    "icons/settings.svg"
                                                })
                                                .size(px(15.))
                                                .flex_none()
                                                .text_color(rgb(0xb5b5ae)),
                                        )
                                        .custom(device_button(cx))
                                        .compact()
                                        .p_0()
                                        .text_size(px(14.))
                                        .tooltip(lang.text(if self.screen == Screen::Settings {
                                            Text::Back
                                        } else {
                                            Text::Settings
                                        }))
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            window.blur();
                                            this.screen = if this.screen == Screen::Download {
                                                Screen::Settings
                                            } else {
                                                Screen::Download
                                            };
                                            cx.notify();
                                        })),
                                    false,
                                ),
                            )
                            .child(
                                div()
                                    .id("device-grip")
                                    .window_control_area(WindowControlArea::Drag)
                                    .flex_1()
                                    .h_full()
                                    .cursor_pointer()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .w(px(120.))
                                            .h(px(6.))
                                            .rounded_full()
                                            .border_1()
                                            .border_color(rgb(0x3b3b3d))
                                            .bg(rgb(0x111112)),
                                    ),
                            )
                            .child(
                                self.buttons.wrap(
                                    Kind::Minimize,
                                    Button::new("minimize")
                                        .label("−")
                                        .custom(device_button(cx))
                                        .compact()
                                        .p_0()
                                        .text_size(px(14.))
                                        .on_click(|_, window, _| window.minimize_window()),
                                    false,
                                ),
                            )
                            .child(
                                self.buttons.wrap(
                                    Kind::Close,
                                    Button::new("close")
                                        .label("×")
                                        .custom(device_button(cx))
                                        .compact()
                                        .p_0()
                                        .text_size(px(14.))
                                        .on_click(|_, window, _| window.remove_window()),
                                    false,
                                ),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(caption("W O R K S H O P"))
                            .child(div().flex().gap(px(5.)).children((0..3).map(|i| {
                                div().size(px(4.)).rounded_full().bg(rgb(if i == 0 {
                                    0xaaa9a1
                                } else {
                                    0x515153
                                }))
                            }))),
                    )
                    .child(screen)
                    .child(div().flex_1())
                    .child(
                        div()
                            .relative()
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                self.buttons.wrap(
                                    Kind::Download,
                                    Button::new("download")
                                        .label(lang.text(if self.screen == Screen::Settings {
                                            Text::Back
                                        } else if self.session.is_busy() {
                                            Text::Stop
                                        } else {
                                            Text::Download
                                        }))
                                        .custom(device_button(cx))
                                        .border_1()
                                        .font_family("Segoe UI")
                                        .text_size(px(12.))
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.trigger(window, cx)
                                        })),
                                    false,
                                ),
                            )
                            .child(
                                div()
                                    .absolute()
                                    .left(px(8.))
                                    .bottom(px(12.))
                                    .w(px(28.))
                                    .h(px(28.))
                                    .rounded_full()
                                    .bg(rgb(0x202021))
                                    .border_1()
                                    .border_color(rgb(0x101011))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(div().size(px(6.)).rounded_full().bg(rgb(light))),
                            ),
                    )
                    .child(div().flex_1())
                    .child(vent()),
            )
    }
}
