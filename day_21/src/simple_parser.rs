use evalexpr::*;
use regex::Regex;

pub fn resolve_equation(input: &str) -> Option<i64> {
  let (left, right) = input.split_once('=').unwrap();

  let mut expression = eval_int(right).ok()?.to_string();

  let reducer = Regex::new(r"\(\d+\s[\+\-\*\/]\s\d+\)").ok()?;
  let mut source = left.to_string();
  while let Some(caps) = reducer.captures(&source) {
    let matched_expression = caps.get(0).unwrap().as_str();
    let evaluated_value = eval_int(matched_expression).ok().unwrap();
    source = source.replace(matched_expression, &evaluated_value.to_string());
  }
  // Now every pair of parentheses left in source encapsulates all other sub-expressions
  let right_reduce =
    Regex::new(r"^\s*\((.*)\s*([\+\-\*\/])\s*(-?\d+)\)\s*$").unwrap();
  let left_reduce =
    Regex::new(r"^\s*\((-?\d+)\s*([\+\-\*\/])\s*(.*)\)\s*$").unwrap();
  let trim_parens = Regex::new(r"^\s*\((.*)\)*\s*$").unwrap();

  let mut stack = vec![source];
  while let Some(source) = stack.pop() {
    if source.trim() == "a" {
      break;
    }
    eprintln!("evaluating {source}");

    if let Some(capture_group) = right_reduce.captures(source.as_str()) {
      let lhs = capture_group.get(1).map_or("", |m| m.as_str());
      let op = capture_group.get(2).map_or("", |m| m.as_str());
      let rhs = capture_group.get(3).map_or("", |m| m.as_str());
      dbg!(lhs, op, rhs);

      expression = match reverse_op(op) {
        "+" | "-" => {
          format!("{expression} {} {}", reverse_op(op), &rhs)
        }
        "*" | "/" => {
          format!("({expression}) {} {}", reverse_op(op), &rhs)
        }
        _ => unimplemented!(),
      };

      stack.push(lhs.to_string());
    } else if let Some(capture_group) = left_reduce.captures(source.as_str()) {
      let lhs = capture_group.get(1).map_or("", |m| m.as_str());
      let op = capture_group.get(2).map_or("", |m| m.as_str());
      let rhs = capture_group.get(3).map_or("", |m| m.as_str());
      dbg!(lhs, op, rhs);

      match op {
        "+" => {
          expression = format!("{expression} {} {}", reverse_op(op), &lhs);
          stack.push(rhs.to_string());
        }
        "-" => {
          expression = format!("{expression} - {lhs}");
          stack.push("(-1 * ".to_string() + rhs + ")");
        }
        "*" => {
          expression = format!("({expression}) {} {}", reverse_op(op), &lhs);
          stack.push(rhs.to_string());
        }
        "/" => {
          expression = format!("{lhs} / ({expression})");
          stack.push(rhs.to_string());
        }
        _ => unimplemented!(),
      }
    } else {
      let Some(capture_group) = trim_parens.captures(source.as_str()) else {
      unreachable!();
    };
      let inner = capture_group.get(1).map_or("", |m| m.as_str());
      stack.push(inner.to_string());
    }
  }

  dbg!(&expression);


  eval_int(expression.as_str()).ok()
}

fn reverse_op(op: &str) -> &str {
  match op {
    "+" => "-",
    "-" => "+",
    "*" => "/",
    "/" => "*",
    _ => unimplemented!(),
  }
}
