pub const LABEL_ALPHABET: &str = "asdfghjklqwertyuiopzxcvbnm";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabeledJumpTarget {
    pub label: char,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LabeledJumpState {
    pub query: String,
    pub targets: Vec<LabeledJumpTarget>,
}

impl LabeledJumpState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn target_for_label(&self, label: char) -> Option<&LabeledJumpTarget> {
        self.targets.iter().find(|target| target.label == label)
    }
}

pub fn collect_visible_targets(
    lines: &[String],
    viewport_offset: usize,
    visible_rows: usize,
    query: &str,
) -> Vec<LabeledJumpTarget> {
    if query.is_empty() || visible_rows == 0 {
        return Vec::new();
    }

    let mut labels = LABEL_ALPHABET.chars();
    let mut targets = Vec::new();
    let end_line = viewport_offset
        .saturating_add(visible_rows)
        .min(lines.len());

    for line_idx in viewport_offset..end_line {
        let Some(line) = lines.get(line_idx) else {
            continue;
        };

        for (byte_idx, _) in line.match_indices(query) {
            let Some(label) = labels.next() else {
                return targets;
            };
            let col = line[..byte_idx].chars().count();
            targets.push(LabeledJumpTarget {
                label,
                line: line_idx,
                col,
            });
        }
    }

    targets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_query_has_no_targets() {
        let lines = vec!["alpha beta".to_string()];

        assert!(collect_visible_targets(&lines, 0, 1, "").is_empty());
    }

    #[test]
    fn collects_visible_targets_with_home_row_labels() {
        let lines = vec![
            "hidden old".to_string(),
            "visible old here".to_string(),
            "old visible too".to_string(),
            "below old".to_string(),
        ];

        let targets = collect_visible_targets(&lines, 1, 2, "old");

        assert_eq!(
            targets,
            vec![
                LabeledJumpTarget {
                    label: 'a',
                    line: 1,
                    col: 8
                },
                LabeledJumpTarget {
                    label: 's',
                    line: 2,
                    col: 0
                },
            ]
        );
    }

    #[test]
    fn caps_targets_to_label_alphabet() {
        let lines = vec!["aa ".repeat(LABEL_ALPHABET.len() + 4)];

        let targets = collect_visible_targets(&lines, 0, 1, "aa");

        assert_eq!(targets.len(), LABEL_ALPHABET.len());
        assert_eq!(targets.first().map(|target| target.label), Some('a'));
        assert_eq!(targets.last().map(|target| target.label), Some('m'));
    }
}
