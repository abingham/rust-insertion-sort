use std::iter::FromIterator;
use std::iter::Iterator;

pub fn sort(xs: &mut Vec<i32>) {
    for index in 1..xs.len() {
        let current = xs[index];

        let back_index = xs[0..index]
            .iter()
            .enumerate()
            .find_map(|(idx, &x)| if current < x { Some(idx) } else { None })
            .unwrap_or(index);

        // Can we remove this copy?
        let to_shift = Vec::from_iter(xs[back_index..index].iter().cloned());
        xs.splice(back_index + 1..index + 1, to_shift);
        xs[back_index] = current;
    }
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn basic_sort() {
        let mut xs = vec![3, 2, 1];
        sort(&mut xs);
        assert_eq!(xs, vec![1, 2, 3]);
    }

    #[test]
    fn sort_empty() {
        let mut xs = vec![];
        sort(&mut xs);
        assert_eq!(xs, vec![]);
    }

    #[test]
    fn sort_sorted() {
        let mut xs = vec![1, 2, 3];
        sort(&mut xs);
        assert_eq!(xs, vec![1, 2, 3]);
    }
}
