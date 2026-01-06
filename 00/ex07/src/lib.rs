pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
    if query.is_empty() {
        for n in pattern {
            if *n != 42 {
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::* ;

    #[test]
    fn empty_query() {
        assert!(strpcmp(b"", b"****"));
        assert!(!strpcmp(b"", b"abc"));
    }
}
