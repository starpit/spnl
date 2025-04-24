use spl_ast::Unit;

pub fn plan<'a>(ast: &'a Unit<'a>) -> Unit<'a> {
    ast.clone()
}
