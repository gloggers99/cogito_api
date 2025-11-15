// It's important to not leak specific information like "invalid username" or "wrong password" to
// avoid user enumeration attacks. That is why these return the same string.

/// The message returned by the API when the password is incorrect.
pub static WRONG_PASSWORD: &'static str = "Invalid credentials.";

/// The message returned by the API when the user is not found.
pub static USER_NOT_FOUND: &'static str = WRONG_PASSWORD;

/// The message returned by the API when a database error occurs.
pub static DATABASE_ERROR: &'static str = "Database error.";

/// The message returned by the API when an internal server error occurs.
pub static SERVER_ERROR: &'static str = "Internal server error.";

/// The message returned by the API when the session is invalid.
pub static BAD_SESSION: &'static str = "Invalid session. Please login again.";
