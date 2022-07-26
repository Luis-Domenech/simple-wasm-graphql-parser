// Macro for simulating ternary operation conditional
// In TS we can do const isArray = Array.isArray(variable) ? true : false
// This simulates that somewhat
#[macro_export]
macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        }
        else {
            $false_expr
        }
    }
}

// pub (crate) use either;
pub use either;