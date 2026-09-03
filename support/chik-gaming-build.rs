use std::collections::HashMap;
use std::fs;

use clvkr::allocator::Allocator;
use toml::{Table, Value};

use chiklisp::classic::clvk_tools::comp_input::RunAndCompileInputData;
use chiklisp::classic::clvk_tools::clvkc::CompileError;
use chiklisp::classic::platform::argparse::ArgumentValue;
use chiklisp::compiler::comptypes::CompileErr;
use chiklisp::compiler::srcloc::Srcloc;

fn do_compile(title: &str, filename: &str) -> Result<(), CompileError> {
    let mut allocator = Allocator::new();
    let mut arguments: HashMap<String, ArgumentValue> = HashMap::new();
    arguments.insert(
        "include".to_string(),
        ArgumentValue::ArgArray(vec![
            ArgumentValue::ArgString(None, "clsp".to_string()),
            ArgumentValue::ArgString(None, ".".to_string()),
        ]),
    );

    let file_content = fs::read_to_string(filename).map_err(|e| {
        CompileErr(
            Srcloc::start(filename),
            format!("failed to read {filename}: {e:?}"),
        )
    })?;

    arguments.insert(
        "path_or_code".to_string(),
        ArgumentValue::ArgString(Some(filename.to_string()), file_content),
    );

    let parsed = RunAndCompileInputData::new(&mut allocator, &arguments).map_err(|e| {
        CompileError::Modern(
            Srcloc::start("*error*"),
            format!("error building chiklisp {title}: {e}"),
        )
    })?;
    let mut symbol_table = HashMap::new();

    parsed.compile_modern(&mut allocator, &mut symbol_table)?;

    Ok(())
}

fn compile_chiklisp() -> Result<(), CompileError> {
    let srcloc = Srcloc::start("chiklisp.toml");
    let chiklisp_toml_text = fs::read_to_string("chiklisp.toml").map_err(|e| {
        CompileError::Modern(
            srcloc.clone(),
            format!("Error reading chiklisp.toml: {e:?}"),
        )
    })?;

    let chiklisp_toml = chiklisp_toml_text
        .parse::<Table>()
        .map_err(|e| CompileError::Modern(srcloc, format!("Error parsing chiklisp.toml: {e:?}")))?;

    if let Some(Value::Table(t)) = chiklisp_toml.get("compile") {
        for (k, v) in t.iter() {
            if let Value::String(s) = v {
                do_compile(k, s)?;
            }
        }
    }

    Ok(())
}

// Compile chiklisp programs in this tree.
fn main() {
    if std::env::var("CHIKLISP_NOCOMPILE").is_err() {
        if let Err(e) = compile_chiklisp() {
            panic!("error compiling chiklisp: {e:?}");
        }
    }
}
