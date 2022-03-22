//use egg;

#[derive(Clone, Debug, Hash)]
pub enum Term {
  Var { name: String }, // TODO: add `global: bool`
  Dup { nam0: String, nam1: String, expr: BTerm, body: BTerm },
  Let { name: String, expr: BTerm, body: BTerm },
  Lam { name: String, body: BTerm },
  App { func: BTerm, argm: BTerm },
  Ctr { name: String, args: Vec<BTerm> },
  U32 { numb: u32 },
  Op2 { oper: Oper, val0: BTerm, val1: BTerm },
}

pub type BTerm = Box<Term>;

#[derive(Clone, Copy, Debug, Hash)]
pub enum Oper {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  And,
  Or,
  Xor,
  Shl,
  Shr,
  Lte,
  Ltn,
  Eql,
  Gte,
  Gtn,
  Neq,
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
          (Term::Var { name: self_name },
          Term::Var { name: other_name }) => false,

          (Term::Dup { nam0: self_nam0, nam1: self_nam1, expr: self_expr, body: self_body },
           Term::Dup { nam0: other_nam0, nam1: other_nam1, expr: other_expr, body: other_body }) => false,

          (Term::Let { name: self_name, expr: self_expr, body: self_body },
          Term::Let { name: other_name, expr: other_expr, body: other_body }) => false,

          (Term::Lam { name: self_name, body: self_body },
          Term::Lam { name: other_name, body: other_body }) => false,

          (Term::App { func: self_func, argm: self_argm },
          Term::App { func: other_func, argm: other_argm }) => false,

          (Term::Ctr { name: self_name, args: self_args },
          Term::Ctr { name: other_name, args: other_args }) => false,

          (Term::U32 { numb: self_numb },
          Term::U32 { numb: other_numb } } => false,

          (Term::Op2 { oper: self_oper, val0: self_val0, val1: self_val1 },
          Term::Op2 { oper: other_oper, val0: other_val0, val1: other_val1 }) => false,

          _ => false,
        }
    }
}
impl Eq for Term {}

fn main() {
    let test = Term::U32 { numb: 0 };
    println!("Hello, world!");
}
