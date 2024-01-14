// "file not included in module tree"

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = exploring::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


// #[attribute]  begin
// used to attach metadata to code.
// a declarative way to add metadata, control compiler behavior, customize code generation, and write documentation


// built-in: derive, cfg, test, bench...
// cfg conditional compilation
// eg [cfg(target_os= "linux")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}


#[allow(unused_variables)]
pub fn bob() -> i32 {
    let x = 42;

    12
}

// #[attribute]  end

