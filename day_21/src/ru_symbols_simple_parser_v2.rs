use chumsky::prelude::*;
use rusymbols::Expression;
use std::{
  fmt,
  fmt::{Display, Formatter},
};

use Token::*;


#[derive(Clone, Debug)]
enum Token {
  Num(f64),
  Var(char),
  Add,
  Div,
  Eq,
  Mul,
  Sub,
  Parens(Vec<Self>),
  Noop,
  Invalid,
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Token::Num(expr) => write!(f, "{}", expr),
      Token::Var(expr) => write!(f, "{}", expr),
      Token::Add => write!(f, "+"),
      Token::Div => write!(f, "/"),
      Token::Eq => write!(f, "="),
      Token::Mul => write!(f, "*"),
      Token::Sub => write!(f, "-"),
      Token::Parens(tokens) => {
        write!(
          f,
          "({})",
          tokens
            .iter()
            .map(|token| token.to_string())
            .collect::<Vec<_>>()
            .join("")
        )
      }
      Token::Noop => write!(f, " "),
      Token::Invalid => write!(f, "INVALID"),
    }
  }
}

struct TokenVecWrapper(Vec<Token>);

impl Display for TokenVecWrapper {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      self
        .0
        .iter()
        .map(|token| token.to_string())
        .collect::<Vec<_>>()
        .join(" ")
    )
  }
}

impl From<Vec<Token>> for TokenVecWrapper {
  fn from(tokens: Vec<Token>) -> Self {
    TokenVecWrapper(tokens)
  }
}

pub fn convert_equation_to_expression(input: &str) -> Option<Expression> {
  let mut ast = input_to_tokens(input);
  ast = factor_out_variable(ast);
  eprintln!("factored ast {}", TokenVecWrapper(ast.clone()));
  tokens_to_expression(&ast)
}

fn input_to_tokens(input: &str) -> Vec<Token> {
  let tokens = tokenizer().parse(input);
  match tokens.into_result() {
    Ok(vec_tokens) => vec_tokens,
    _ => vec![Invalid],
  }
}

fn tokenizer<'a>(
) -> impl Parser<'a, &'a str, Vec<Token>, extra::Err<Simple<'a, char>>> {
  recursive(|input| {
    choice((
      just('+').to(Add),
      just('/').to(Div),
      just('=').to(Eq),
      just('*').to(Mul),
      just('-').to(Sub),
      any().filter(|c: &char| c.is_ascii_lowercase()).map(Var),
      any()
        .filter(|c: &char| c.is_whitespace())
        .repeated()
        .at_least(1)
        .collect::<String>()
        .to(Noop),
      any()
        .filter(|c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|n| Num(n.parse().unwrap())),
    ))
    .or(input.delimited_by(just('('), just(')')).map(Parens))
    .recover_with(via_parser(nested_delimiters('(', ')', [], |_| Invalid)))
    .repeated()
    .collect()
  })
}

fn token_to_simple_expression(token: &Token) -> Option<Expression> {
  match token {
    Num(n) => Some(Expression::new_val(n.to_owned())),
    Var(c) => Some(Expression::new_var(c.to_string().as_str())),
    Parens(tokens) => tokens_to_expression(tokens),
    _ => None,
  }
}

fn apply_op_to_expression(
  op: Token,
  lhs: Expression,
  rhs: Expression,
) -> Option<Expression> {
  match op {
    Add => Some(lhs + rhs),
    Div => Some(lhs / rhs),
    Mul => Some(lhs * rhs),
    Sub => Some(lhs - rhs),
    _ => None,
  }
}

fn get_rhs(tokens: &[Token], index: usize) -> Option<(usize, Expression)> {
  let mut offset = 2;
  while matches!(tokens[index + offset], Noop) {
    offset += 1;
  }
  let rhs = token_to_simple_expression(&tokens[index + offset])?;

  Some((offset, rhs))
}

fn tokens_to_expression(tokens: &[Token]) -> Option<Expression> {
  if tokens.is_empty() {
    return None;
  }

  let mut index = 0;
  let mut expression = token_to_simple_expression(&tokens[index])?;

  while index + 1 < tokens.len() {
    let op = tokens[index + 1].to_owned();
    match op {
      Add | Div | Mul | Sub => {
        let (offset, rhs) = get_rhs(tokens, index)?;
        index += offset;

        expression = apply_op_to_expression(op.clone(), expression, rhs)?;
      }
      Eq => {
        let (offset, rhs) = get_rhs(tokens, index)?;
        index += offset;

        expression = apply_op_to_expression(Sub, expression, rhs)?;
      }
      Noop => {
        index += 1;
      }
      _ => return None,
    };
  }

  Some(expression)
}

fn reverse_operation(op: Token) -> Token {
  match op {
    Add => Sub,
    Sub => Add,
    Mul => Div,
    Div => Mul,
    Eq => Add,
    _ => op,
  }
}

fn includes_variable(tokens: &[Token]) -> bool {
  tokens.iter().any(|token| match token {
    Var(_) => true,
    Parens(inner_tokens) => includes_variable(inner_tokens),
    _ => false,
  })
}

enum FragmentOrder {
  Postfix,
  Prefix,
}

use FragmentOrder::*;


fn load_fragment(
  lhs: &mut Vec<Token>,
  op: &mut Token,
  token: &mut Token,
  order: FragmentOrder,
) {
  let last: Vec<Token> = lhs.drain(0..).collect();
  let new_token = {
    let up_til = if last.len() == 1 || matches!(op, Add | Sub) {
      if let Parens(v) = last[0].clone() {
        v
      } else {
        vec![last[0].to_owned()]
      }
    } else {
      vec![Parens(last)]
    };

    match order {
      Postfix => vec![Parens(
        vec![up_til, vec![op.clone(), token.clone()]].concat(),
      )],
      Prefix => vec![Parens(vec![vec![token.clone(), op.clone()]].concat())],
    }
  };

  lhs.extend(new_token);

  *op = Noop;
  *token = Noop;
}

fn factor_out_variable(tokens: Vec<Token>) -> Vec<Token> {
  let mut stack = vec![tokens; 1];
  let mut lhs = vec![Num(0.)];
  let mut reverse_op = Noop;
  let mut available_token = Noop;

  while let Some(tokens) = stack.pop() {
    let tokens_clone = tokens.clone();
    for (i, token) in tokens.into_iter().enumerate() {
      let token_clone = token.clone();
      match token {
        Add | Mul | Eq => {
          reverse_op = reverse_operation(token.to_owned());
          if !matches!(available_token, Noop) {
            load_fragment(
              &mut lhs,
              &mut reverse_op,
              &mut available_token,
              Postfix,
            );
          }
        }
        Sub => {
          reverse_op = reverse_operation(token.to_owned());
          if !matches!(available_token, Noop) {
            eprintln!("sub with token {}", available_token);
            load_fragment(&mut lhs, &mut Sub, &mut available_token, Postfix);
            let restack =
              vec![Num(-1.), Mul, Parens(tokens_clone[i + 1..].to_vec())];
            stack.extend(vec![restack]);
            break;
          }
        }
        Div => {
          reverse_op = reverse_operation(token.to_owned());
          if !matches!(available_token, Noop) {
            eprintln!("div with token {}", available_token);
            load_fragment(&mut lhs, &mut Add, &mut available_token, Prefix);
          }
        }
        Num(_) => {
          available_token = token.to_owned();
          if !matches!(reverse_op, Noop) {
            load_fragment(
              &mut lhs,
              &mut reverse_op,
              &mut available_token,
              Postfix,
            );
          }
        }
        Parens(inner_tokens) => {
          if includes_variable(&inner_tokens) {
            stack.push(inner_tokens)
          } else {
            available_token = token_clone;
            if !matches!(reverse_op, Noop) {
              load_fragment(
                &mut lhs,
                &mut reverse_op,
                &mut available_token,
                Postfix,
              );
            }
          }
        }
        _ => (),
      }
    }
  }

  lhs
}
