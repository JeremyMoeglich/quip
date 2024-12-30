use fst::*;

trait PrintFSTNode {
    fn print_into(&self, buf: &mut String);
}

impl PrintFSTNode for File {
    fn print_into(&self, buf: &mut String) {
        self.pre_space.print_into(buf);
        for (stmt, ws) in &self.statements {
            stmt.print_into(buf);
            ws.print_into(buf);
        }
    }
}

impl PrintFSTNode for Whitespace0 {
    fn print_into(&self, buf: &mut String) {
        for space in self {
            space.print_into(buf);
        }
    }
}

impl PrintFSTNode for SpaceElement {
    fn print_into(&self, buf: &mut String) {
        match self {
            SpaceElement::LineComment(s) => buf.push_str(s),
            SpaceElement::BlockComment(s) => buf.push_str(s),
            SpaceElement::Space(s) => buf.push_str(s),
        }
    }
}

impl PrintFSTNode for Statement {
    fn print_into(&self, buf: &mut String) {
        match self {
            Statement::Expression { expr, semi } => {
                expr.print_into(buf);
                if let Some(semi) = semi {
                    semi.print_into(buf);
                    buf.push(';');
                }
            }
            Statement::Return(label_expression) => {
                buf.push_str("return");
                label_expression.print_into(buf);
            }
            Statement::Break(label_expression) => {
                buf.push_str("break");
                label_expression.print_into(buf);
            }
            Statement::Continue(spaced_label) => {
                buf.push_str("continue");
                spaced_label.print_into(buf);
            }
            Statement::Function { name, closure } => {
                buf.push_str("function");
                name.print_into(buf);
                closure.print_into(buf);
            }
            Statement::Struct { name, fields } => {
                buf.push_str("struct");
                name.print_into(buf);
            }
        }
    }
}
