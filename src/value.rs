use std::{fmt::Debug, rc::Rc, cell::RefCell, collections::HashMap};

use crate::{state::State, parser::Action, errors::NoVar};

#[derive(Clone)]
pub enum Value {
    Number(u32),
    NativeFunction(fn(&mut State, Value) -> Result<Value, Box<dyn std::error::Error>>),
    Function {
        arg_name: String,
        captures: Vec<(String, Rc<RefCell<Value>>)>,
        action: Action
    },
    LazyCall {
        func: Rc<RefCell<Value>>,
        arg: Rc<RefCell<Value>>,
        path: String,
        span: logos::Span
    }
}

impl Value {
    pub fn native_function(val: fn(&mut State, Value) -> Result<Value, Box<dyn std::error::Error>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Value::NativeFunction(val)))
    }

    pub fn eval(&self, state: &mut State) -> Result<Self, Box<dyn std::error::Error>> {
        if let Value::LazyCall { func, arg, path, span } = self {
            let mut func = func.borrow().eval(state)?;
            let arg = arg.borrow().eval(state)?;

            while let Value::LazyCall { .. } = &func {
                func = func.eval(state)?;
            }

            match func {
                Value::NativeFunction(func) => func(state, arg),
                Value::Function { arg_name, captures, action } => {
                    let mut scope = HashMap::with_capacity(captures.len() + 1);
                    for (name, val) in captures {
                        scope.insert(name, val);
                    }
                    scope.insert(arg_name, Rc::new(RefCell::new(arg)));
                    state.push_scope(scope);
                    let res = Self::from_action(state, action);
                    state.pop_scope();
                    res
                }
                _ => Err(Box::new(crate::errors::CallNotFunction::new(path, span.clone())))
            }
        } else {
            Ok(self.clone())
        }
    }

    pub fn from_action(state: &State, action: Action) -> Result<Self, Box<dyn std::error::Error>> {
        match action.kind() {
            crate::parser::ActionKind::Chunk(_) => panic!(),
            crate::parser::ActionKind::Ident(name) => match state.local(name) {
                Some(val) => Ok(val.borrow().clone()),
                None => Err(Box::new(NoVar::new(action.path(), action.span(), name)))
            }
            crate::parser::ActionKind::Assign { .. } => panic!(),
            crate::parser::ActionKind::Call { func, arg } => Ok(Value::LazyCall {
                func: Rc::new(RefCell::new(Self::from_action(state, func.as_ref().clone())?)),
                arg: Rc::new(RefCell::new(Self::from_action(state, arg.as_ref().clone())?)),
                path: func.path().into(),
                span: func.span().start..arg.span().end
            }),
            crate::parser::ActionKind::FuncDef { arg, expr, captures: cap_names } => {
                let mut captures =  Vec::with_capacity(cap_names.len());
                for name in cap_names {
                    match state.local(name) {
                        Some(var) => captures.push((name.clone(), var)),
                        None => return Err(Box::new(NoVar::new(expr.path(), expr.span(), name)))
                    }
                }
                Ok(Value::Function {
                    arg_name: arg.clone(),
                    captures,
                    action: (**expr).clone()
                })
            }
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::NativeFunction(..) => write!(f, "NativeFunction"),
            Self::Function { arg_name, captures, action } => f.debug_struct("Function").field("arg_name", arg_name).field("captures", captures).field("action", action).finish(),
            Self::LazyCall { .. } => write!(f, "LazyCall")
        }
    }
}
