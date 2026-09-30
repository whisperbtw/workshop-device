#![windows_subsystem = "windows"]

mod assets;
mod backend;
mod buttons;
mod device;
mod diagnostics;
mod i18n;
mod install;
mod model;
mod native;
mod session;
mod steamcmd;
mod ui;

use gpui::{prelude::*, *};
use gpui_component::{Root, Theme, ThemeMode};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).is_some_and(|a| a == "--download") {
        let result = cli_download(&args);
        if let Err(error) = result {
            eprintln!("{error:#}");
            std::process::exit(1);
        }
        return;
    }
    // DirectComposition preserves alpha at the antialiased device outline.
    unsafe {
        std::env::remove_var("GPUI_DISABLE_DIRECT_COMPOSITION");
    }
    Application::new()
        .with_assets(assets::Assets)
        .run(|cx: &mut App| {
            gpui_component::init(cx);
            Theme::change(ThemeMode::Dark, None, cx);
            Theme::global_mut(cx).background = transparent_black();
            let bounds = Bounds::centered(
                None,
                size(px(device::WINDOW_WIDTH), px(device::WINDOW_HEIGHT)),
                cx,
            );
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    // Fixed native device size; avoid a minimum including the original frame.
                    is_resizable: false,
                    window_background: WindowBackgroundAppearance::Transparent,
                    titlebar: Some(TitlebarOptions {
                        title: Some("Workshop".into()),
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| ui::Downloader::new(window, cx));
                    window.defer(cx, |window, _| native::configure(window));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .expect("Não foi possível abrir a janela GPUI");
            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
            cx.activate(true);
        });
}

fn cli_download(args: &[String]) -> anyhow::Result<()> {
    use std::{
        path::PathBuf,
        sync::{Arc, atomic::AtomicBool, mpsc},
    };
    let id = backend::parse_id(
        args.get(2)
            .ok_or_else(|| anyhow::anyhow!("Informe um link"))?,
    )?;
    let dest = PathBuf::from(
        args.get(3)
            .ok_or_else(|| anyhow::anyhow!("Informe uma pasta absoluta"))?,
    );
    let (tx, rx) = mpsc::channel();
    backend::start(id, dest, tx, Arc::new(AtomicBool::new(false)));
    for event in rx {
        match event {
            model::Event::Stage(stage) => println!("{}", stage.label()),
            model::Event::Progress { done, total } => println!("{done}/{total} bytes"),
            model::Event::Metadata(item) => {
                println!("{} · {} ({} bytes)", item.game, item.title, item.bytes)
            }
            model::Event::Complete(done) => {
                println!("SALVO: {} ({} bytes)", done.folder.display(), done.bytes);
                return Ok(());
            }
            model::Event::Failed(failure) => {
                eprintln!("{}", failure.diagnostic);
                anyhow::bail!(i18n::Language::System.text(failure.kind));
            }
            model::Event::Cancelled => anyhow::bail!("Download cancelado."),
        }
    }
    anyhow::bail!("O download terminou sem resultado")
}
