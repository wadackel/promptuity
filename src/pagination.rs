//! A module that assists with pagination functionality.
//!
//! # Pagination
//!
//! The `pagination` module provides functionalities to assist with pagination.  
//! It offers generic logic as a utility, so it can be expected to be useful in your custom prompts as well.
//!
//! In built-in prompts, it is used in [`crate::prompts::Select`] and [`crate::prompts::MultiSelect`].
//!
//! ## Examples
//!
//! ```
//! use promptuity::pagination::{Page, paginate};
//!
//! let page_size = 5;
//! let items = vec![1, 2, 3, 4, 5, 6, 7, 8];
//!
//! assert_eq!(paginate(page_size, &items, 1), Page {
//!     first: true,
//!     last: false,
//!     items: &[1, 2, 3, 4, 5],
//!     cursor: 1,
//!     total: 8,
//! });
//!
//! assert_eq!(paginate(page_size, &items, 3), Page {
//!     first: false,
//!     last: false,
//!     items: &[2, 3, 4, 5, 6],
//!     cursor: 2,
//!     total: 8,
//! });
//!
//! assert_eq!(paginate(page_size, &items, 7), Page {
//!     first: false,
//!     last: true,
//!     items: &[4, 5, 6, 7, 8],
//!     cursor: 4,
//!     total: 8,
//! });
//! ```

/// A page of items.
#[derive(Debug, PartialEq)]
pub struct Page<'a, T> {
    /// A flag indicating whether this is the first page.
    pub first: bool,
    /// A flag indicating whether this is the last page.
    pub last: bool,
    /// The items in this page.
    pub items: &'a [T],
    /// The cursor position in this page.
    pub cursor: usize,
    /// The total number of items.
    pub total: usize,
}

/// Paginates the given items.
pub fn paginate<T>(size: usize, items: &[T], current: usize) -> Page<'_, T> {
    let (begin, end, cursor) = if items.len() <= size {
        (0, items.len(), current)
    } else if current < size / 2 {
        (0, size, current)
    } else if items.len() - current - 1 < size / 2 {
        let begin = items.len() - size;
        (begin, items.len(), current - begin)
    } else {
        (current - size / 2, current + (size - size / 2), size / 2)
    };

    Page {
        first: begin == 0,
        last: end == items.len(),
        items: &items[begin..end],
        cursor,
        total: items.len(),
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    fn range(r: std::ops::Range<usize>) -> Vec<usize> {
        (r.start..r.end + 1).collect()
    }

    #[test]
    fn test_page() {
        let items_1_2 = range(1..2);
        let items_1_5 = range(1..5);
        let items_4_8 = range(4..8);
        let items_9_13 = range(9..13);
        let items_11_15 = range(11..15);

        let tests = vec![
            (
                5,
                range(1..15),
                0,
                Page {
                    first: true,
                    last: false,
                    items: &items_1_5,
                    cursor: 0,
                    total: 15,
                },
            ),
            (
                5,
                range(1..15),
                2,
                Page {
                    first: true,
                    last: false,
                    items: &items_1_5,
                    cursor: 2,
                    total: 15,
                },
            ),
            (
                5,
                range(1..15),
                5,
                Page {
                    first: false,
                    last: false,
                    items: &items_4_8,
                    cursor: 2,
                    total: 15,
                },
            ),
            (
                5,
                range(1..15),
                10,
                Page {
                    first: false,
                    last: false,
                    items: &items_9_13,
                    cursor: 2,
                    total: 15,
                },
            ),
            (
                5,
                range(1..15),
                12,
                Page {
                    first: false,
                    last: true,
                    items: &items_11_15,
                    cursor: 2,
                    total: 15,
                },
            ),
            (
                5,
                range(1..15),
                13,
                Page {
                    first: false,
                    last: true,
                    items: &items_11_15,
                    cursor: 3,
                    total: 15,
                },
            ),
            (
                5,
                range(1..15),
                14,
                Page {
                    first: false,
                    last: true,
                    items: &items_11_15,
                    cursor: 4,
                    total: 15,
                },
            ),
            (
                5,
                range(1..5),
                3,
                Page {
                    first: true,
                    last: true,
                    items: &items_1_5,
                    cursor: 3,
                    total: 5,
                },
            ),
            (
                5,
                range(1..2),
                1,
                Page {
                    first: true,
                    last: true,
                    items: &items_1_2,
                    cursor: 1,
                    total: 2,
                },
            ),
        ];
        for (size, items, current, expected) in tests {
            assert_eq!(expected, paginate(size, &items, current));
        }
    }
}
