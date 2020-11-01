extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct WordProblem;

fn compute(sum: i32, pair: Pair<Rule>) -> Option<i32> {
    match pair.as_rule() {
        Rule::num => pair.as_str().parse::<i32>().ok(),
        Rule::plus => Some(sum + compute(sum, pair.into_inner().next().unwrap())?),
        Rule::minus => Some(sum - compute(sum, pair.into_inner().next().unwrap())?),
        Rule::multiplied => Some(sum * compute(sum, pair.into_inner().next().unwrap())?),
        Rule::divided => Some(sum / compute(sum, pair.into_inner().next().unwrap())?),
        Rule::exponent => Some(sum.pow(compute(sum, pair.into_inner().next().unwrap())? as u32)),
        Rule::expr => compute(sum, pair.into_inner().next().unwrap()),
        _ => None,
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let mut result = WordProblem::parse(Rule::stmt, command).ok()?;

    let top_rule = result.next().unwrap();
    let mut inner = top_rule.into_inner();
    let mut total = inner.next().unwrap().as_str().parse::<i32>().ok()?;

    for expr in inner {
        total = compute(total, expr)?;
    }

    Some(total)
}
