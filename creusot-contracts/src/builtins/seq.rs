use creusot_contracts_proc::*;

use crate::Int;
use std::ops::Index;

#[creusot::builtins = "seq.Seq.seq"]
pub struct Seq<T: ?Sized>(std::marker::PhantomData<T>);

impl<T> Seq<T> {
    #[logic]
    pub fn get(self, ix: Int) -> Option<T> {
        if ix < self.len() {
            Some(*self.index(ix))
        } else {
            None
        }
    }

    #[trusted]
    #[logic]
    #[creusot::builtins = "seq_ext.SeqExt.subsequence"]
    pub fn subsequence(self, i: Int, j: Int) -> Self {
        std::process::abort()
    }

    #[logic]
    pub fn tail(self) -> Self {
        self.subsequence(1, self.len())
    }

    #[trusted]
    #[logic]
    #[creusot::builtins = "seq.Seq.length"]
    pub fn len(self) -> Int {
        std::process::abort()
    }

    #[trusted]
    #[logic]
    #[creusot::builtins = "seq.Seq.set"]
    pub fn set(self, _: Int, _: T) -> Self {
        std::process::abort()
    }

    #[trusted]
    #[logic]
    #[creusot::builtins = "seq.Seq.snoc"]
    pub fn push(self, _: T) -> Self {
        std::process::abort()
    }
    #[predicate]
    pub fn permutation_of(self, o: Self) -> bool {
        self.permut(o, 0, self.len())
    }

    #[trusted]
    #[predicate]
    #[creusot::builtins = "seq.Permut.permut"]
    pub fn permut(self, _: Self, _: Int, _: Int) -> bool {
        std::process::abort()
    }
}

// A hack which allows us to use [..] notation for sequences.
// Relies on the fact we don't enforce that implementations of traits are of
// the same function type as the trait signature.. When this is addressed
// the following instance will error.
impl<T> std::ops::Index<Int> for Seq<T> {
    type Output = T;

    #[trusted]
    #[logic]
    #[creusot::builtins = "seq.Seq.get"]
    fn index(&self, _: Int) -> &T {
        std::process::abort()
    }
}
