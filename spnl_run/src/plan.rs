use spnl_ast::Unit;

pub fn plan(ast: &Unit) -> Unit {
    match ast {
        x => x.clone(),
    }
}
/*        Unit::Plus{description, units} => {

}*/
