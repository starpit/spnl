use crate::Query;

fn expand_repeats(v: &Vec<Query>) -> Vec<Query> {
    v.iter()
        .flat_map(|u| match u {
            Query::Repeat((n, uu)) => ::std::iter::repeat(plan(&uu)).take(*n).collect::<Vec<_>>(),
            x => vec![plan(x)],
        })
        .collect()
}

pub fn plan(ast: &Query) -> Query {
    // this is probably the wrong place for this, but here we expand any Repeats under Plus or Cross
    match ast {
        Query::Plus(v) => Query::Plus(expand_repeats(v)),
        Query::Cross(v) => Query::Cross(expand_repeats(v)),
        Query::Generate((m, i, mt, t, accumulate)) => {
            Query::Generate((m.clone(), Box::new(plan(i)), *mt, *t, *accumulate))
        }
        x => x.clone(),
    }
}
