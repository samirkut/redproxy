use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::Path;
use log::{debug};

use starlark::environment::{Globals, GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::{Value, dict::Dict};

fn read_script(script: &Path) -> Result<String> {
    let content = fs::read_to_string(script)?;
    Ok(content)
}

pub fn evaluate(script: &Path, fnName: &str) -> Result<HashMap<String, String>> {
    let content = read_script(script)?;
    let content2 = content + "\n" + fnName + "\n";
    let filename = script.file_name().unwrap_or_default().to_str().unwrap_or_default();
    let ast = AstModule::parse(filename, content2, &Dialect::Standard)?;
    let globals = Globals::standard();
    let module = Module::new();
    let mut eval = Evaluator::new(&module);
    let sc = eval.eval_module(ast, &globals)?;
    let res = eval.eval_function(sc, &[], &[])?;
    
    println!("Type: {}",res.get_type());
    let dict = Dict::from_value(res);
    let mut ret = HashMap::new();

    if let Some(d) = dict {
        for (k, v) in d.iter(){
            ret.insert(k.unpack_str().unwrap_or_default().to_string(), v.unpack_str().unwrap_or_default().to_string());
        }
    }

    Ok(ret)
}
