use proptest::prelude::*;
use proptest::strategy::*;
use proptest::test_runner::TestRunner;

#[derive(Clone, Debug)]
struct Test {
    numb: u32,
}

enum MyTree {
    Mono,
}

impl ValueTree for MyTree {
    type Value = Test;
    fn current(&self) -> Test {
        Test { numb: 0 }
    }
    fn complicate(&mut self) -> bool {
        false
    }
    fn simplify(&mut self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
struct Any(());
const ANY: Any = Any(());

impl Strategy for Any {
    type Tree = MyTree;
    type Value = Test;
    fn new_tree(&self, _runner: &mut TestRunner) -> NewTree<Self> {
        Ok(MyTree::Mono)
    }
}

//impl Strategy for Test {
//    type Tree = Test;
//    type Value = u32;
//}

proptest! {
    #[test]
    fn test_do_stuff(t in ANY) {
        println!("{:?}", t);
    }
}

fn main() {
    ()
}
