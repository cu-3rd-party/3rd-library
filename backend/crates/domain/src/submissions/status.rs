use std::fmt;

use crate::DomainError;

/// Статус заявки: `draft` → `pending_review` → `approved` | `rejected` (→ правка → снова `pending_review`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubmissionStatus {
    Draft,
    PendingReview,
    Approved,
    Rejected,
}

impl SubmissionStatus {
    pub const ALL: [Self; 4] = [Self::Draft, Self::PendingReview, Self::Approved, Self::Rejected];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::PendingReview => "pending_review",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
        }
    }

    pub fn parse(value: &str) -> Result<Self, DomainError> {
        Self::ALL
            .into_iter()
            .find(|status| status.as_str() == value)
            .ok_or_else(|| DomainError::validation("status", format!("unknown submission status '{value}'")))
    }
}

impl fmt::Display for SubmissionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_roundtrip() {
        for status in SubmissionStatus::ALL {
            assert_eq!(SubmissionStatus::parse(status.as_str()).unwrap(), status);
        }
        assert!(SubmissionStatus::parse("published").is_err());
    }
}
