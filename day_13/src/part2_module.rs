use super::common::*;
use super::problem_solver::ProblemSolver;


const TWO: &str = "[[2]]";
const SIX: &str = "[[6]]";

pub struct PSInput {
  packets: Vec<String>,
}

pub struct PSSolution {
  ordered_packets: Vec<String>,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let packets: Vec<String> = lines.filter(|l| !l.is_empty()).collect();

    Self::Input { packets }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let mut ordered_packets = input.packets;
    ordered_packets.extend_from_slice(&[TWO.to_string(), SIX.to_string()]);
    ordered_packets
      .sort_by(|left, right| compare(left.as_bytes(), right.as_bytes()));
    Self::Solution { ordered_packets }
  }

  fn output(solution: Self::Solution) {
    let two_packet = solution
      .ordered_packets
      .iter()
      .position(|s| s == TWO)
      .unwrap()
      + 1;
    let six_packet = solution
      .ordered_packets
      .iter()
      .position(|s| s == SIX)
      .unwrap()
      + 1;
    println!("decoder_key: {}", two_packet * six_packet);
  }
}
