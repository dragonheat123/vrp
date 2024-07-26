# Heuristics

This page provides some high level overview of general heuristic used to solve VRP. Some information can be found on
vrp-core crate's [documentation page](https://docs.rs/vrp-core/latest/vrp_core/solver/index.html)


## Starting from scratch: constructive heuristics

To build initial solutions to start with, the solver internally can use different built-in constructive heuristics, such as:

- variation of Clark & Wright Savings algorithm
- regret insertion
- insertion with blinks
- nearest neighbor
- random insertions
- etc.

[Related documentation](https://docs.rs/vrp-core/latest/vrp_core/construction/heuristics/index.html)

To support faster evaluation of jobs insertion in large tours, insertion algorithm uses a search optimization to prevent
greedy evaluation. This is useful in case of large scale VRPs when greedy evaluation significantly increases running time.

[Related documentation](https://docs.rs/vrp-core/latest/vrp_core/utils/trait.SelectionSamplingSearch.html)

Typically, the solver builds four initial solutions, then they are memorized as initial state by the one of the population algorithms:

- `greedy`: only the best solution is kept
- `elitism`: n best solutions are kept using some diversification criteria
- `rosomaxa`: a custom population-based algorithm which focuses on improving exploration/exploitation ratio.

The latter is default, however, others can be used if amount of available CPU is low.

[Related documentation](https://docs.rs/rosomaxa/latest/rosomaxa/population/index.html)

## Searching for better solution: meta heuristics

The goal of metaheuristic (or just heuristic for simplicity) is to refine one (or many) of the known solutions.
Currently available heuristics:

- `ruin and recreate` principle (Adaptive Large Neighborhood Search): ruin parts of solution and recreates them. Key ideas:
  - use multiple ruin/recreate methods and combine them differently
  - make a larger moves in solution space
- `local search`: use different local search operators. The main difference from R&R:
  - avoids making big steps in a solution space
  - target to improve specific aspects in solution
- `explorative heuristics`: these can be seen as generators for explorative moves in solutions space:
  - `redistribute search`: removes jobs from specific route and prevents their insertion back to it
  - `infeasible search`: allows constraint violations to explore infeasible solutions space. It has recovery step
     to move back to feasible space.
- `decomposition search` (some kind of Divide and Conquer algorithm): splits existing solution into multiple smaller ones
   (e.g. not more than 2-4 routes) and tries to improve them in isolation. Typically, it uses all heuristics just mentioned.

Each heuristic accepts one of solutions from the population (not necessary the best known) and tries to improve it (or diversify).
During one of refinement iterations, many solutions are picked at the same time and many heuristics are called then in parallel.
Such incremental step is called a `generation`. Once it is completed, all found solutions are introduced to the population,
which decides how to store them.

[Related documentation](https://docs.rs/vrp-core/latest/vrp_core/solver/search/index.html)


## What heuristic to pick: hyper heuristic

As the solver is not limited to one heuristic, there is a problem: what is the best strategy to pick one of the pre-defined
heuristics at given search state? To solve that problem, there are two heuristics are available:

- `static selective`: associate with every heuristic some probability weight and use it to decide which one to pick next
- `dynamic selective`: try to learn probability dynamically based on search progression over time.

The latter is used by default.

[Related documentation](https://docs.rs/rosomaxa/latest/rosomaxa/hyper/index.html)


## When to stop: termination criteria

The search is terminated and the best known solution is returned when some termination criteria is met. The following
termination criteria are supported:

- `time`: stop after some specified amount of seconds
- `generation`: stop after some specified amount of generations
- `coefficient of variation`: stop if there is no `significant` improvement in specific time or amount of generations
- `user interrupted` from command line, e.g. by pressing Ctrl + C

Interruption when building initial solutions is supported. Default is 300 seconds or 3000 generations max.

[Related documentation](https://docs.rs/rosomaxa/latest/rosomaxa/termination/index.html)
