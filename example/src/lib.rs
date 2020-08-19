use pm::pm;

struct S {}

impl S {
    #[pm]
    fn by_mut_ref(&mut self) -> bool {
        false
    }

    #[pm]
    fn own(self) -> bool {
        false
    }

    #[pm]
    fn own_mut(mut self) -> bool {
        false
    }

    #[pm]
    fn by_ref(&self) -> bool {
        false
    }
}
