use crate::step::BackToken;

/// Creates a [`BackToken`] that enables `InputValidation::Back` in form steps.
///
/// This function is the only way to construct a `BackToken`.
/// Import this from your process runner module — not from form implementations.
#[must_use]
pub fn create_back_token() -> BackToken {
  BackToken(()) // fixme maybe hide behind a feature flag
}
