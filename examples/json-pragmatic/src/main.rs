//! An examples of **Vehicle Routing Problem** solver usage.

#![forbid(unsafe_code)]

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::sync::Arc;
use vrp_pragmatic::checker::CheckerContext;
use vrp_pragmatic::core::models::{Problem as CoreProblem, Solution as CoreSolution};
use vrp_pragmatic::core::prelude::*;
use vrp_pragmatic::format::problem::{deserialize_matrix, deserialize_problem, Matrix, PragmaticProblem, Problem};
use vrp_pragmatic::format::solution::{deserialize_solution, write_pragmatic, Solution};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let base_path = args.get(1).expect("please set a proper path to example data");
    run_examples(base_path.as_str());
}

fn run_examples(base_path: &str) {
    let names: Vec<(_, Option<Vec<&str>>)> = vec![
        ("basics/break.basic", None),
        ("basics/multi-day.basic", None),
        ("basics/multi-job.basic", None),
        ("basics/multi-job.mixed", None),
        ("basics/multi-objective.balance-load", None),
        ("basics/multi-objective.default", None),
        ("basics/multi-objective.maximize-value", None),
        ("basics/priorities.value", None),
        ("basics/profiles.basic", Some(vec!["basics/profiles.basic.matrix.car", "basics/profiles.basic.matrix.truck"])),
        ("basics/relation-strict.basic", None),
        ("basics/relation-any.basic", None),
        ("basics/reload.basic", None),
        ("basics/reload.multi", None),
        ("basics/skills.basic", None),
        ("basics/unassigned.unreachable", None),
        ("simple.basic", None),
        ("simple.index", Some(vec!["simple.basic.matrix"])),
        ("clustering/berlin.vicinity-continue", Some(vec!["clustering/berlin.vicinity-continue.matrix"])),
        ("clustering/berlin.vicinity-return", Some(vec!["clustering/berlin.vicinity-return.matrix"])),
        ("objectives/berlin.default", None),
        ("objectives/berlin.balance-activities", None),
        ("objectives/berlin.balance-distance", None),
        ("objectives/berlin.balance-max-load", None),
    ];

    for (name, matrices) in names {
        let problem = get_pragmatic_problem(base_path, name);

        let (core_problem, problem, matrices) = if let Some(matrices) = matrices {
            let matrices = matrices
                .iter()
                .map(|path| deserialize_matrix(open_file(format!["{base_path}/{path}.json"].as_str())))
                .collect::<Result<Vec<Matrix>, _>>()
                .unwrap_or_else(|errors| panic!("cannot read matrix: {errors}"));
            ((problem.clone(), matrices.clone()).read_pragmatic(), problem, Some(matrices))
        } else {
            (problem.clone().read_pragmatic(), problem, None)
        };

        let core_problem =
            Arc::new(core_problem.unwrap_or_else(|errors| panic!("cannot read pragmatic problem: {errors}")));

        let config = VrpConfigBuilder::new(core_problem.clone())
            .prebuild()
            .expect("cannot prebuild vrp configuration")
            .with_max_generations(Some(100))
            .build()
            .unwrap_or_else(|err| panic!("cannot build default solver configuration: {err}"));
        let solution = Solver::new(core_problem.clone(), config)
            .solve()
            .unwrap_or_else(|err| panic!("cannot solver problem: {err}"));

        let solution = get_pragmatic_solution(&core_problem, &solution);

        if let Err(errs) = CheckerContext::new(core_problem, problem, matrices, solution).and_then(|ctx| ctx.check()) {
            panic!("unfeasible solution in '{}':\n'{}'", name, GenericError::join_many(&errs, "\n"));
        }
    }
}

fn open_file(path: &str) -> BufReader<File> {
    println!("reading '{path}'");
    BufReader::new(File::open(path).unwrap_or_else(|err| panic!("cannot open {path} file: '{err}'")))
}

fn get_pragmatic_problem(base_path: &str, name: &str) -> Problem {
    deserialize_problem(open_file(format!["{base_path}/{name}.problem.json"].as_str())).unwrap()
}

fn get_pragmatic_solution(problem: &CoreProblem, solution: &CoreSolution) -> Solution {
    let output_type = Default::default();
    let mut writer = BufWriter::new(Vec::new());

    write_pragmatic(problem, solution, output_type, &mut writer).expect("cannot write pragmatic solution");
    let bytes = writer.into_inner().expect("cannot get bytes from writer");

    deserialize_solution(BufReader::new(bytes.as_slice())).expect("cannot deserialize solution")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_run_examples() {
        run_examples("../data/pragmatic");
    }
}
