extern crate quickcheck;
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

macro_rules! check_specialized {
    ($iterator:tt.$($token:tt)*) => {
        {
            let expected = Unspecialized($iterator.clone()).$($token)*;
            let actual = $iterator.clone().$($token)*;
            assert_eq!(expected, actual);
            actual
        }
    }
}

struct Unspecialized<I>(I);
impl<I> Iterator for Unspecialized<I>
where
    I: Iterator,
{
    type Item = I::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<I::Item> {
        self.0.next()
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

#[quickcheck]
fn iterator_test(test_vec: Vec<i32>) {
    let hms: multiset::HashMultiSet<_> = test_vec.into_iter().collect();
    let iter = hms.iter();
    let size = check_specialized!(iter.count());
    check_specialized!(iter.last());
    for n in 0..size + 2 {
        check_specialized!(iter.nth(n));
    }
    let mut it_sh = iter.clone();
    for n in 0..size + 2 {
        let len = it_sh.clone().count();
        let (min, max) = it_sh.size_hint();
        assert_eq!((size - n.min(size)), len);
        assert!(min <= len);
        if let Some(max) = max {
            assert!(len <= max);
        }
        it_sh.next();
    }
    check_specialized!(iter.fold(0i32, |acc, v| *v ^ acc));
}
