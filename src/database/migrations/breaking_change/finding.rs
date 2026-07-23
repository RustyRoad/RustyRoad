use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BreakingChangeKind {
    ColumnTypeChange,
    ExplicitCast,
    ConstraintDrop,
}

impl Display for BreakingChangeKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Self::ColumnTypeChange => "column type change",
            Self::ExplicitCast => "explicit data cast",
            Self::ConstraintDrop => "constraint removal",
        };
        formatter.write_str(description)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BreakingChangeFinding {
    pub path: PathBuf,
    pub line: usize,
    pub kind: BreakingChangeKind,
    pub sql: String,
}

impl Display for BreakingChangeFinding {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}:{}: {}: {}",
            self.path.display(),
            self.line,
            self.kind,
            self.sql
        )
    }
}
