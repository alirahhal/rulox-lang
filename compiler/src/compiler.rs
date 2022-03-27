use crate::scanner::token::Token;

pub struct Local {
    pub name: Token,
    pub depth: i32,
}

pub struct Compiler {
    pub locals: Vec<Local>,
    pub scope_depth: i32,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            scope_depth: 0,
            locals: Vec::new(),
        }
    }

    pub fn add_local(&mut self, name: &Token) {
        let local = Local {
            name: name.clone(),
            depth: -1,
        };
        self.locals.push(local);
    }

    pub fn local_at(&self, index: usize) -> &Local {
        &self.locals[index] as _
    }

    pub fn update_local_depth_at(&mut self, index: usize, depth: i32) {
        self.locals[index].depth = depth;
    }
}
