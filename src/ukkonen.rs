/// Return True if `a` and `b` differ by at most `k` edits
#[allow(non_snake_case)]
pub fn ukkonen<'a>(a: &'a str, b: &'a str, k: isize) -> isize {
    if a == b {
        return 0;
    }

    let mut achars: Vec<char> = a.chars().collect();
    let mut bchars: Vec<char> = b.chars().collect();

    let mut a_size = achars.len() as isize;
    let mut b_size = bchars.len() as isize;

    if a_size > b_size {
        std::mem::swap(&mut achars, &mut bchars);
        std::mem::swap(&mut a_size, &mut b_size);
    }

    while a_size > 0 && achars[(a_size - 1) as usize] == bchars[(b_size - 1) as usize] {
        a_size -= 1;
        b_size -= 1;
    }

    // remove the suffix
    achars.drain(a_size as usize..achars.len());
    bchars.drain(b_size as usize..bchars.len());

    // if the shorter string is gone, return b_size or threshold
    if a_size == 0 {
        if b_size < k {
            return b_size;
        } else {
            return k;
        }
    }

    // Drop shared prefix
    let mut start = 0;

    while start < a_size as usize && achars[start] == bchars[start] {
        start += 1;
    }

    if start != 0 {
        achars.drain(0..start);
        bchars.drain(0..start);
        a_size -= start as isize;
        b_size -= start as isize;
    }

    // if the shorter string is gone, return b_size or threshold
    if a_size == 0 {
        if b_size < k {
            return b_size;
        } else {
            return k;
        }
    }

    let k = std::cmp::min(b_size, k);

    let size_d = b_size - a_size;

    if size_d > k {
        return k;
    }

    let ZERO_K = std::cmp::min(k, a_size) / 2 + 2;
    let array_size = size_d + ZERO_K * 2 + 2;

    let mut current_row = vec![-1; array_size as usize];
    let mut next_row = vec![-1; array_size as usize];

    let mut i = 0isize;
    let condition_row = size_d + ZERO_K;
    let end_max = condition_row * 2;

    loop {
        i += 1;

        std::mem::swap(&mut current_row, &mut next_row);

        let start;
        let mut previous_cell;
        let mut current_cell = -1;
        let mut next_cell;

        if i <= ZERO_K {
            start = -i + 1;
            next_cell = i - 2;
        } else {
            start = i - ZERO_K * 2 + 1;
            next_cell = current_row[(ZERO_K + start) as usize];
        }

        let end;
        if i <= condition_row {
            end = i;
            next_row[(ZERO_K + i) as usize] = -1;
        } else {
            end = end_max - i;
        }

        for q in start..end {
            let row_index = q + ZERO_K;

            previous_cell = current_cell;
            current_cell = next_cell;
            next_cell = current_row[(row_index + 1) as usize];

            let mut t = std::cmp::max(
                std::cmp::max(current_cell + 1, previous_cell),
                next_cell + 1,
            );

            while t < a_size && t + q < b_size && achars[t as usize] == bchars[(t + q) as usize] {
                t += 1;
            }

            next_row[row_index as usize] = t;
        }
        if !(next_row[condition_row as usize] < a_size && i <= k) {
            break;
        }
    }

    return i - 1;
}

#[cfg(test)]
mod tests {
    use super::ukkonen;
    #[test]
    fn simple() {
        assert_eq!(ukkonen("test", "testttt", 8), 3);
    }

    #[test]
    fn hellow_large_k() {
        assert_eq!(ukkonen("hello", "world", 5), 4);
    }

    #[test]
    fn hellow_small_k() {
        assert_eq!(ukkonen("hello", "world", 2), 2);
    }
}
