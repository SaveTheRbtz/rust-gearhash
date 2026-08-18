use crate::Table;

#[inline]
pub(crate) fn next_match(hash: &mut u64, table: &Table, buf: &[u8], mask: u64) -> Option<usize> {
    let mut current_hash = *hash;
    let mut i = 0;
    // Preload independent table values so their lookups can overlap before the
    // ordered hash dependency chain is evaluated.
    while i + 8 <= buf.len() {
        let first = table[buf[i] as usize];
        let second = table[buf[i + 1] as usize];
        let third = table[buf[i + 2] as usize];
        let fourth = table[buf[i + 3] as usize];
        let fifth = table[buf[i + 4] as usize];
        let sixth = table[buf[i + 5] as usize];
        let seventh = table[buf[i + 6] as usize];
        let eighth = table[buf[i + 7] as usize];

        current_hash = (current_hash << 1).wrapping_add(first);
        if current_hash & mask == 0 {
            *hash = current_hash;
            return Some(i + 1);
        }
        current_hash = (current_hash << 1).wrapping_add(second);
        if current_hash & mask == 0 {
            *hash = current_hash;
            return Some(i + 2);
        }
        current_hash = (current_hash << 1).wrapping_add(third);
        if current_hash & mask == 0 {
            *hash = current_hash;
            return Some(i + 3);
        }
        current_hash = (current_hash << 1).wrapping_add(fourth);
        if current_hash & mask == 0 {
            *hash = current_hash;
            return Some(i + 4);
        }
        current_hash = (current_hash << 1).wrapping_add(fifth);
        if current_hash & mask == 0 {
            *hash = current_hash;
            return Some(i + 5);
        }
        current_hash = (current_hash << 1).wrapping_add(sixth);
        if current_hash & mask == 0 {
            *hash = current_hash;
            return Some(i + 6);
        }
        current_hash = (current_hash << 1).wrapping_add(seventh);
        if current_hash & mask == 0 {
            *hash = current_hash;
            return Some(i + 7);
        }
        current_hash = (current_hash << 1).wrapping_add(eighth);
        if current_hash & mask == 0 {
            *hash = current_hash;
            return Some(i + 8);
        }

        i += 8;
    }

    for (tail, b) in buf[i..].iter().enumerate() {
        current_hash = (current_hash << 1).wrapping_add(table[*b as usize]);

        if current_hash & mask == 0 {
            *hash = current_hash;
            return Some(i + tail + 1);
        }
    }

    *hash = current_hash;
    None
}

#[cfg(test)]
mod tests {
    use super::next_match;
    use crate::DEFAULT_TABLE;

    quickcheck::quickcheck! {
        fn check_against_reference(start_hash: u64, mask: u64, buf: Vec<u8>) -> bool {
            let mut hash = start_hash;
            let mut reference_hash = start_hash;
            let expected = buf.iter().enumerate().find_map(|(i, b)| {
                reference_hash = (reference_hash << 1)
                    .wrapping_add(DEFAULT_TABLE[*b as usize]);
                if reference_hash & mask == 0 {
                    Some(i + 1)
                } else {
                    None
                }
            });

            let actual = next_match(&mut hash, &DEFAULT_TABLE, &buf, mask);
            actual == expected && hash == reference_hash
        }
    }
}

#[cfg(feature = "bench")]
#[bench]
fn throughput(b: &mut test::Bencher) {
    crate::bench::throughput(b, |hash, buf, mask| {
        next_match(hash, &crate::DEFAULT_TABLE, buf, mask)
    })
}
