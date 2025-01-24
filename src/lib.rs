use std::fmt::Debug;

/// A trait that is implemented for everything that can be a sequence of bytes
pub trait ParseHelper: AsRef<[u8]> {
    /// Skips prefix of slice until sequence is found
    fn take_until(&self, pattern: &[u8]) -> Result<(&[u8], &[u8]), ()> {
        let source = self.as_ref();
        if source.len() < pattern.len() {
            return Err(());
        }
        for i in 0..=source.len() - pattern.len() {
            if source[i..].starts_with(pattern) {
                return Ok((&source[..i], &source[i..]));
            }
        }
        Err(())
    }

    /// Skips prefix of slice until sequence is found and returns provided error if not
    fn take_until_err<E: Debug>(&self, pattern: &[u8], err: E) -> Result<(&[u8], &[u8]), E> {
        self.take_until(pattern).map_err(|_| err)
    }

    /// Returns a slice of exact length
    fn take_exact(&self, count: usize) -> Result<(&[u8], &[u8]), ()> {
        let source = self.as_ref();
        if source.len() < count {
            return Err(());
        }
        Ok((&source[..count], &source[count..]))
    }

    /// Returns a slice of exact length and returns provided error if not
    fn take_exact_err<E: Debug>(&self, count: usize, err: E) -> Result<(&[u8], &[u8]), E> {
        self.take_exact(count).map_err(|_| err)
    }

    /// Returns a slice of the provided pattern and the rest of the slice
    fn take_expect(&self, pattern: &[u8]) -> Result<(&[u8], &[u8]), &[u8]> {
        let source = self.as_ref();
        if source.len() < pattern.len() {
            return Err(source);
        }
        for i in 0..pattern.len() {
            if source[i] != pattern[i] {
                return Err(source);
            }
        }
        Ok((&source[..pattern.len()], &source[pattern.len()..]))
    }

    /// Returns a slice of the provided pattern and the rest of the slice and returns provided error if not
    fn take_expect_err<E: Debug>(&self, pattern: &[u8], err: E) -> Result<(&[u8], &[u8]), E> {
        self.take_expect(pattern).map_err(|_| err)
    }

    /// If the next pattern is optional, it may return it, otherwise it returns the original slice
    fn maybe_expect(&self, pattern: &[u8]) -> (Option<&[u8]>, &[u8]) {
        match self.take_expect(pattern) {
            Ok((first, second)) => (Some(first), second),
            Err(_) => (None, self.as_ref())
        }
    }

    /// Returns the smallest first slice found from the start that matches the condition
    /// i.e. it runs the function until the first time it is true
    fn take_smallest_err<E: Debug, F: Fn(&[u8]) -> bool>(&self, f: F, min_size: usize, err: E) -> Result<(&[u8], &[u8]), E> {
        for i in min_size..self.as_ref().len() {
            if f(&self.as_ref()[..i]) {
                return Ok((&self.as_ref()[..i], &self.as_ref()[i..]))
            }
        }
        Err(err)
    }

    /// Returns the largest slice found from the start that matches the condition.
    /// i.e. it runs the function until the last time it is true
    fn take_largest_err<E: Debug, F: Fn(&[u8]) -> bool>(&self, f: F, min_size:usize, err: E) -> Result<(&[u8], &[u8]), E> {
        let mut largest = None;
        for i in min_size..self.as_ref().len() {
            if f(&self.as_ref()[..i]) {
                largest = Some(i);
            }
        };
        largest.map(|i| (&self.as_ref()[..i], &self.as_ref()[i..])).ok_or(err)
    }
}

impl ParseHelper for &[u8] {}
impl ParseHelper for [u8] {}
impl ParseHelper for Vec<u8> {}
impl ParseHelper for &str {}

#[cfg(test)]
mod test {
    use crate::ParseHelper;

    #[test]
    fn test_take_until() {
        let source = b"hello world";
        let pattern = b" ";
        let (before, after) = source.take_until(pattern).unwrap();
        assert_eq!(before, b"hello");
        assert_eq!(after, b" world");

        let (before,after) = "GET / HTTP/1.1\r\n\r\n".take_until(b"\r\n\r\n").unwrap();
        assert_eq!(before, b"GET / HTTP/1.1");
        assert_eq!(after, b"\r\n\r\n");
    }

    #[test]
    fn take_exact() {
        let source = b"hello world";
        let (exact, after) = source.take_exact(5).unwrap();
        assert_eq!(exact, b"hello");
        assert_eq!(after, b" world");
    }

    #[test]
    fn take_expect() {
        let source = b"hello world";
        let (matching, remained) = source.take_expect(b"hello ").unwrap();
        assert_eq!(matching, b"hello ");
        assert_eq!(remained, b"world");
    }

    #[test]
    fn take_smallest() {
        let source = b"aaaabbbbcccc";
        assert!(source.take_smallest_err(|s| s.starts_with(b"bbbb"), 0, ()).is_err());

        let (first ,second) = source.take_smallest_err(|s| !s.contains(&b"b"[0]), 0, ()).unwrap();
        assert_eq!(first, b"a");
        assert_eq!(second, b"aaabbbbcccc");
    }

    #[test]
    fn take_largest() {
        let source = b"aaaabbbbcccc";

        let func = |s: &[u8]| s.starts_with(b"b");
        assert!(source.take_largest_err(func, 0, ()).is_err());

        let func = |s: &[u8]| !s.contains(&b"b"[0]);
        let (first, second) = source.take_largest_err(func, 0, ()).unwrap();
        assert_eq!(first, b"aaaa");
        assert_eq!(second, b"bbbbcccc");
    }
}
