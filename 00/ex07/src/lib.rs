pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
    let mut p: usize = 0;
    let mut q: usize = 0;
    let mut t_q: usize = 0;
    let mut t_p: usize = 0;

    while q < query.len() && p < pattern.len() {
        if query[q] == pattern[p] {
            q += 1;
            p += 1;
        } else if pattern[p] == b'*' && p + 1 < pattern.len() && query[q] != pattern[p + 1] {
            q += 1;
            t_p = p;
        } else if pattern[p] == b'*' && p + 1 < pattern.len() && query[q] == pattern[p + 1] {
            p += 1;
            t_q = q;
        } else if pattern[p] == b'*' && p + 1 == pattern.len() {
            return true;
        } else if pattern[p] != query[q] && t_q != 0 {
            q = t_q + 1;
            p = t_p;
        } else {
            break;
        }
    }

    while p < pattern.len() && pattern[p] == b'*' {
        p += 1;
    }

    p == pattern.len() && q == query.len()
}

#[test]
fn some_test() {
    assert!(strpcmp(b"abc", b"abc"));

    assert!(strpcmp(b"abcd", b"ab*"));
    assert!(!strpcmp(b"cab", b"ab*"));

    assert!(strpcmp(b"dcab", b"*ab"));
    assert!(!strpcmp(b"abc", b"*ab"));

    assert!(strpcmp(b"ab000cd", b"ab*cd"));
    assert!(strpcmp(b"abcd", b"ab*cd"));

    assert!(strpcmp(b"", b"****"));

    assert!(strpcmp(b"testte", b"t*te"));
}
