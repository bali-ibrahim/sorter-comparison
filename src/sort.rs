// https://www.rosettacode.org/wiki/Category:Sorting_Algorithms

#[derive(Default, Debug)]
pub struct NumberOf {
    assignments: usize,
    comparisons: usize,
}

pub fn with_selection(array: &mut Vec<isize>) -> NumberOf {
    let mut number_of = NumberOf {
        assignments: 0,
        comparisons: 0,
    };
    let mut min;

    for i in 0..array.len() {
        min = i;

        for j in (i + 1)..array.len() {
            if array[j] < array[min] {
                min = j;
            }
            number_of.comparisons = number_of.comparisons + 1;
        }

        let tmp = array[i];
        array[i] = array[min];
        array[min] = tmp;
        number_of.assignments = number_of.assignments + 2;
    }
    number_of
}

pub fn with_merge<T: Copy + PartialOrd>(x: &mut Vec<T>) -> NumberOf {
    let mut number_of = NumberOf {
        assignments: 0,
        comparisons: 0,
    };

    let n = x.len();
    let mut y = x.to_vec();
    let mut len = 1;
    while len < n {
        let mut i = 0;
        while i < n {
            if i + len >= n {
                y[i..].copy_from_slice(&x[i..]);
            } else if i + 2 * len > n {
                merge(&x[i..i + len], &x[i + len..], &mut y[i..], &mut number_of);
            } else {
                merge(
                    &x[i..i + len],
                    &x[i + len..i + 2 * len],
                    &mut y[i..i + 2 * len],
                    &mut number_of,
                );
            }
            i += 2 * len;
        }
        len *= 2;
        if len >= n {
            x.copy_from_slice(&y);
            return number_of;
        }
        i = 0;
        while i < n {
            if i + len >= n {
                x[i..].copy_from_slice(&y[i..]);
            } else if i + 2 * len > n {
                merge(&y[i..i + len], &y[i + len..], &mut x[i..], &mut number_of);
            } else {
                merge(
                    &y[i..i + len],
                    &y[i + len..i + 2 * len],
                    &mut x[i..i + 2 * len],
                    &mut number_of,
                );
            }
            i += 2 * len;
        }
        len *= 2;
    }
    number_of
}

fn merge<T: Copy + PartialOrd>(x1: &[T], x2: &[T], y: &mut [T], number_of: &mut NumberOf) {
    assert_eq!(x1.len() + x2.len(), y.len());
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
        number_of.assignments = number_of.assignments + 1;
        number_of.comparisons = number_of.comparisons + 1;
    }
    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }
    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

pub fn with_quicksort(v: &mut Vec<isize>) -> NumberOf {
    let mut number_of = NumberOf {
        assignments: 0,
        comparisons: 0,
    };
    quick_sort(v, &|x, y| x < y, &mut number_of);
    number_of
}

fn quick_sort<T, F>(v: &mut [T], f: &F, number_of: &mut NumberOf)
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    if len >= 2 {
        let pivot_index = partition(v, f, number_of);
        quick_sort(&mut v[0..pivot_index], f, number_of);
        quick_sort(&mut v[pivot_index + 1..len], f, number_of);
    }
}

fn partition<T, F>(v: &mut [T], f: &F, number_of: &mut NumberOf) -> usize
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    let pivot_index = len / 2;
    let last_index = len - 1;

    v.swap(pivot_index, last_index);

    let mut store_index = 0;
    for i in 0..last_index {
        if f(&v[i], &v[last_index]) {
            v.swap(i, store_index);
            store_index += 1;
            number_of.assignments = number_of.assignments + 2;
        }
        number_of.comparisons = number_of.comparisons + 1;
    }

    v.swap(store_index, len - 1);
    number_of.assignments = number_of.assignments + 2;
    store_index
}
