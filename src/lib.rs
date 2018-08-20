pub struct VectorWrapper<A> {
    vector: Vec<A>
}

pub struct Concatenated<'a, 'b, A: 'a + 'b> {
    left: &'a Persistent<A>,
    right: &'b Persistent<A>
}

pub struct Prepended<'a, 'b, A: 'a + 'b> {
    value: &'b A,
    rest: &'a Persistent<A>
}

pub struct Appended<'a, 'b, A: 'a + 'b> {
    rest: &'a Persistent<A>,
    value: &'b A
}

pub struct Changed<'a, 'b, A: 'a + 'b> {
    rest: &'a Persistent<A>,
    index: usize,
    value: &'b A
}

pub fn from_vec<A>(v: Vec<A>) -> VectorWrapper<A> {
    VectorWrapper{vector : v}
}

pub trait Persistent<A> {
    fn concat<'a, 'b>(&'a self, v: &'b Persistent<A>) -> Concatenated<'a, 'b, A>;
    fn prepend<'a, 'b>(&'a self, v: &'b A) -> Prepended<'a, 'b, A>;
    fn append<'a, 'b>(&'a self, v: &'b A) -> Appended<'a, 'b, A>;
    fn set<'a, 'b>(&'a self, i: usize, v: &'b A) -> Changed<'a, 'b, A>;
    fn index(&self, i: usize) -> &A;
    fn length(&self) -> usize;

    fn flatten(&self) -> Vec<&A> {
        let mut v = Vec::with_capacity(self.length());
        for i in 0..self.length() {
            v.push(self.index(i));
        }
        return v;
    }
}

impl<A> Persistent<A> for VectorWrapper<A> {
    fn concat<'a, 'b>(&'a self, v: &'b Persistent<A>) -> Concatenated<'a, 'b, A> {
        Concatenated {
            left: self,
            right: v,
        }
    }

    fn prepend<'a, 'b>(&'a self, v: &'b A) -> Prepended<'a, 'b, A> {
        Prepended {
            value: v,
            rest: self
        }
    }

    fn append<'a, 'b>(&'a self, v: &'b A) -> Appended<'a, 'b, A> {
        Appended {
            value: v,
            rest: self
        }
    }

    fn set<'a, 'b>(&'a self, i: usize, v: &'b A) -> Changed<'a, 'b, A> {
        Changed {
            index: i,
            value: v,
            rest: self
        }
    }

    fn index(&self, i: usize) -> &A {
        &self.vector[i]
    }
    fn length(&self) -> usize {
        self.vector.len()
    }
}

impl<'i, 'j, A> Persistent<A> for Concatenated<'i, 'j, A> {
    fn concat<'a, 'b>(&'a self, v: &'b Persistent<A>) -> Concatenated<'a, 'b, A> {
        Concatenated {
            left: self,
            right: v,
        }
    }

    fn prepend<'a, 'b>(&'a self, v: &'b A) -> Prepended<'a, 'b, A> {
        Prepended {
            value: v,
            rest: self
        }
    }

    fn append<'a, 'b>(&'a self, v: &'b A) -> Appended<'a, 'b, A> {
        Appended {
            value: v,
            rest: self
        }
    }

    fn set<'a, 'b>(&'a self, i: usize, v: &'b A) -> Changed<'a, 'b, A> {
        Changed {
            index: i,
            value: v,
            rest: self
        }
    }


    fn index(&self, i: usize) -> &A {
        if i < self.left.length() {
            self.left.index(i)
        }
        else {
            self.right.index(i - self.left.length())
        }
    }

    fn length(&self) -> usize {
        self.left.length() + self.right.length()
    }
}


impl<'i, 'j, A> Persistent<A> for Prepended<'i, 'j, A> {
    fn concat<'a, 'b>(&'a self, v: &'b Persistent<A>) -> Concatenated<'a, 'b, A> {
        Concatenated {
            left: self,
            right: v,
        }
    }

    fn prepend<'a, 'b>(&'a self, v: &'b A) -> Prepended<'a, 'b, A> {
        Prepended {
            value: v,
            rest: self
        }
    }

    fn append<'a, 'b>(&'a self, v: &'b A) -> Appended<'a, 'b, A> {
        Appended {
            value: v,
            rest: self
        }
    }

    fn set<'a, 'b>(&'a self, i: usize, v: &'b A) -> Changed<'a, 'b, A> {
        Changed {
            index: i,
            value: v,
            rest: self
        }
    }


    fn index(&self, i: usize) -> &A {
        if i == 0 {
            self.value
        }
        else {
            self.rest.index(i - 1)
        }
    }

    fn length(&self) -> usize {
        self.rest.length() + 1
    }
}

impl<'i, 'j, A> Persistent<A> for Appended<'i, 'j, A> {
    fn concat<'a, 'b>(&'a self, v: &'b Persistent<A>) -> Concatenated<'a, 'b, A> {
        Concatenated {
            left: self,
            right: v,
        }
    }

    fn prepend<'a, 'b>(&'a self, v: &'b A) -> Prepended<'a, 'b, A> {
        Prepended {
            value: v,
            rest: self
        }
    }

    fn append<'a, 'b>(&'a self, v: &'b A) -> Appended<'a, 'b, A> {
        Appended {
            value: v,
            rest: self
        }
    }

    fn set<'a, 'b>(&'a self, i: usize, v: &'b A) -> Changed<'a, 'b, A> {
        Changed {
            index: i,
            value: v,
            rest: self
        }
    }


    fn index(&self, i: usize) -> &A {
        if i == self.rest.length() {
            self.value
        }
        else {
            self.rest.index(i)
        }
    }

    fn length(&self) -> usize {
        self.rest.length() + 1
    }
}

impl<'i, 'j, A> Persistent<A> for Changed<'i, 'j, A> {
    fn concat<'a, 'b>(&'a self, v: &'b Persistent<A>) -> Concatenated<'a, 'b, A> {
        Concatenated {
            left: self,
            right: v,
        }
    }

    fn prepend<'a, 'b>(&'a self, v: &'b A) -> Prepended<'a, 'b, A> {
        Prepended {
            value: v,
            rest: self
        }
    }

    fn append<'a, 'b>(&'a self, v: &'b A) -> Appended<'a, 'b, A> {
        Appended {
            value: v,
            rest: self
        }
    }

    fn set<'a, 'b>(&'a self, i: usize, v: &'b A) -> Changed<'a, 'b, A> {
        Changed {
            index: i,
            value: v,
            rest: self
        }
    }


    fn index(&self, i: usize) -> &A {
        if i == self.index {
            self.value
        }
        else {
            self.rest.index(i)
        }
    }

    fn length(&self) -> usize {
        self.rest.length()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
