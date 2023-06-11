pub trait ProblemSolver {
  type Input;
  type Solution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input;
  fn solve(input: Self::Input) -> Self::Solution;
  fn output(solution: Self::Solution) -> String;
}

pub fn solve_problem<T: ProblemSolver>(payload: String) -> String {
  let lines = payload.lines().map(|line| line.to_owned());

  let input = T::initialize(lines);
  let solution = T::solve(input);

  T::output(solution)
}
