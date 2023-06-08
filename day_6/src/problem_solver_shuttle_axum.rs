pub struct ProblemContexts<Ctx> {
  pub initialize: Option<Ctx>,
  pub solve: Option<Ctx>,
  pub output: Option<Ctx>,
}

impl<Ctx> Default for ProblemContexts<Ctx> {
  fn default() -> Self {
      ProblemContexts {
          initialize: None,
          solve: None,
          output: None,
      }
  }
}

pub trait ProblemSolver {
  type Input;
  type Solution;
  type Context;

  fn initialize(
      lines: impl Iterator<Item = String>,
      context: Option<Self::Context>,
  ) -> Self::Input;
  fn solve(input: Self::Input, context: Option<Self::Context>) -> Self::Solution;
  fn output(solution: Self::Solution, context: Option<Self::Context>) -> String;
}

pub fn solve_problem<T: ProblemSolver>(payload: String, problem_contexts: Option<ProblemContexts<T::Context>>) -> String {
  let lines = payload.lines().map(|line| line.to_owned());
  let default_contexts = ProblemContexts::default();
  let contexts = problem_contexts.unwrap_or(default_contexts);

  let input = T::initialize(lines, contexts.initialize);
  let solution = T::solve(input, contexts.solve);
  let result = T::output(solution, contexts.output);

  result
}
