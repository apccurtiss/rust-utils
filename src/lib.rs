#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct VectorWrapper<A> {
    vector: Vec<A>
}

struct Concatinated<'a, 'b, A: 'b + 'a> {
    left: &'a ListThingy<A>,
    right: &'b ListThingy<A>
}

trait ListThingy<A> {
    fn concat<'a, 'b>(&'a self, v: &'b ListThingy<A>) -> Concatinated<'a, 'b, A>;
    fn index(&self, i: usize) -> &A;
    fn length(&self) -> usize;
}

impl<A> ListThingy<A> for VectorWrapper<A> {
    fn concat<'a, 'b>(&'a self, v: &'b ListThingy<A>) -> Concatinated<'a, 'b, A> {
        Concatinated {
            left: self,
            right: v,
        }
    }
    fn index(&self, i: usize) -> &A {
        &self.vector[i]
    }
    fn length(&self) -> usize {
        self.vector.len()
    }
}

impl<'i, 'j, A> ListThingy<A> for Concatinated<'i, 'j, A> {
    fn concat<'a, 'b>(&'a self, v: &'b ListThingy<A>) -> Concatinated<'a, 'b, A> {
        Concatinated {
            left: self,
            right: v,
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

// impl<'a, 'b: 'a, A> ListThingy<'a, 'b, A> for VectorWrapper<A> {
//     fn concat(&self, v: &'b ListThingy<'a, 'b, A>) -> Concatinated<A> {
//         Concatinated {
//             left: self,
//             right: v,
//         }
//     }
//     fn index(&self, i: usize) -> &A {
//         self.vector[i]
//     }
// }



pub fn greetings() -> String {
    let a = VectorWrapper{
        vector: vec![1, 2]
    };
    let b = VectorWrapper{
        vector: vec![3, 4]
    };

    println!("{}", a.index(1));

    let c = a.concat(&b);
    println!("{}", c.index(1));
    println!("{}", c.index(3));
    return "Test".to_string();
}
