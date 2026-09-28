/// Error payload used by the `Result`-typed sandbox fixture fields.
pub const SANDBOX_ERR_TEXT: &str = "error";

/// Username assigned to the sandbox `User` fixture that carries an email.
pub const SANDBOX_USER_NAME: &str = "Alice";

/// Password that `#[debug(skip)]` must keep out of the formatted output.
pub const SANDBOX_PASSWORD: &str = "secret123";

/// Email address carried by the sandbox `User` fixture.
pub const SANDBOX_EMAIL: &str = "alice@ltpp.vip";

/// First element of the sandbox `Vec<String>` fixtures.
pub const SANDBOX_HELLO: &str = "hello";

/// Second element of the sandbox `Vec<String>` fixtures.
pub const SANDBOX_WORLD: &str = "world";

/// Value written through a generated setter to prove mutation works.
pub const SANDBOX_UPDATED: &str = "updated";

/// Data payload of the sandbox `Response::Success` variant.
pub const SANDBOX_SUCCESS_TEXT: &str = "Operation completed";

/// Message payload of the sandbox `Response::Error` variant.
pub const SANDBOX_FAILURE_TEXT: &str = "Something went wrong";

/// Generic name of the sandbox `User` fixture exercising `String` parameters.
pub const SANDBOX_TEST_NAME: &str = "test";

/// Username of the sandbox `User` fixture built through the `New` derive.
pub const SANDBOX_USERNAME: &str = "alice";

/// Product name of the sandbox `Product` fixture.
pub const SANDBOX_PRODUCT_NAME: &str = "Laptop";

/// Name of the sandbox `PrivatePerson` fixture with narrowed visibility.
pub const SANDBOX_PRIVATE_NAME: &str = "Charlie";

/// First element of the sandbox `Into<Vec<String>>` setter fixture.
pub const SANDBOX_ITEM_ONE: &str = "item1";

/// Second element of the sandbox `Into<Vec<String>>` setter fixture.
pub const SANDBOX_ITEM_TWO: &str = "item2";

/// Name written through the sandbox `AsRef<str>` setter fixture.
pub const SANDBOX_RENAMED: &str = "new name";

/// First element of the replacement `Vec<String>` pushed into the fixture.
pub const SANDBOX_NEW_ITEM_ONE: &str = "new1";

/// Second element of the replacement `Vec<String>` pushed into the fixture.
pub const SANDBOX_NEW_ITEM_TWO: &str = "new2";

/// Name of the sandbox nested-struct fixture.
pub const SANDBOX_NESTED_NAME: &str = "inner";

/// Key of the sandbox metadata map fixture.
pub const SANDBOX_MAP_KEY: &str = "key";

/// Value of the sandbox metadata map fixture.
pub const SANDBOX_MAP_VALUE: &str = "value";

/// Name of the sandbox enum struct-variant fixture.
pub const SANDBOX_VISIBLE: &str = "visible";

/// Field that `#[debug(skip)]` must keep out of the enum debug output.
pub const SANDBOX_HIDDEN: &str = "hidden";

/// Borrowed name of the sandbox lifetimes fixture.
pub const SANDBOX_LIFETIME_NAME: &str = "rust";

/// Borrowed description of the sandbox lifetimes fixture.
pub const SANDBOX_LIFETIME_DESCRIPTION: &str = "language";

/// First element of the replacement vector pushed through the `Into<Vec<String>>` setter.
pub const SANDBOX_NEW_VALUE: &str = "new";

/// Second element of the replacement vector pushed through the `Into<Vec<String>>` setter.
pub const SANDBOX_VALUES: &str = "values";
