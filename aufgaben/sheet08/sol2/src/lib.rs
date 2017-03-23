mod tests;

use std::cmp::Ordering;

/// Sorts the array with either quick or insertion sort, depending on the
/// array's size.
fn hybrid_sort<T, F>(arr: &mut [T], compare: &mut F)
    where   F: FnMut(&T, &T) -> Ordering,
{
    if arr.len() <= 32 {
        insertion_sort(arr, compare);
    } else {
        quick_sort(arr, compare);
    }
}

/// Performs quick sort.
///
/// Quick sort is a sorting algorithm running in O(n * log n) average case. The
/// worst case is O(n²), though. A lot depends on the choice of the pivot
/// element. This implementation chooses the pivot in a rather stupid manner,
/// so the worst-case is only "highly unlikely" and not "extremely unlikely".
///
/// The algorithm works recursively by partitioning the array into two parts.
/// In the first, all elements have to be smaller than the pivot element, in
/// the second everything is greater than the pivot. Both parts can then be
/// sorted independent of each other. After the array has been partitioned it
/// looks somewhat like this:
///
/// ```
///              ▇ ▂   ▆
///    ▃ ▅ ▃ ▅ ▁ █ █ ▇ █ ▆
///
///    \       / \       /
///      less      greater
/// ```
fn quick_sort<T, F>(arr: &mut [T], compare: &mut F)
    where   F: FnMut(&T, &T) -> Ordering,
{
    // If the array only conains 0 or 1 element, we can stop, since it's
    // already sorted.
    if arr.len() <= 1 {
        return;
    }
}

/// Performs insertion sort.
///
/// Insertion sort runs in O(n²) worst case, but is extremly fast on nearly
/// sorted or small inputs. It's commonly used to handle the small sub-arrays
/// of recursive algorithms such as quick sort or merge sort.
///
/// Insertion sort splits the array in two parts: a sorted first part and an
/// unsorted second part. In each iteration the first element of the unsorted
/// part is inserted at the correct position into the sorted part. The array
/// may look like this during the algorithm:
///
/// ```
///          ▂ ▆     ▇
///    ▃ ▆ ▇ █ █ ▅ ▁ █ ▅ ▃
///
///    \       / \       /
///     sorted    unsorted
/// ```
///
/// # Performance
///
/// To insert the next element at the correct position in the sorted part, we
/// have multiple possibilities:
///
/// 1. Always swap the element with the element left to it, if the former is
///    smaller. This is what this implementation does. It's fairly easy but
///    we do quite a few unnecessary writes. This method is *slow*!
///
/// 2. We can first find the correct position to insert by searching through
///    all elements first, then shift all elements to the right and insert the
///    new element. This is way better than the first method.
///
/// 3. Like (2.) but we already shift all elements to the right while searching
///    for the right insertion-position. This is the best method.
///
/// Sadly, (2.) and (3.) require some unsafe hackery. Why? Ownership and Drop!
///
/// We don't know what `Drop` is, yet, but we know that each resource should
/// have one owner. We *can't* move out of an array, so we can't really get
/// out the new element to do the shift with the rest. We will learn how to
/// theoretically optimize this method later.
///
fn insertion_sort<T, F>(arr: &mut [T], compare: &mut F)
    where   F: FnMut(&T, &T) -> Ordering,
{

}

pub fn sort_by<T, F>(arr: &mut [T], mut f: F)
    where   F: FnMut(&T, &T) -> Ordering,
{

}

pub fn sort_by_key<T, F, K>(arr: &mut [T], mut map: F)
    where   F: FnMut(&T) -> K,
            K: Ord,
{

}

pub fn sort<T: Ord>(arr: &mut [T]) {

}
