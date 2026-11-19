//! Assembles the text of a dictation from the recognizer's segments.

/// Text recognized so far in a dictation.
///
/// The recognizer splits speech into segments at pauses. Finished segments are
/// committed; the segment being spoken is the partial and may still change.
#[derive(Debug, Default)]
pub(crate) struct Transcript {
    committed: Vec<String>,
    partial: String,
}

impl Transcript {
    /// Replaces the in-progress segment. Returns whether the text changed.
    pub(crate) fn set_partial(&mut self, text: &str) -> bool {
        let text = normalize_whitespace(text);
        if text == self.partial {
            return false;
        }
        self.partial = text;
        true
    }

    /// Moves the in-progress segment into the committed text. Returns whether
    /// there was anything to commit.
    pub(crate) fn commit_partial(&mut self) -> bool {
        if self.partial.is_empty() {
            return false;
        }
        self.committed.push(std::mem::take(&mut self.partial));
        true
    }

    /// The whole transcript, segments separated by a single space.
    pub(crate) fn text(&self) -> String {
        self.committed
            .iter()
            .chain(std::iter::once(&self.partial))
            .filter(|segment| !segment.is_empty())
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Trims and collapses whitespace runs. The recognizer strips language tags
/// from its output, which can leave doubled spaces behind.
fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_committed_segments_and_partial() {
        let mut transcript = Transcript::default();
        transcript.set_partial(" Hello there. ");
        transcript.commit_partial();
        transcript.set_partial("How are");

        assert_eq!(transcript.text(), "Hello there. How are");
    }

    #[test]
    fn reports_only_real_changes() {
        let mut transcript = Transcript::default();

        assert!(transcript.set_partial("Ahoy"));
        assert!(!transcript.set_partial("Ahoy "));
        assert!(transcript.commit_partial());
        assert!(!transcript.commit_partial());
    }

    #[test]
    fn collapses_whitespace_left_by_removed_tags() {
        let mut transcript = Transcript::default();
        transcript.set_partial("Tudo bem.  Eu  queria");

        assert_eq!(transcript.text(), "Tudo bem. Eu queria");
    }

    #[test]
    fn ignores_empty_segments() {
        let mut transcript = Transcript::default();
        transcript.set_partial("Ahoy");
        transcript.commit_partial();
        transcript.set_partial("   ");

        assert_eq!(transcript.text(), "Ahoy");
    }
}
