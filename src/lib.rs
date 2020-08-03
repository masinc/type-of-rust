#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Type {
    full_name: String,
}

impl Type {
    pub fn from<T>(_: &T) -> Self {
        let type_name = std::any::type_name::<T>();
        Type::from_string(type_name)
    }

    fn from_string(s: impl Into<String>) -> Self {
        Type {
            full_name: s.into(),
        }
    }

    pub fn name(&self) -> &str {
        const DOUBLE_COLLON: &str = "::";
        let s: &str = &self.full_name;

        let end = s.find("<").unwrap_or(s.len());
        let s = &s[..(end)];

        let start = s
            .rfind(DOUBLE_COLLON)
            .map(|i| i + DOUBLE_COLLON.len())
            .unwrap_or(0);

        &s[(start)..]
    }

    pub fn full_name(&self) -> &str {
        self.full_name.as_str()
    }

    pub fn generics(&self) -> Option<Vec<Type>> {
        let s: &str = &self.full_name;

        let start = s.find("<");
        let end = s.rfind(">");
        match (start, end) {
            (Some(start), Some(end)) => {
                let s = &s[(start + 1)..(end)];
                let v = s
                    .split(",")
                    .map(|s| s.trim())
                    .map(Type::from_string)
                    .collect();
                Some(v)
            }
            (_, _) => None,
        }
    }
}

use std::fmt;
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.full_name)
    }
}

impl From<Type> for String {
    fn from(t: Type) -> Self {
        t.full_name
    }
}

impl<'a> From<&'a Type> for &'a str {
    fn from(t: &'a Type) -> &'a str {
        t.full_name.as_str()
    }
}

pub trait TypeOf {
    fn type_of(self) -> Type;
}

impl<T> TypeOf for &T
where
    T: Sized,
{
    fn type_of(self) -> Type {
        Type::from(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name;

    macro_rules! test_val {
        ($t:ty, $v:expr) => {
            let a: $t = $v;
            let t = a.type_of();
            assert_eq!(stringify!($t), t.name());
            assert_eq!(type_name::<$t>(), t.full_name());
            assert_eq!(None, t.generics())
        };

        ($st:ty, $lt:path, $v:expr) => {
            let a: $st = $v;
            let t = a.type_of();
            assert_eq!(stringify!($st), t.name());
            assert_eq!(stringify!($lt), t.full_name());
            assert_eq!(None, t.generics())
        };
    }

    #[test]
    fn test_int() {
        //i*
        test_val!(i8, 1);
        test_val!(i16, 1);
        test_val!(i32, 1);
        test_val!(i64, 1);
        test_val!(i128, 1);
        //u*
        test_val!(u8, 1);
        test_val!(u16, 1);
        test_val!(u32, 1);
        test_val!(u64, 1);
        test_val!(u128, 1);
        //*size
        test_val!(isize, 1);
        test_val!(usize, 1);
    }

    #[test]
    fn test_double() {
        test_val!(f32, 1.0);
        test_val!(f64, 1.0);
    }

    #[test]
    fn test_str() {
        test_val!(&str, "");
    }

    #[test]
    fn test_string() {
        test_val!(String, alloc::string::String, String::from(""));
    }

    #[test]
    fn test_vec() {
        let v: Vec<i32> = vec![];
        let t = v.type_of();
        assert_eq!("Vec", t.name());
        assert_eq!("alloc::vec::Vec<i32>", t.full_name());
        assert_eq!(Some(vec![Type::from(&0i32)]), t.generics());
    }

    #[test]
    fn test_hashmap() {
        use std::collections::HashMap;
        let m: HashMap<String, Vec<bool>> = HashMap::new();
        let t = m.type_of();
        assert_eq!("HashMap", t.name());
        assert_eq!(
            "std::collections::hash::map::HashMap<alloc::string::String, alloc::vec::Vec<bool>>",
            t.full_name()
        );

        let g = t.generics().unwrap();

        // String
        let s = g.get(0).unwrap();
        assert_eq!("String", s.name());
        assert_eq!("alloc::string::String", s.full_name());
        assert_eq!(None, s.generics());

        // Vec<bool>
        let v = g.get(1).unwrap();
        assert_eq!("Vec", v.name());
        assert_eq!("alloc::vec::Vec<bool>", v.full_name());
        assert_eq!(Some(vec![Type::from(&true)]), v.generics());
    }

    #[test]
    fn test_option() {
        let a: Option<i32> = None;
        let t = a.type_of();

        assert_eq!("Option", t.name());
        assert_eq!("core::option::Option<i32>", t.full_name());
        assert_eq!(Some(vec![Type::from(&0i32)]), t.generics());
    }

    #[test]
    fn test_result() {
        use std::io::Error;
        let a: Result<(), Error> = Result::Ok(());

        let t = a.type_of();
        assert_eq!("Result", t.name());
        assert_eq!(
            "core::result::Result<(), std::io::error::Error>",
            t.full_name()
        );

        let g = t.generics().unwrap();

        //unit
        let u = g.get(0).unwrap();
        assert_eq!("()", u.name());
        assert_eq!("()", u.full_name());
        assert_eq!(None, u.generics());
    }
}
