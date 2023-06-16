use aleph_syntax_tree::syntax::AlephTree as at;

fn gen(ast: at, indent: i64) -> String {
    let c_indent=aleph_syntax_tree::comp_indent(indent);
    match ast {
        at::Unit => format!("{}", ""),
        at::Ellipsis => format!("{}", ""),
        at::Int{value} => format!("{}{}", c_indent, value),
        at::Float{value} => format!("{}{}", c_indent, value),
        at::Bool{value} => format!("{}{}", c_indent, value),
        at::String{value} => format!("{}{}", c_indent, value),
        at::Complex{real, imag} => format!("{}{} + ({} *j)", c_indent, real, imag),
        at::Bytes{elems} => format!("{}", String::from_utf8(elems).expect("Found invalid UTF-8")),
        at::Tuple{elems} => format!("{}", aleph_syntax_tree::gen_list_expr_sep(elems, gen, ", ")),
        at::Array{elems} => format!("[{}]", aleph_syntax_tree::gen_list_expr_sep(elems, gen, ", ")),
        at::Neg{expr} => format!("{}-{}", c_indent, gen(*expr, 0)),
        at::Not{bool_expr} => format!("{}!({})", c_indent, gen(*bool_expr, 0)),
        at::And{bool_expr1, bool_expr2} => format!("{}{} & {}", c_indent, gen(*bool_expr1, 0), gen(*bool_expr2, 0)),
        at::Or{bool_expr1, bool_expr2} => format!("{}{} | {}", c_indent, gen(*bool_expr1, 0), gen(*bool_expr2, 0)),
        at::Add{number_expr1, number_expr2} => format!("{}{} + {}", c_indent, gen(*number_expr1, 0), gen(*number_expr2, 0)),
        at::Sub{number_expr1, number_expr2} => format!("{}{} - {}", c_indent, gen(*number_expr1, 0), gen(*number_expr2, 0)),
        at::Mul{number_expr1, number_expr2} => format!("{}{} * {}", c_indent, gen(*number_expr1, 0), gen(*number_expr2, 0)),
        at::Div{number_expr1, number_expr2} => format!("{}{} / {}", c_indent, gen(*number_expr1, 0), gen(*number_expr2, 0)),
        at::Eq{expr1, expr2} => format!("{}{} = {}", c_indent, gen(*expr1, 0), gen(*expr2, 0)),
        at::LE{expr1, expr2} => format!("{}{} <= {}", c_indent, gen(*expr1, 0), gen(*expr2, 0)),
        at::In{expr1, expr2} => format!("{}{} in {}", c_indent, gen(*expr1, 0), gen(*expr2, 0)),
        at::If{condition, then,els} => match *els {
            at::Unit => format!("{c_indent}({cond})?{{\n{then}\n{c_indent}}}", c_indent=c_indent, cond=gen(*condition, 0), then=gen(*then, indent+1)),
            _ => format!("{c_indent}({cond})?{{\n{then}\n{c_indent}}}:{{\n{els}\n{c_indent}}}", c_indent=c_indent, cond=gen(*condition, 0), then=gen(*then, indent+1), els=gen(*els, indent+1)),
        },
        at::While{init_expr, condition, loop_expr, post_expr} => {
            format!("{}\n{}({})?*{{\n{}\n{}\n{}}}", gen(*init_expr, indent), c_indent, gen(*condition, 0), gen(*loop_expr, indent+1), gen(*post_expr, indent+1), c_indent)
        },
        at::Let{var, is_pointer, value, expr} => match *expr {
            at::Unit{} => format!("{}{}{} = {};", c_indent, var, (if is_pointer=="true" {":"} else {""}), gen(*value, 0)),
            _ => format!("{}{}{} = {};\n{}", c_indent, var, (if is_pointer=="true" {":"} else {""}), gen(*value, 0), gen(*expr, indent)),
        },
        at::LetRec{name, args, body} => format!("{}fun {}({}) = {{\n{}\n{}}}", c_indent, name, aleph_syntax_tree::gen_list_expr_sep(args, gen, ", "), gen(*body, indent+1), c_indent),
        at::Get{array_name, elem} => format!("{}{}[{}]", c_indent, array_name, gen(*elem, 0)),
        at::Put{array_name, elem, value, insert} => format!("{}{}[{}{}] = {}", c_indent, array_name, (if insert=="true" {"+"} else {""}), gen(*elem, 0), gen(*value, 0)),
        at::Remove{array_name, elem, is_value} => format!("{}{}[{}{}]", c_indent, array_name, (if is_value=="true" {"-"} else {"/"}), gen(*elem, 0)),
        at::Length{var} => format!("{}|{}|", c_indent, var),
        at::Match{expr, case_list} => format!("{}match {} with\n{}", c_indent, gen(*expr, 0), aleph_syntax_tree::gen_list_expr(case_list, gen)),
        at::MatchLine{condition, case_expr} => format!("{}: {} -> {}\n", c_indent, gen(*condition, 0), gen(*case_expr, 0)),
        at::Var{var, is_pointer} => format!("{}{}{}",c_indent, (if is_pointer=="true" {"!"} else {""}), var),
        at::App{object_name, fun, param_list} => format!("{}{}{}({})",c_indent, (if object_name.ne("") {format!("{}.", object_name)} else {String::from("")}), gen(*fun, 0), aleph_syntax_tree::gen_list_expr_sep(param_list, gen, ", ")),
        at::Stmts{expr1, expr2} => format!("{};\n{}", gen(*expr1, indent), gen(*expr2, indent)), 
        at::Iprt{name} => format!("{}import {}", c_indent, name),
        at::Clss{name, attribute_list, body} => format!("{}class {} {{\n{}{};\n{}\n}}", c_indent, name, aleph_syntax_tree::comp_indent(indent+1), attribute_list.join(&format!(";\n{}", aleph_syntax_tree::comp_indent(indent+1))), gen(*body, indent+1)), 
        at::Return{value} => format!("return {}", gen(*value, 0)),
        at::Comment{value} => format!("{}{}", c_indent, value),
        at::CommentMulti{value} => format!("{}{}", c_indent, value),
        at::Break | at::Continue | at::Assert { .. } => todo!(),
    }
}

pub fn generate(ast: at) -> String {
    gen(ast, 0)
}
