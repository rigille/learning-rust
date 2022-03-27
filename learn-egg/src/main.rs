use egg::{*, rewrite as rw};

fn main() {
    let rules: &[Rewrite<SymbolLang, ()>] = &[
        rw!("commute-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        rw!("commute-mul"; "(* ?x ?y)" => "(* ?y ?x)"),

        rw!("add-0"; "(+ ?x 0)" => "?x"),
        rw!("mul-0"; "(* ?x 0)" => "0"),
        rw!("mul-1"; "(* ?x 1)" => "?x"),
    ];

    // While it may look like we are working with numbers,
    // SymbolLang stores everything as strings.
    // We can make our own Language later to work with other types.
    let start = "(+ 0 (* 1 a))".parse().unwrap();

    // That's it! We can run equality saturation now.
    let runner = Runner::default().with_expr(&start).run(rules);

    //let mut expr = RecExpr::default();

//    let test = expr.add(runner.egraph[runner.roots[0]].nodes[1].clone());
//    println!("class: {:?}", test);
    for class in runner.egraph.classes() {
        println!("class: {:?}", class);
    }
}
