use mz_sql_parser::ast::display::{AstDisplay, ToDoc};
use mz_sql_parser::ast::*;
use mz_sql_parser::parser::{parse_statements, ParserError};

pub fn to_pretty(stmt: &Statement<Raw>, width: usize) -> String {
    let mut w = Vec::new();
    verify(stmt);
    stmt.to_doc().render(width, &mut w).unwrap();
    let mut s = String::from_utf8(w).unwrap();
    s.push(';');
    s
}

// Panics if stmt's to_doc and to_ast_string differ when re-parsed.
pub fn verify(stmt: &Statement<Raw>) {
    fn pdoc<T: mz_sql_parser::ast::display::ToDoc>(t: &T) -> Vec<String> {
        let doc = t.to_doc();
        [10000, 0]
            .map(|i| {
                let mut cur = Vec::new();
                doc.render(i, &mut cur).unwrap();
                String::from_utf8(cur).unwrap()
            })
            .to_vec()
    }

    let from_ast_display = &parse_statements(&stmt.to_ast_string_stable()).unwrap()[0];
    for doc in pdoc(from_ast_display) {
        let n = &parse_statements(&doc).unwrap()[0];
        assert_eq!(n, from_ast_display, "doc: {doc}, orig: {from_ast_display}");
    }
}

pub fn pretty_strs(str: &str, width: usize) -> Result<Vec<String>, ParserError> {
    let stmts = parse_statements(str)?;
    Ok(stmts.iter().map(|s| to_pretty(s, width)).collect())
}

pub fn pretty_str(str: &str, width: usize) -> Result<String, ParserError> {
    Ok(pretty_strs(str, width)?.join("\n\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mz_sql_parser::parser::parse_statements;

    #[test]
    fn pretty() {
        let stmts = vec![
            "with a as (select 'blah', 'another string') select 1, 2, 3 from a, b, c where a = b group by c, d having 1 = 4 AND a < c order by a limit 1 offset 2 rows",
            //"with a as (select 'blah', 'another string') select 1 from a",
            //"select 1 union select 2",
            //"insert into t (a,b,c) values (1,2,3), (4,5,6)",
            //"CREATE VIEW view_1 (col_2) AS SELECT * FROM (VALUES (CAST(0.9841192240680561 AS float)), (CAST(0.37823189648731315 AS float)), (CAST(0.8390174385199045 AS float)), (CAST(0.22188376517105302 AS float)), (CAST(0.5787854533815643 AS float)), (CAST(0.7234205380688273 AS float)), (CAST(0.39567191795118384 AS float)), (CAST(0.4348712893998896 AS float)), (CAST(0.8856762904388714 AS float)), (CAST(0.7704453261942663 AS float)), (CAST(0.5133022896871524 AS float)), (CAST(NULL AS float)), (CAST(0.5170637540787644 AS float)), (CAST(0.6762831752745486 AS float)), (CAST(0.2424964369655378 AS float)), (CAST(0.031422253928415134 AS float)), (CAST(0.7791437964022883 AS float)), (CAST(0.7976069716256476 AS float)), (CAST(0.49670516047468893 AS float))) AS tab_2",
            //"SELECT CAST(CAST(26884955 AS int) AS int) INTERSECT SELECT view_537.col_1111 FROM view_537 WHERE NOT ((view_537.col_1111 > CAST(- 406118011 AS int)) <= ((CAST(182393886938628546 AS bigint) % CAST(- 7505760285872234247 AS bigint)) >= CAST(6268575506156459773 AS bigint)));",
            //"SELECT tab_111.col_1 FROM tab_1 AS tab_111 FULL JOIN tab_1 AS tab_112 ON (CAST(- 2474187877618617778 AS bigint) = CAST(2715953958593069068 AS bigint)) RIGHT JOIN tab_1 AS tab_113 ON (tab_113.col_1 > CAST('' AS text));",
            //"select limit 1",
            //"SELECT * FROM [u123 AS materialize.public.foo];",
            //"select 1+2 as eeeee, *",
            /*
                        "SELECT
                origins.id,
                (SELECT count(*) FROM events WHERE ((payload ->> 'foo_865' = '843' OR payload ->> 'bar_449' = '658' OR payload ->> 'qux_600' = '583') AND mz_logical_timestamp() BETWEEN (1000 * date_part('epoch', timestamp_col)::numeric) AND (1000 * date_part('epoch', timestamp_col + INTERVAL '30 days')::numeric) AND category_id = origins.category_id AND origin_id = origins.id))
                    AS foo__m4__count_30_days,
                (SELECT max(timestamp_col) FROM events WHERE ((payload ->> 'foo_573' = '43' OR payload ->> 'bar_727' = '631' OR payload ->> 'qux_976' = '262') AND category_id = origins.category_id AND origin_id = origins.id))
                    AS foo__m19__max_time
                    FROM
                origins
                    JOIN
                        categories
                        ON categories.id = origins.category_id
            WHERE categories.type = 'foo';",
                    */
            // "select blag(1, 2, 'a')",
            //  "select 1,2, blag('a', 'b')",
            "SELECT max(timestamp_col)",
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
                if n > (stmt.len() + 5) {
                    break;
                }
                //break;
            }
        }
    }
}
