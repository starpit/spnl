use crate::Unit;

fn expand_repeats(v: &Vec<Unit>) -> Vec<Unit> {
    v.iter()
        .flat_map(|u| match u {
            Unit::Repeat((n, uu)) => ::std::iter::repeat(*uu.clone())
                .take(*n)
                .collect::<Vec<_>>(),
            x => vec![x.clone()],
        })
        .collect()
}

pub fn plan(ast: &Unit) -> Unit {
    // this is probably the wrong place for this, but here we expand any Repeats under Plus or Cross
    match ast {
        Unit::Plus(v) => Unit::Plus(expand_repeats(v)),
        Unit::Cross(v) => Unit::Cross(expand_repeats(v)),
        x => x.clone(),
    }
}
