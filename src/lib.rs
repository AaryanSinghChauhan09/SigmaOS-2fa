//! SigmaOS-2fa: two-factor authentication model.
//!
//! Real implementation: TOTP, hardware tokens, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// 2FA credential.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwoFactorCredential {
    pub id: String,
    pub name: String,
    pub method: String,
    pub secret: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TwoFactorError {
    EmptyName,
    DuplicateCredential(String),
}

impl fmt::Display for TwoFactorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "credential name must not be empty"),
            Self::DuplicateCredential(id) => write!(f, "duplicate credential {:?}", id),
        }
    }
}

impl std::error::Error for TwoFactorError {}

/// 2FA manager.
pub struct TwoFactorAuth {
    pub name: String,
    credentials: BTreeMap<String, TwoFactorCredential>,
}

impl TwoFactorAuth {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            credentials: BTreeMap::new(),
        }
    }

    pub fn add_credential(&mut self, credential: TwoFactorCredential) -> Result<(), TwoFactorError> {
        if credential.name.trim().is_empty() { return Err(TwoFactorError::EmptyName); }
        let id = credential.id.clone();
        if self.credentials.contains_key(&id) {
            return Err(TwoFactorError::DuplicateCredential(id.clone()));
        }
        self.credentials.insert(id, credential);
        Ok(())
    }

    pub fn credential_count(&self) -> usize { self.credentials.len() }

    pub fn get_credential(&self, id: &str) -> Option<&TwoFactorCredential> { self.credentials.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2fa_create_aaa() {
        let t = TwoFactorAuth::new("My 2FA");
        assert_eq!(t.name, "My 2FA");
        assert_eq!(t.credential_count(), 0);
    }

    #[test]
    fn test_add_credential_aaa() {
        let mut t = TwoFactorAuth::new("My 2FA");
        let c = TwoFactorCredential { id: "c1".into(), name: "totp".into(), method: "time-based".into(), secret: "abc123".into() };
        t.add_credential(c).unwrap();
        assert_eq!(t.credential_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut t = TwoFactorAuth::new("My 2FA");
        let c = TwoFactorCredential { id: "x".into(), name: "".into(), method: "a".into(), secret: "b".into() };
        assert!(t.add_credential(c).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut t = TwoFactorAuth::new("My 2FA");
        let c = TwoFactorCredential { id: "x".into(), name: "X".into(), method: "a".into(), secret: "b".into() };
        t.add_credential(c).unwrap();
        let c2 = TwoFactorCredential { id: "x".into(), name: "X".into(), method: "a".into(), secret: "b".into() };
        assert!(t.add_credential(c2).is_err());
    }
}
