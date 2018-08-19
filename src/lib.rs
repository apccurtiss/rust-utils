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

struct Concatinated<'a, A> {
    left: &'a ListThingy<A>,
    right: &'a ListThingy<A>
}

trait ListThingy<A> {
}
//     fn concat<'x, 'y, 'n: 'x+'l, 'm: 'y>(&self, v: &'x ListThingy<'y, A>) -> Concatinated<'n, 'm, A>;
//     fn index(&self, i: usize) -> A;
// }

// impl<'a, 'b: 'a, A> ListThingy<'a, 'b, A> for VectorWrapper<A> {
//     fn concat(&self, v: &'b ListThingy<'a, 'b, A>) -> Concatinated<A> {
//         Concatinated {
//             left: self,
//             right: v,
//         }
//     }
//     fn index(&self, i: usize) -> A {
//         self.vector[i]
//     }
// }



pub fn greetings() -> String {
    Concatinated {
        left: VectorWrapper {
            vector: vec![1, 2]
        },
        right: VectorWrapper {
            vector: vec![1, 2]
        }
    };
    return "Test".to_string();
}
