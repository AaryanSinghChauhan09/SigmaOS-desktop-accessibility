// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-desktop-accessibility: desktop accessibility component model.
//!
//! Real implementation: accessibilitys, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Accessibility {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesktopAccessibilityError {
    EmptyAccessibilityName,
    DuplicateAccessibility(String),
    EmptyEventSubject,
}

impl fmt::Display for DesktopAccessibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAccessibilityName => write!(f, "accessibility name must not be empty"),
            Self::DuplicateAccessibility(id) => write!(f, "duplicate accessibility {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DesktopAccessibilityError {}

pub struct DesktopAccessibility {
    pub name: String,
    accessibilitys: BTreeMap<String, Accessibility>,
    events: BTreeMap<String, Event>,
}

impl DesktopAccessibility {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            accessibilitys: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_accessibility(&mut self, item: Accessibility) -> Result<(), DesktopAccessibilityError> {
        if item.name.trim().is_empty() {
            return Err(DesktopAccessibilityError::EmptyAccessibilityName);
        }
        let id = item.id.clone();
        if self.accessibilitys.contains_key(&id) {
            return Err(DesktopAccessibilityError::DuplicateAccessibility(id.clone()));
        }
        self.accessibilitys.insert(id, item);
        Ok(())
    }

    pub fn accessibility_count(&self) -> usize {
        self.accessibilitys.len()
    }

    pub fn get_accessibility(&self, id: &str) -> Option<&Accessibility> {
        self.accessibilitys.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DesktopAccessibilityError> {
        if event.subject.trim().is_empty() {
            return Err(DesktopAccessibilityError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_accessibility(id: &str, name: &str) -> Accessibility {
        Accessibility {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DesktopAccessibility::new("DesktopAccessibility");
        assert_eq!(m.name, "DesktopAccessibility");
        assert_eq!(m.accessibility_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_accessibility() {
        let mut m = DesktopAccessibility::new("DesktopAccessibility");
        m.add_accessibility(sample_accessibility("a1", "Alpha")).unwrap();
        assert_eq!(m.accessibility_count(), 1);
        assert!(m.get_accessibility("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DesktopAccessibility::new("DesktopAccessibility");
        assert!(m.add_accessibility(sample_accessibility("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DesktopAccessibility::new("DesktopAccessibility");
        m.add_accessibility(sample_accessibility("x", "One")).unwrap();
        assert!(m.add_accessibility(sample_accessibility("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DesktopAccessibility::new("DesktopAccessibility");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
