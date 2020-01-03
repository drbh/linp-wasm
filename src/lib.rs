mod utils;
extern crate rulp;

use rulp::builder::{Builder, BuilderBase};
use rulp::parser::{Parser, ParserBase};
use rulp::solver::{SimplexSolver, SolverBase};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, linp-wasm!");
}

#[wasm_bindgen]
pub fn execute(text_problem: String) -> String {
    let builder = Builder::new();
    let lp = Parser::lp_from_text(&text_problem, builder);
    println!("{}", lp);
    let solver = SimplexSolver::new(lp.clone());
    let solution = solver.solve();

    let vars = lp.vars;
    let obj = solution.objective;
    let mut vals = solution.values.unwrap();
    let data = format!(
        "{{ \"objective\": {},  \"variables\": {:?} }}",
        obj.unwrap(),
        vals
    );
    data
}
