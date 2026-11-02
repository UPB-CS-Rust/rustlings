// This quiz tests:
// - Ownership (Copy and move types)
// - Borrowing
// - Lifetime labels
//
// A small catalog of books. Implement the following functions. Some of them
// need lifetime labels in their signatures, add them where they are needed.
//
// - `title_of`: Returns (a reference to) the title of a book.
// - `older`: Returns (a reference to) the book that was published earlier.
//   If both were published in the same year, return the first one.
// - `retitle`: Replaces the title of a book.
// - `into_author`: Consumes the book and returns its author (no clone!).
// - `swap_authors`: Swaps the authors of two books.
//   Hint: `std::mem::swap(&mut x, &mut y)` swaps two values in place.

struct Book {
    title: String,
    author: String,
    year: u32,
}

// TODO: Implement the functions below and add the missing lifetime labels.

fn title_of(book: &Book) -> &str {
    ""
}

fn older(a: &Book, b: &Book) -> &Book {
    a
}

fn retitle(book: &mut Book, new_title: &str) {}

fn into_author(book: Book) -> String {
    String::new()
}

fn swap_authors(a: &mut Book, b: &mut Book) {}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    fn dune() -> Book {
        Book {
            title: String::from("Dune"),
            author: String::from("Frank Herbert"),
            year: 1965,
        }
    }

    fn hobbit() -> Book {
        Book {
            title: String::from("The Hobbit"),
            author: String::from("J. R. R. Tolkien"),
            year: 1937,
        }
    }

    #[test]
    fn test_title() {
        let book = dune();
        assert_eq!(title_of(&book), "Dune");
    }

    #[test]
    fn test_older() {
        let a = dune();
        let result;
        {
            let b = hobbit();
            result = older(&a, &b);
            assert_eq!(result.title, "The Hobbit");
        }
        let c = dune();
        assert_eq!(older(&a, &c).author, "Frank Herbert");
    }

    #[test]
    fn test_retitle() {
        let mut book = dune();
        retitle(&mut book, "Dune Messiah");
        assert_eq!(book.title, "Dune Messiah");
        assert_eq!(book.year, 1965);
    }

    #[test]
    fn test_into_author() {
        let author = into_author(hobbit());
        assert_eq!(author, "J. R. R. Tolkien");
    }

    #[test]
    fn test_swap() {
        let mut a = dune();
        let mut b = hobbit();
        swap_authors(&mut a, &mut b);
        assert_eq!(a.author, "J. R. R. Tolkien");
        assert_eq!(b.author, "Frank Herbert");
        assert_eq!(a.title, "Dune");
    }
}
