pub mod token;

#[derive(Parser)]
#[grammar = "rst.pest"]
pub struct RstParser;



#[test]
fn line() {
    use pest::Parser;
    let result = RstParser::parse(Rule::plain, &"line\n").expect("unsuccessful parse").next().unwrap();
    eprintln!("{}", result);
}

#[test]
fn title() {
    use pest::Parser;
    let result = RstParser::parse(Rule::heading, &"\
Title
=====
").expect("unsuccessful parse").next().unwrap();
    eprintln!("{}", result);
}

#[test]
fn heading_title() {
    use pest::Parser;
    let result = RstParser::parse(Rule::heading_title, &"\
-----
Title
-----
").expect("unsuccessful parse").next().unwrap();
    eprintln!("{}", result);
}
