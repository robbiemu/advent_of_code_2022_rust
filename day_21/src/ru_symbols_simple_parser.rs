use chumsky::prelude::*;
use rusymbols::Expression;

use Token::*;


#[derive(Clone, Debug)]
enum Token {
  Num(Expression),
  Var(Expression),
  Add,
  Div,
  Eq,
  Mul,
  Sub,
  Parens(Vec<Self>),
  Noop,
  Invalid,
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
      any()
        .filter(|c: &char| c.is_ascii_lowercase())
        .map(|c| Var(Expression::new_var(c.to_string().as_str()))),
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
        .map(|n| Num(Expression::new_val(n.parse().unwrap()))),
    ))
    .or(input.delimited_by(just('('), just(')')).map(Parens))
    .recover_with(via_parser(nested_delimiters('(', ')', [], |_| Invalid)))
    .repeated()
    .collect()
  })
}

fn token_to_simple_expression(token: &Token) -> Option<Expression> {
  match token {
    Num(n) => Some(n.to_owned()),
    Var(c) => Some(c.to_owned()),
    Parens(tokens) => token_to_expression(tokens),
    _ => None,
  }
}

fn token_to_expression(tokens: &[Token]) -> Option<Expression> {
  if tokens.is_empty() {
    return None;
  }

  let mut index = 0;
  while matches!(tokens[index], Noop) {
    index += 1;
  }
  let mut result = token_to_simple_expression(&tokens[index])?;

  while index + 1 < tokens.len() {
    match tokens[index + 1].to_owned() {
      Num(n) => n.clone(),
      Var(a) => a.clone(),
      Add => {
        let (offset, rhs) = get_rhs(tokens, index)?;
        index += offset;

        result = result + rhs;
        continue;
      }
      Sub => {
        let (offset, rhs) = get_rhs(tokens, index)?;
        index += offset;

        result = result - rhs;
        continue;
      }
      Mul => {
        let (offset, rhs) = get_rhs(tokens, index)?;
        index += offset;

        result = result * rhs;
        continue;
      }
      Div => {
        let (offset, rhs) = get_rhs(tokens, index)?;
        index += offset;

        result = result / rhs;
        continue;
      }
      Eq => {
        let (offset, rhs) = get_rhs(tokens, index)?;
        index += offset;

        result = result - rhs;
        continue;
      }
      Noop => {
        index += 1;
        continue;
      }
      _ => break,
    };
  }

  Some(result)
}

fn get_rhs(tokens: &[Token], index: usize) -> Option<(usize, Expression)> {
  let mut offset = 2;
  while matches!(tokens[index + offset], Noop) {
    offset += 1;
  }
  let rhs = token_to_simple_expression(&tokens[index + offset])?;

  Some((offset, rhs))
}

fn input_to_tokens(input: &str) -> Vec<Token> {
  let tokens = tokenizer().parse(input);
  match tokens.into_result() {
    Ok(vec_tokens) => vec_tokens,
    _ => vec![Invalid],
  }
}

// mark - exported function
pub fn convert_equation_to_expression(input: &str) -> Option<Expression> {
  let mut ast = input_to_tokens(input);
  ast = factor_out_variable(ast);
  // dbg!(&ast);
  token_to_expression(&ast)
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


fn factor_out_variable(tokens: Vec<Token>) -> Vec<Token> {
  let mut stack = vec![tokens; 1];
  let mut lhs = vec![Num(Expression::new_val(0.))];
  let mut reverse_op = Noop;
  let mut available_token = Noop;

  while let Some(tokens) = stack.pop() {
    for token in tokens {
      let token_clone = token.clone();
      match token {
        Parens(inner_tokens) => {
          if includes_variable(&inner_tokens) {
            stack.push(inner_tokens)
          } else {
            available_token = token_clone;
            if !matches!(reverse_op, Noop) {
              load_fragment(&mut lhs, &mut reverse_op, &mut available_token);
            }
          }
        }
        Add | Mul | Sub | Div | Eq => {
          reverse_op = reverse_operation(token);
          if !matches!(available_token, Noop) {
            load_fragment(&mut lhs, &mut reverse_op, &mut available_token);
          }
        }
        Num(_) => {
          available_token = token;
          if !matches!(reverse_op, Noop) {
            load_fragment(&mut lhs, &mut reverse_op, &mut available_token);
          }
        }
        _ => (),
      }
    }
  }

  lhs
}

fn load_fragment(lhs: &mut Vec<Token>, op: &mut Token, token: &mut Token) {
  let new_token = {
    let last: Vec<Token> = lhs.drain(0..).collect();
    let up_til = Parens(last);

    vec![Parens(vec![up_til, op.clone(), token.clone()])]
  };

  lhs.extend(new_token);

  *op = Noop;
  *token = Noop;
}
