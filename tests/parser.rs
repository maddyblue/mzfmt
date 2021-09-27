use mzfmt::*;

// An ignored test to compare mz's parser tests so we can ensure
// round-trippness and line squashing. This will munge the mz tests.
// Run with:
// REWRITE=1 cargo test -- --nocapture --ignored
#[test]
#[ignore]
fn parser() {
    use datadriven::walk;
    use sql::ast::display::AstDisplay;
    use sql_parser::parser::parse_statements;

    walk(
        "/home/mjibson/materialize/src/sql-parser/tests/testdata",
        |f| {
            f.run(|tc| -> String {
                match tc.directive.as_str() {
                    "parse-statement" => {
                        let stmt1 = match parse_statements(&tc.input) {
                            Ok(stmt) => match stmt.into_iter().next() {
                                Some(stmt) => stmt,
                                None => return "".to_string(),
                            },
                            Err(_) => return "".to_string(),
                        };
                        let mut res = Vec::new();
                        for n in &[1, 40, 1000000] {
                            let n = *n;
                            let pretty1 = to_pretty(&stmt1, n);
                            let stmt2 = parse_statements(&pretty1)
                                .unwrap()
                                .into_iter()
                                .next()
                                .unwrap();
                            let pretty2 = to_pretty(&stmt2, n);
                            assert_eq!(pretty1, pretty2);
                            assert_eq!(stmt1.to_ast_string_stable(), stmt2.to_ast_string_stable());
                            // Everything should always squash to a single line.
                            if n > (tc.input.len() * 2) {
                                assert_eq!(pretty1.lines().count(), 1, "{}: {}", n, pretty1);
                            }
                            res.push(format!("{}: {}", n, pretty1));
                        }
                        format!("{}\n", res.join("\n"))
                    }
                    _ => "".to_string(),
                }
            })
        },
    );
}
