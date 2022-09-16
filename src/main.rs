use std::{rc::Rc, cell::RefCell};

use parser::ActionKind;

use crate::value::Value;

mod errors;
mod parser;
mod value;
mod state;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        return Err(Box::new(errors::Usage));
    }
    let path = args[1].clone();
    let src = std::fs::read_to_string(&path)?;
    let actions = parser::parse(&src, &path)?;
    let mut state = state::State::new();
    let globals = state.globals_mut();
    globals.insert("print_num".into(), Value::native_function(print_num));
    if let ActionKind::Chunk(actions) = actions.kind() {
        for action in actions {
            if let ActionKind::Assign { target, expr } = action.kind() {
                let val = Value::from_action(&state, (**expr).clone())?;
                let globals = state.globals_mut();
                globals.insert(target.clone(), Rc::new(RefCell::new(val)));
            } else {
                let value = Value::from_action(&mut state, action.clone())?;
                if let Value::LazyCall {..} = value {
                    value.eval(&mut state)?;
                }
            }
        }
    } else {
        panic!();
    }
    Ok(())
}

fn print_num(state: &mut state::State, val: Value) -> Result<Value, Box<dyn std::error::Error>> {
    println!("{val:#?}");
    let tmp1 = Value::LazyCall {
        func: Rc::new(RefCell::new(val)),
        arg: Value::native_function(|_state, val| {
            if let Value::Number(num) = val {
                Ok(Value::Number(num + 1))
            } else {
                Err(Box::new(errors::BadNumber))
            }
        })
    };
    let tmp1 = tmp1.eval(state)?;
    let mut tmp2 = Value::LazyCall {
        func: Rc::new(RefCell::new(tmp1)),
        arg: Rc::new(RefCell::new(Value::Number(0)))
    };
    while let Value::LazyCall { .. } = &tmp2 {
        tmp2 = tmp2.eval(state)?;
    }
    if let Value::Number(res) = tmp2 {
        println!("{res}");
        Ok(Value::NativeFunction(print_num))
    } else {
        Err(Box::new(errors::BadNumber))
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
