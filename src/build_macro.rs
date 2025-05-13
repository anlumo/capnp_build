/// Create a new message builder and populate it with the given data.
///
/// This is the single entry point for the crate's functionality.
#[macro_export]
macro_rules! capnp_build {
    // Empty message
    ($root_builder:path) => {{
        let mut message = capnp::message::Builder::new_default();
        message.init_root::<$root_builder>();

        message
    }};
    ($root_builder:path, {
            $($body:tt)*
    }) => {{
        let mut message = capnp::message::Builder::new_default();
        #[allow(unused)]
        let mut builder = message.init_root::<$root_builder>();

        #[allow(unused_assignments)]
        {
            $crate::capnp_build_fields!(builder, { $($body)* });
        }
        message
    }};
}

// Entry for primitive and structured fields
#[doc(hidden)]
#[macro_export]
macro_rules! capnp_build_fields {
    // Struct nested inline
    ($builder:ident, { $field:ident = { $($inner:tt)* }, $($rest:tt)* }) => {{
        paste::paste! {
            #[allow(unused, unused_mut)]
            let mut nested = $builder.reborrow().[<init_ $field>]();
        }
        $crate::capnp_build_fields!(nested, { $($inner)* });
        $crate::capnp_build_fields!($builder, { $($rest)* });
    }};

    // Struct nested inline (no trailing comma)
    ($builder:ident, { $field:ident = { $($inner:tt)* } }) => {{
        paste::paste! {
            #[allow(unused, unused_mut)]
            let mut nested = $builder.reborrow().[<init_ $field>]();
        }
        $crate::capnp_build_fields!(nested, { $($inner)* });
    }};

    // List of structs
    ($builder:ident, { $field:ident = [ $({ $($item:tt)* }),* $(,)? ], $($rest:tt)* }) => {{
        paste::paste! {
            let mut list = $builder.reborrow().[<init_ $field>]($crate::count_exprs!($( { $($item)*} ),*));
        }
        let mut i = 0;
        $(
            #[allow(unused_mut)]
            let mut elem = list.reborrow().get(i);
            $crate::capnp_build_fields!(elem, { $($item)* });
            i += 1;
        )*
        $crate::capnp_build_fields!($builder, { $($rest)* });
    }};
    // List of structs (no trailing comma)
    ($builder:ident, { $field:ident = [ $({ $($item:tt)* }),* $(,)? ] }) => {{
        paste::paste! {
            let mut list = $builder.reborrow().[<init_ $field>]($crate::count_exprs!($( { $($item)*} ),*));
        }
        let mut i = 0;
        $(
            #[allow(unused_mut)]
            let mut elem = list.reborrow().get(i);
            $crate::capnp_build_fields!(elem, { $($item)* });
            i += 1;
        )*
    }};

    // List of primitives
    ($builder:ident, { $field:ident = [ $($val:expr),* $(,)? ], $($rest:tt)* }) => {{
        paste::paste! {
            let mut list = $builder.reborrow().[<init_ $field>]($crate::count_exprs!($($val),*));
        }
        let mut i = 0;
        $(
            list.set(i, $val);
            i += 1;
        )*
        $crate::capnp_build_fields!($builder, { $($rest)* });
    }};
    // List of primitives (no trailing comma)
    ($builder:ident, { $field:ident = [ $($val:expr),* $(,)? ] }) => {{
        paste::paste! {
            let mut list = $builder.reborrow().[<init_ $field>]($crate::count_exprs!($($val),*));
        }
        let mut i = 0;
        $(
            list.set(i, $val);
            i += 1;
        )*
    }};

    // Function call, passing in the builder
    ($builder:ident, { $field:ident [ $func:ident ] = $val:expr, $($rest:tt)* }) => {{
        paste::paste! {
            $func($val, $builder.reborrow().[<init_ $field>]());
        }
        $crate::capnp_build_fields!($builder, { $($rest)* });
    }};
    // Function call, passing in the builder (no trailing comma)
    ($builder:ident, { $field:ident [ $func:ident ] = $val:expr }) => {{
        paste::paste! {
            $func($val, $builder.[<init_ $field>]());
        }
    }};

    // Primitives or Enums or Void
    ($builder:ident, { $field:ident = $val:expr, $($rest:tt)* }) => {{
        paste::paste! {
            $builder.[<set_ $field>]($val);
        }
        $crate::capnp_build_fields!($builder, { $($rest)* });
    }};
    // Primitives or Enums or Void (no trailing comma)
    ($builder:ident, { $field:ident = $val:expr }) => {{
        paste::paste! {
            $builder.[<set_ $field>]($val);
        }
    }};

    // End
    ($builder:ident, {}) => {};
}

// Helper for counting list elements
#[doc(hidden)]
#[macro_export]
macro_rules! count_exprs {
    () => (0u32);
    ($_head:tt $(, $_tail:tt)*) => (1u32 + $crate::count_exprs!($($_tail),*));
}
