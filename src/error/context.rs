use core::any::Any;

use crate::fmt;
use crate::input::Input;

use super::WithContext;

/// Information surrounding an error.
pub trait Context: Any {
    /// The operation that was attempted when an error occurred.
    ///
    /// It should described in a simple manner what is trying to be achieved and
    /// make sense in the following sentence if you were to substitute it:
    ///
    /// ```text
    /// error attempting to <operation>.
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`fmt::Error`] if failed to write to the formatter.
    fn operation(&self, w: &mut dyn fmt::Write) -> fmt::Result;

    /// Returns `true` if there is an expected value.
    fn has_expected(&self) -> bool {
        false
    }

    /// The expected value.
    ///
    /// # Errors
    ///
    /// Returns a [`fmt::Error`] if failed to write to the formatter.
    fn expected(&self, _w: &mut dyn fmt::Write) -> fmt::Result {
        Err(fmt::Error)
    }

    /// Return a reference of self as [`Any`].
    fn as_any(&self) -> &dyn Any;
}

///////////////////////////////////////////////////////////////////////////////
// Basic expected context

impl Context for &'static str {
    fn operation(&self, w: &mut dyn fmt::Write) -> fmt::Result {
        w.write_str("context")
    }

    fn has_expected(&self) -> bool {
        true
    }

    fn expected(&self, w: &mut dyn fmt::Write) -> fmt::Result {
        w.write_str(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

///////////////////////////////////////////////////////////////////////////////
// Expected context

/// A [`Context`] with an expected value and operation name.
///
/// # Example
///
/// ```nocompile
/// ExpectedContext {
///   operation: "my operation",
///   expected: "value",
/// }
/// ```
#[derive(Copy, Clone)]
pub struct ExpectedContext {
    /// Value for [`Context::operation()`].
    pub operation: &'static str,
    /// Value for [`Context::expected()`].
    pub expected: &'static str,
}

impl Context for ExpectedContext {
    fn operation(&self, w: &mut dyn fmt::Write) -> fmt::Result {
        w.write_str(self.operation)
    }

    fn has_expected(&self) -> bool {
        !self.expected.is_empty()
    }

    fn expected(&self, w: &mut dyn fmt::Write) -> fmt::Result {
        w.write_str(self.expected)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Debug for ExpectedContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExpectedContext")
            .field("operation", &self.operation)
            .field("expected", &self.expected)
            .finish()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Operation context

/// An operation [`Context`].
///
/// # Example
///
/// ```nocompile
/// OperationContext("my operation")
/// ```
#[derive(Copy, Clone)]
pub struct OperationContext(
    /// Value for [`Context::operation()`].
    pub &'static str,
);

impl Context for OperationContext {
    fn operation(&self, w: &mut dyn fmt::Write) -> fmt::Result {
        w.write_str(self.0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Debug for OperationContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OperationContext").field(&self.0).finish()
    }
}

///////////////////////////////////////////////////////////////////////////////

#[inline(always)]
pub(crate) fn with_context<'i, F, T, E>(
    input: impl Input<'i>,
    context: impl Context,
    f: F,
) -> Result<T, E>
where
    E: WithContext<'i>,
    F: FnOnce() -> Result<T, E>,
{
    match f() {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err.with_context(input, context)),
    }
}
