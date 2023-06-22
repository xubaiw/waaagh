use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{char, one_of},
    combinator::eof,
    multi::{many0, many1, separated_list0},
    IResult,
};

#[allow(unused_must_use)]
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = args.get(1).expect("Should give path as argument");
    let text = std::fs::read_to_string(&path).expect("Fail to read file");
    let doc = doc(&text);
    dbg!(doc);
}

#[derive(Debug, PartialEq)]
enum SExpr {
    Node(String),
    Leaf(Vec<SExpr>),
}

fn doc(i: &str) -> IResult<&str, Vec<SExpr>> {
    let (i, _) = ws0(i)?;
    let (i, exprs) = separated_list0(ws0, leaf)(i)?;
    let (i, _) = ws0(i)?;
    let (i, _) = eof(i)?;
    Ok((i, exprs))
}

fn s_expr(i: &str) -> IResult<&str, SExpr> {
    alt((node, leaf))(i)
}

fn node(i: &str) -> IResult<&str, SExpr> {
    let (i, n) = alt((quoted_node, unquoted_node))(i)?;
    Ok((i, SExpr::Node(n)))
}

fn leaf(i: &str) -> IResult<&str, SExpr> {
    let (i, _) = char('(')(i)?;
    let (i, _) = ws0(i)?;
    let (i, nodes) = separated_list0(ws1, s_expr)(i)?;
    let (i, _) = ws0(i)?;
    let (i, _) = char(')')(i)?;
    Ok((i, SExpr::Leaf(nodes)))
}

fn quoted_node(i: &str) -> IResult<&str, String> {
    let (i, _) = char('"')(i)?;
    let (i, b) = is_not("\"")(i)?;
    let (i, _) = char('"')(i)?;
    Ok((i, format!("\"{b}\"")))
}

/// Unquoted node allow all non-space character
fn unquoted_node(i: &str) -> IResult<&str, String> {
    let (i, n) = is_not(" ()\r\n\t\"")(i)?;
    Ok((i, n.to_owned()))
}

fn ws0(i: &str) -> IResult<&str, ()> {
    let (i, _) = many0(one_of(" \r\n\t"))(i)?;
    Ok((i, ()))
}

fn ws1(i: &str) -> IResult<&str, ()> {
    let (i, _) = many1(one_of(" \r\n\t"))(i)?;
    Ok((i, ()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sexpr_var() {
        let text = "(def foo 233)";
        s_expr(text).unwrap();
    }

    #[test]
    fn test_sexpr_function() {
        let text = "(def foo (fn (x) (i32-add x 1)))";
        s_expr(text).unwrap();
    }

    #[test]
    fn test_sexpr_more_space() {
        let text = "( def  foo  ( fn  ( x )  ( i32-add  x  1 ) ) )";
        s_expr(text).unwrap();
    }

    #[test]
    fn test_sexpr_multiline() {
        let text = "(def foo
        (fn (x)
            (i32-add x 1)))";
        s_expr(text).unwrap();
    }
}
