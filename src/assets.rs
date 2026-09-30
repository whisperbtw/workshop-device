use gpui::{AssetSource, SharedString};
use std::borrow::Cow;

pub struct Assets;
impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        Ok(match path {
            "icons/settings.svg" => Some(Cow::Borrowed(include_bytes!("../assets/settings.svg"))),
            "icons/back.svg" => Some(Cow::Borrowed(include_bytes!("../assets/back.svg"))),
            _ => None,
        })
    }
    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(["icons/settings.svg", "icons/back.svg"]
            .into_iter()
            .filter(|name| name.starts_with(path))
            .map(Into::into)
            .collect())
    }
}
