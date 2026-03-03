fn kmp_search<T: PartialEq>(haystack: &[T], needle: &[T]) -> Vec<usize> {
    let m = needle.len();
    let mut res = vec![];
    let pi = pi(needle);

    let mut matched = 0;

    for (i, h) in haystack.iter().enumerate() {
        while matched > 0 && *h != needle[matched] {
            matched = pi[matched - 1];
        }
        if *h == needle[matched] {
            matched += 1;
            if matched == m {
                res.push(i + 1 - m);
                matched = pi[matched - 1];
            }
        }
    }
    res
}

fn pi<T: PartialEq>(needle: &[T]) -> Vec<usize> {
    let n = needle.len();
    let mut pi = vec![0; n];

    let (mut begin, mut matched) = (1, 0);

    while begin + matched < n {
        if needle[begin + matched] == needle[matched] {
            matched += 1;
            pi[begin + matched - 1] = matched;
        } else if matched == 0 {
            begin += 1;
        } else {
            begin += matched - pi[matched - 1];
            matched = pi[matched - 1]
        }
    }
    pi
}
