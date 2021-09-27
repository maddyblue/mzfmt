use pretty::*;
use sql_parser::ast::display::AstDisplay;
use sql_parser::{ast::*, parser::parse_statements};
use std::fmt::Display;

const TAB: isize = 4;

pub fn to_doc(v: &Statement<Raw>) -> RcDoc {
    match v {
        Statement::Select(v) => doc_select_statement(v),
        Statement::Insert(v) => doc_insert(v),
        Statement::CreateView(v) => doc_create_view(v),
        _ => doc_display(v),
    }
}

pub fn to_pretty(stmt: &Statement<Raw>, width: usize) -> String {
    let mut w = Vec::new();
    to_doc(stmt).render(width, &mut w).unwrap();
    let mut s = String::from_utf8(w).unwrap();
    s.push(';');
    s
}

pub fn pretty_strs(str: &str, width: usize) -> Result<Vec<String>, String> {
    let stmts = match parse_statements(str) {
        Ok(stmts) => stmts,
        Err(err) => return Err(err.to_string()),
    };
    Ok(stmts.iter().map(|s| to_pretty(s, width)).collect())
}

pub fn pretty_str(str: &str, width: usize) -> Result<String, String> {
    Ok(pretty_strs(str, width)?.join("\n\n"))
}

fn doc_display<'a, T: Display>(v: &T) -> RcDoc<'a, ()> {
    //eprintln!("UNKNOWN PRETTY TYPE {}, {}", std::any::type_name::<T>(), v);
    RcDoc::text(format!("{}", v))
}

fn nest_title<S>(title: S, v: RcDoc) -> RcDoc
where
    S: Into<String>,
{
    RcDoc::intersperse(vec![RcDoc::text(title.into()), v], Doc::line())
        .nest(TAB)
        .group()
}

fn title_comma_separate<'a, F, T, S>(title: S, f: F, v: &'a [T]) -> RcDoc<'a, ()>
where
    F: Fn(&'a T) -> RcDoc<'a, ()>,
    S: Into<String>,
{
    let title = RcDoc::text(title.into());
    if v.is_empty() {
        title
    } else {
        RcDoc::intersperse(vec![title, comma_separate(f, v)], Doc::line())
            .group()
            .nest(TAB)
    }
}

fn comma_separate<'a, F, T>(f: F, v: &'a [T]) -> RcDoc<'a, ()>
where
    F: Fn(&'a T) -> RcDoc<'a, ()>,
{
    let docs = v.iter().map(|v| f(v)).collect();
    comma_separated(docs)
}

fn comma_separated(v: Vec<RcDoc>) -> RcDoc {
    RcDoc::intersperse(v, RcDoc::concat(vec![RcDoc::text(","), RcDoc::line()])).group()
}

fn bracket<S: Into<String>>(left: S, d: RcDoc, right: S) -> RcDoc {
    RcDoc::concat(vec![
        RcDoc::text(left.into()),
        RcDoc::concat(vec![RcDoc::line_(), d]).nest(TAB),
        RcDoc::line_(),
        RcDoc::text(right.into()),
    ])
    .group()
}

//

fn doc_create_view(v: &CreateViewStatement<Raw>) -> RcDoc {
    let mut docs = vec![];
    docs.push(RcDoc::text(format!(
        "CREATE{}{}{} VIEW{}",
        if v.if_exists == IfExistsBehavior::Replace {
            " OR REPLACE"
        } else {
            ""
        },
        if v.temporary { " TEMPORARY" } else { "" },
        if v.materialized { " MATERIALIZED" } else { "" },
        if v.if_exists == IfExistsBehavior::Skip {
            " IF NOT EXISTS"
        } else {
            ""
        },
    )));
    docs.push(doc_view_definition(&v.definition));
    RcDoc::intersperse(docs, Doc::line()).nest(TAB).group()
}

fn doc_view_definition(v: &ViewDefinition<Raw>) -> RcDoc {
    let mut docs = vec![RcDoc::text(v.name.to_string())];
    if !v.with_options.is_empty() {
        docs.push(bracket(
            "WITH (",
            comma_separate(doc_display, &v.with_options),
            ")",
        ));
    }
    if !v.columns.is_empty() {
        docs.push(bracket("(", comma_separate(doc_display, &v.columns), ")"));
    }
    docs.push(nest_title("AS", doc_query(&v.query)));
    RcDoc::intersperse(docs, Doc::line()).group()
}

fn doc_insert(v: &InsertStatement<Raw>) -> RcDoc {
    let mut first = vec![RcDoc::text(format!("INSERT INTO {}", v.table_name))];
    if !v.columns.is_empty() {
        first.push(bracket("(", comma_separate(doc_display, &v.columns), ")"));
    }
    let sources = match &v.source {
        InsertSource::Query(query) => doc_query(&query),
        _ => doc_display(&v.source),
    };
    RcDoc::intersperse(
        vec![
            RcDoc::intersperse(first, Doc::line()).nest(TAB).group(),
            sources,
        ],
        Doc::line(),
    )
    .nest(TAB)
    .group()
}

fn doc_select_statement(v: &SelectStatement<Raw>) -> RcDoc {
    let mut doc = doc_query(&v.query);
    if let Some(as_of) = &v.as_of {
        doc = RcDoc::intersperse(
            vec![doc, nest_title("AS OF", doc_expr(&as_of))],
            Doc::line(),
        )
        .nest(TAB)
        .group();
    }
    doc.group()
}

fn doc_query(v: &Query<Raw>) -> RcDoc {
    let mut docs = vec![];
    if !v.ctes.is_empty() {
        docs.push(title_comma_separate("WITH", doc_cte, &v.ctes));
    }
    docs.push(doc_set_expr(&v.body));
    if !v.order_by.is_empty() {
        docs.push(title_comma_separate("ORDER BY", doc_display, &v.order_by));
    }

    let offset = if let Some(offset) = &v.offset {
        vec![RcDoc::concat(vec![nest_title("OFFSET", doc_expr(&offset))])]
    } else {
        vec![]
    };

    if let Some(limit) = &v.limit {
        if limit.with_ties {
            docs.extend(offset);
            docs.push(RcDoc::concat(vec![
                RcDoc::text("FETCH FIRST "),
                doc_expr(&limit.quantity),
                RcDoc::text(" ROWS WITH TIES"),
            ]));
        } else {
            docs.push(nest_title("LIMIT", doc_expr(&limit.quantity)));
            docs.extend(offset);
        }
    } else {
        docs.extend(offset);
    }

    RcDoc::intersperse(docs, Doc::line()).group()
}

fn doc_cte(v: &Cte<Raw>) -> RcDoc {
    RcDoc::concat(vec![
        RcDoc::text(format!("{} AS", v.alias)),
        RcDoc::line(),
        bracket("(", doc_display(&v.query), ")"),
    ])
}

fn doc_set_expr(v: &SetExpr<Raw>) -> RcDoc {
    match v {
        SetExpr::Select(v) => doc_select(v),
        SetExpr::Query(v) => bracket("(", doc_query(v), ")"),
        SetExpr::SetOperation {
            op,
            all,
            left,
            right,
        } => {
            let all_str = if *all { " ALL" } else { "" };
            RcDoc::concat(vec![
                doc_set_expr(left),
                RcDoc::line(),
                RcDoc::concat(vec![
                    RcDoc::text(format!("{}{}", op, all_str)),
                    RcDoc::line(),
                    doc_set_expr(right),
                ])
                .nest(TAB)
                .group(),
            ])
        }
        SetExpr::Values(v) => doc_values(v),
    }
    .group()
}

fn doc_values(v: &Values<Raw>) -> RcDoc {
    let rows =
        v.0.iter()
            .map(|row| bracket("(", comma_separate(doc_expr, row), ")"))
            .collect();
    RcDoc::concat(vec![
        RcDoc::text("VALUES"),
        RcDoc::line(),
        comma_separated(rows),
    ])
    .nest(TAB)
    .group()
}

fn doc_table_with_joins(v: &TableWithJoins<Raw>) -> RcDoc {
    let mut docs = vec![doc_table_factor(&v.relation)];
    for j in &v.joins {
        docs.push(doc_join(j));
    }
    RcDoc::intersperse(docs, Doc::line()).nest(TAB).group()
}

fn doc_join(v: &Join<Raw>) -> RcDoc {
    let (constraint, name) = match &v.join_operator {
        JoinOperator::Inner(constraint) => (constraint, "JOIN"),
        JoinOperator::FullOuter(constraint) => (constraint, "FULL JOIN"),
        JoinOperator::LeftOuter(constraint) => (constraint, "LEFT JOIN"),
        JoinOperator::RightOuter(constraint) => (constraint, "RIGHT JOIN"),
        _ => return doc_display(v),
    };
    let constraint = match constraint {
        JoinConstraint::On(expr) => RcDoc::concat(vec![RcDoc::text("ON "), doc_expr(&expr)]),
        JoinConstraint::Using(idents) => {
            bracket("USING(", comma_separate(doc_display, &idents), ")")
        }
        _ => return doc_display(v),
    };
    RcDoc::intersperse(
        vec![RcDoc::text(name), doc_table_factor(&v.relation), constraint],
        Doc::line(),
    )
    .nest(TAB)
    .group()
}

fn doc_table_factor(v: &TableFactor<Raw>) -> RcDoc {
    match v {
        TableFactor::Derived {
            lateral,
            subquery,
            alias,
        } => {
            if *lateral {
                return doc_display(v);
            }
            let mut docs = vec![bracket("(", doc_query(subquery), ")")];
            if let Some(alias) = &*alias {
                docs.push(RcDoc::text(format!("AS {}", alias)));
            }
            RcDoc::intersperse(docs, Doc::line()).nest(TAB).group()
        }
        TableFactor::NestedJoin { join, alias } => {
            let mut doc = bracket("(", doc_table_with_joins(join), ")");
            if let Some(alias) = alias {
                doc =
                    RcDoc::intersperse(vec![doc, RcDoc::text(format!("AS {}", alias))], Doc::line())
                        .nest(TAB)
                        .group()
            }
            doc
        }
        _ => doc_display(v),
    }
}

fn doc_select(v: &Select<Raw>) -> RcDoc {
    let mut docs = vec![];
    docs.push(title_comma_separate(
        format!(
            "SELECT{}",
            if let Some(distinct) = &v.distinct {
                format!(" {}", distinct.to_ast_string_stable())
            } else {
                "".into()
            }
        ),
        doc_display,
        &v.projection,
    ));
    if !v.from.is_empty() {
        docs.push(title_comma_separate("FROM", doc_table_with_joins, &v.from));
    }
    if let Some(selection) = &v.selection {
        docs.push(nest_title("WHERE", doc_expr(selection)));
    }
    if !v.group_by.is_empty() {
        docs.push(title_comma_separate("GROUP BY", doc_expr, &v.group_by));
    }
    if let Some(having) = &v.having {
        docs.push(nest_title("HAVING", doc_expr(having)));
    }
    if !v.options.is_empty() {
        docs.push(bracket(
            "OPTION (",
            comma_separate(doc_display, &v.options),
            ")",
        ));
    }
    RcDoc::intersperse(docs, Doc::line()).group()
}

fn doc_expr(v: &Expr<Raw>) -> RcDoc {
    match v {
        Expr::Op { op, expr1, expr2 } => {
            if let Some(expr2) = expr2 {
                RcDoc::concat(vec![
                    doc_expr(expr1),
                    RcDoc::line(),
                    RcDoc::text(format!("{} ", op)),
                    doc_expr(expr2).nest(TAB),
                ])
            } else {
                RcDoc::concat(vec![RcDoc::text(format!("{} ", op)), doc_expr(expr1)])
            }
        }
        Expr::Cast { expr, data_type } => bracket(
            "CAST(",
            RcDoc::concat(vec![
                doc_expr(expr),
                RcDoc::line(),
                RcDoc::text(format!("AS {}", data_type)),
            ])
            .nest(TAB),
            ")",
        ),

        Expr::Nested(ast) => bracket("(", doc_expr(ast), ")"),
        _ => doc_display(v),
    }
    .group()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sql_parser::parser::parse_statements;

    #[test]
    fn pretty() {
        let stmts = vec![
            //"with a as (select 'blah', 'another string') select 1, 2, 3 from a, b, c where a = b group by c, d having 1 = 4 AND a < c order by a limit 1 offset 2 rows",
            //"with a as (select 'blah', 'another string') select 1 from a",
            //"select 1 union select 2",
            //"insert into t (a,b,c) values (1,2,3), (4,5,6)",
            //"CREATE VIEW view_1 (col_2) AS SELECT * FROM (VALUES (CAST(0.9841192240680561 AS float)), (CAST(0.37823189648731315 AS float)), (CAST(0.8390174385199045 AS float)), (CAST(0.22188376517105302 AS float)), (CAST(0.5787854533815643 AS float)), (CAST(0.7234205380688273 AS float)), (CAST(0.39567191795118384 AS float)), (CAST(0.4348712893998896 AS float)), (CAST(0.8856762904388714 AS float)), (CAST(0.7704453261942663 AS float)), (CAST(0.5133022896871524 AS float)), (CAST(NULL AS float)), (CAST(0.5170637540787644 AS float)), (CAST(0.6762831752745486 AS float)), (CAST(0.2424964369655378 AS float)), (CAST(0.031422253928415134 AS float)), (CAST(0.7791437964022883 AS float)), (CAST(0.7976069716256476 AS float)), (CAST(0.49670516047468893 AS float))) AS tab_2",
            //"SELECT CAST(CAST(26884955 AS int) AS int) INTERSECT SELECT view_537.col_1111 FROM view_537 WHERE NOT ((view_537.col_1111 > CAST(- 406118011 AS int)) <= ((CAST(182393886938628546 AS bigint) % CAST(- 7505760285872234247 AS bigint)) >= CAST(6268575506156459773 AS bigint)));",
            //"SELECT tab_111.col_1 FROM tab_1 AS tab_111 FULL JOIN tab_1 AS tab_112 ON (CAST(- 2474187877618617778 AS bigint) = CAST(2715953958593069068 AS bigint)) RIGHT JOIN tab_1 AS tab_113 ON (tab_113.col_1 > CAST('' AS text));",
            "select limit 1",
            "SELECT
    *
FROM
    [u123 AS materialize.public.foo];",
        ];

        for stmt in stmts {
            println!("\n-------------\n");
            let ast = parse_statements(stmt).unwrap().into_iter().next().unwrap();
            let mut n = 1;
            let mut last = "".to_string();
            loop {
                let s = to_pretty(&ast, n);
                if s != last {
                    println!("{}:\n{}\n", n, s);
                    last = s;
                }
                n += 1;
                if n > stmt.len() {
                    break;
                }
            }
        }
    }
}
