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


pub mod bicycle {
    use std::fmt;

    pub struct Bicycle {
        type_name: String,
        wheel_size: u32,
        color: String,
        electric: bool,
    }

    impl Bicycle {
        pub fn new(type_name: &str, wheel_size: u32, color: &str, electric: bool) -> Bicycle {
            Bicycle {
                type_name: type_name.to_string(),
                wheel_size,
                color: color.to_string(),
                electric,
            }
        }

        pub fn get_type(&self) -> &str {
            &self.type_name
        }
    }

    // implement Display trait
    impl fmt::Display for Bicycle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} Bicycle ({}), Color: {}, Electric: {}", 
                   self.type_name, self.wheel_size, self.color, self.electric)
        }
    }

    // implement Debug trait
    impl fmt::Debug for Bicycle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Bicycle")
             .field("type_name", &self.type_name)
             .field("wheel_size", &self.wheel_size)
             .field("color", &self.color)
             .field("electric", &self.electric)
             .finish()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn create_bicycle() {
            let bike = Bicycle::new("Mountain", 29, "Red", false);
            assert_eq!(bike.type_name, "Mountain");
            assert_eq!(bike.wheel_size, 29);
            assert_eq!(bike.color, "Red");
            assert_eq!(bike.electric, false);
        }

        #[test]
        fn bicycle_display() {
            let bike = Bicycle::new("Road", 28, "Blue", false);
            assert_eq!(format!("{}", bike), "Road Bicycle (28), Color: Blue, Electric: false");
        }

        #[test]
        fn bicycle_debug() {
            let bike = Bicycle::new("Electric", 26, "Black", true);
            let debug_string = format!("{:?}", bike);
            assert!(debug_string.contains("Electric"));
            assert!(debug_string.contains("26"));
            assert!(debug_string.contains("Black"));
            assert!(debug_string.contains("true"));
        }
    }

}
