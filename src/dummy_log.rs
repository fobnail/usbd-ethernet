#![allow(unused_macros)]

macro_rules! trace {
    ($($x:tt),*) => {{
        $(let _ = $x;)*
    }};
}

macro_rules! debug {
    ($($x:tt),*) => {{
        $(let _ = $x;)*
    }};
}

macro_rules! info {
    ($($x:tt),*) => {{
        $(let _ = $x;)*
    }};
}

macro_rules! error {
    ($($x:tt),*) => {{
        $(let _ = $x;)*
    }};
}
