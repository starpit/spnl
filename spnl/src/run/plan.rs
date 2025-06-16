use crate::{Generate, Query, Repeat};

fn expand_repeats(v: &Vec<Query>) -> Vec<Query> {
    v.iter()
        .flat_map(|u| match u {
            Query::Repeat(Repeat { n, query }) => ::std::iter::repeat(plan(&query))
                .take(*n)
                .collect::<Vec<_>>(),
            x => vec![plan(x)],
        })
        .collect()
}

pub fn plan(ast: &Query) -> Query {
    // this is probably the wrong place for this, but here we expand any Repeats under Plus or Cross
    match ast {
        Query::Plus(v) => Query::Plus(expand_repeats(v)),
        Query::Cross(v) => Query::Cross(expand_repeats(v)),
        Query::Generate(Generate {
            model,
            input,
            max_tokens,
            temperature,
            accumulate,
        }) => Query::Generate(Generate {
            model: model.clone(),
            input: Box::new(plan(input)),
            max_tokens: max_tokens.clone(),
            temperature: temperature.clone(),
            accumulate: accumulate.clone(),
        }),
        x => x.clone(),
    }
}
