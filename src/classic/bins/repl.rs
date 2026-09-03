extern crate clvkr as clvk_rs;

use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use std::rc::Rc;

use clvk_rs::allocator::Allocator;

use chiklisp::compiler::compiler::DefaultCompilerOpts;
use chiklisp::compiler::optimize::get_optimizer;
use chiklisp::compiler::repl::Repl;
use chiklisp::compiler::srcloc::Srcloc;
use chiklisp::compiler::BasicCompileContext;

use chiklisp::classic::clvk_tools::stages::stage_0::DefaultProgramRunner;

fn main() {
    let opts = Rc::new(DefaultCompilerOpts::new("*program*"));
    let optimizer = match get_optimizer(&Srcloc::start("*repl*"), opts.clone()) {
        Ok(o) => o,
        Err(e) => {
            print!("failed to get optimizer {e:?}");
            return;
        }
    };
    let runner = Rc::new(DefaultProgramRunner::new());
    let mut context =
        BasicCompileContext::new(Allocator::new(), runner.clone(), HashMap::new(), optimizer);
    let stdin = io::stdin();
    let mut repl = Repl::new(opts, runner);

    print!(">>> ");
    io::stdout().flush().unwrap();

    for l in stdin.lock().lines() {
        match l {
            Err(_) => break,
            Ok(line) => {
                let _ = repl
                    .process_line(&mut context, line)
                    .map(|result| {
                        if let Some(result) = result {
                            print!("{}\n>>> ", result.to_sexp());
                        } else {
                            print!("... ");
                        }
                    })
                    .map_err(|e| {
                        print!("failed: {e:?}\n>>> ");
                    });
            }
        }
        io::stdout().flush().unwrap();
    }
}
