use egg::{*};

#[test]
fn two_matches() {
    let zero: SymbolLang = SymbolLang::leaf("0");
    let one: SymbolLang = SymbolLang::leaf("1");
    let foo: SymbolLang = SymbolLang::leaf("foo");
    let onefoo: RecExpr<SymbolLang> = "(* 1 foo)".parse().unwrap();
    let fooone: RecExpr<SymbolLang> = "(* foo 1)".parse().unwrap();

    let mut egraph = EGraph::<SymbolLang, ()>::default();
    egraph.add(zero);
    egraph.add(one);
    let foo_id: Id = egraph.add(foo);
    let onefoo_id: Id = egraph.add_expr(&onefoo);
    let fooone_id: Id = egraph.add_expr(&fooone);

    egraph.union(foo_id, onefoo_id);
    egraph.union(foo_id, fooone_id);
    egraph.rebuild();

    let rule: Rewrite<SymbolLang,()> = rewrite!("assoc"; "(* (* ?a ?b) ?c)" => "(* ?a (* ?b ?c))");
    
    assert_eq!(2, rule.searcher.n_matches(&egraph));
// println!("{:?}", rule.search(&egraph));
}
