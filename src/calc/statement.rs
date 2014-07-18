use super::CalcResult;
use super::expression::Expression;
use super::environment::Environment;

pub enum StmtType {
    Assign,
    FuncDef,
}

pub struct Statement {
    pub stmt_type: StmtType,
    pub name: String,
    pub rhs: Expression
}

impl Statement {
    pub fn exec(&self, env: &mut Environment) -> CalcResult<()> {
        match self.stmt_type {
            Assign => {
                let rhs = try!(self.rhs.eval(env));
                env.set_var(self.name.as_slice(), rhs)
            }
            FuncDef => {
                fail!("Not implemented")
            }
        }
    }
}
