use crate::i18n::{Language, Text};
use crate::model::{Download, Event, Failure, Stage};
use std::{path::Path, time::Instant};

#[derive(Default)]
pub enum Session {
    #[default]
    Ready,
    Running {
        stage: Stage,
        progress: Option<(u64, u64)>,
        started: Instant,
        cancelling: bool,
    },
    Saved(Download),
    Failed(Failure),
    Cancelled,
}

impl Session {
    pub fn start(&mut self) {
        *self = Self::Running {
            stage: Stage::Lookup,
            progress: None,
            started: Instant::now(),
            cancelling: false,
        };
    }

    pub fn is_busy(&self) -> bool {
        matches!(self, Self::Running { .. })
    }

    pub fn cancel(&mut self) {
        if let Self::Running { cancelling, .. } = self {
            *cancelling = true;
        }
    }

    pub fn status(&self, language: Language) -> &'static str {
        match self {
            Self::Ready => language.text(Text::Ready),
            Self::Running {
                cancelling: true, ..
            } => language.text(Text::Cancelling),
            Self::Running { stage, .. } => language.text(stage.text()),
            Self::Saved(_) => language.text(Text::Saved),
            Self::Failed(_) => language.text(Text::Retry),
            Self::Cancelled => language.text(Text::Cancelled),
        }
    }

    pub fn error(&self, language: Language) -> Option<&'static str> {
        match self {
            Self::Failed(failure) => Some(language.text(failure.kind)),
            _ => None,
        }
    }

    pub fn saved_folder(&self) -> Option<&Path> {
        match self {
            Self::Saved(done) => Some(&done.folder),
            _ => None,
        }
    }

    pub fn elapsed(&self) -> Option<u64> {
        match self {
            Self::Running { started, .. } => Some(started.elapsed().as_secs()),
            _ => None,
        }
    }

    pub fn progress(&self) -> Option<f32> {
        match self {
            Self::Running {
                progress: Some((done, total)),
                ..
            } if *total > 0 => Some((*done as f64 / *total as f64).clamp(0., 1.) as f32),
            Self::Saved(_) => Some(1.),
            _ => None,
        }
    }

    pub fn apply(&mut self, event: Event) -> Option<Download> {
        // Terminal states cannot be overwritten by late progress from a worker.
        if !self.is_busy() {
            return None;
        }
        match event {
            Event::Stage(next) => {
                if let Self::Running {
                    stage, progress, ..
                } = self
                {
                    *stage = next;
                    *progress = None;
                }
            }
            Event::Progress { done, total } => {
                if let Self::Running { progress, .. } = self {
                    *progress = (total > 0).then_some((done.min(total), total));
                }
            }
            Event::Metadata(_) => {}
            Event::Complete(done) => {
                *self = Self::Saved(done.clone());
                return Some(done);
            }
            Event::Failed(message) => {
                *self = Self::Failed(message);
            }
            Event::Cancelled => {
                *self = Self::Cancelled;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_change_resets_progress_and_late_events_cannot_clear_failure() {
        let mut session = Session::default();
        session.start();
        session.apply(Event::Progress {
            done: 50,
            total: 100,
        });
        assert_eq!(session.progress(), Some(0.5));
        session.apply(Event::Stage(Stage::Downloading));
        assert_eq!(session.progress(), None);
        session.apply(Event::Failed(Failure::new(Text::Interrupted, None, None)));
        session.apply(Event::Progress {
            done: 100,
            total: 100,
        });
        assert!(!session.is_busy());
        assert_eq!(
            session.error(Language::Portuguese),
            Some(Language::Portuguese.text(Text::Interrupted))
        );
    }

    #[test]
    fn cancellation_request_is_not_a_completed_cancellation() {
        let mut session = Session::default();
        session.start();
        session.cancel();
        assert!(session.is_busy());
        assert_eq!(session.status(Language::Portuguese), "CANCELANDO");
        session.apply(Event::Cancelled);
        assert!(!session.is_busy());
        assert_eq!(session.error(Language::Portuguese), None);
    }
}
