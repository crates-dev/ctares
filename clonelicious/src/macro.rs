/// A helper macro to clone variables into closures or async blocks easily.
///
/// This macro supports several usage patterns:
///
/// - Clone variables into an `async move` block.
/// - Clone variables into a regular block.
/// - Clone variables into an `async move` closure with or without explicit `move`.
/// - Clone variables into a regular closure with or without explicit `move`.
#[macro_export]
macro_rules! clone {
    ( $( $var:ident ),* => async move $body:block ) => {{
        $( let $var = $var.clone(); )*
        async move $body
    }};

    ( $( $var:ident ),* => $body:block ) => {{
        $( let $var = $var.clone(); )*
        $body
    }};

    ( $( $var:ident ),* => move |$( $arg:ident $(: $ty:ty)? ),*| async move $body:block ) => {{
        $( let $var = $var.clone(); )*
        move |$( $arg $(: $ty)? ),*| {
            $( let $var = $var.clone(); )*
            async move $body
        }
    }};

    ( $( $var:ident ),* => |$( $arg:ident $(: $ty:ty)? ),*| async move $body:block ) => {{
        $( let $var = $var.clone(); )*
        move |$( $arg $(: $ty)? ),*| {
            $( let $var = $var.clone(); )*
            async move $body
        }
    }};

    ( $( $var:ident ),* => move |$( $arg:ident $(: $ty:ty)? ),*| $body:block ) => {{
        $( let $var = $var.clone(); )*
        move |$( $arg $(: $ty)? ),*| {
            $( let $var = $var.clone(); )*
            $body
        }
    }};

    ( $( $var:ident ),* => |$( $arg:ident $(: $ty:ty)? ),*| $body:block ) => {{
        $( let $var = $var.clone(); )*
        move |$( $arg $(: $ty)? ),*| {
            $( let $var = $var.clone(); )*
            $body
        }
    }};
}
