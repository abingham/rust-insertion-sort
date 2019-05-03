use std::iter::FromIterator;
use std::iter::Iterator;

pub fn sort<I: PartialOrd + Copy>(xs: &mut Vec<I>) {
    for current_index in 1..xs.len() {
        let current = &xs[current_index];

        match xs[0..current_index]
            .iter()
            .enumerate()
            .find_map(|(idx, &x)| if current < &x { Some(idx) } else { None })
        {
            Some(insertion_index) => {
                let to_shift = Vec::from_iter(xs[insertion_index..current_index].iter().cloned());
                let current = current.clone();
                xs.splice(insertion_index + 1..current_index + 1, to_shift);
                xs[insertion_index] = current.clone();
            }
            _ => {}
        }
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
        let mut xs: Vec<i32> = vec![];
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
