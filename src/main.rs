use std::io;
mod lexical;
use lexical::*;


/**
 * Gera a expressão a partir do vetor de simbolos
    NÃO FUNCIONAL
 */
fn generate_expression() -> Expressão{
    let exp1 = Expressão::EXP { lhs: (Box::new(Expressão::Numero(10))), rhs: (Box::new(Expressão::Numero(20))), op: Operator::PLUS };
    let exp2 = Expressão::EXP { lhs: (Box::new(Expressão::Numero(40))), rhs: (Box::new(Expressão::Numero(20))), op: Operator::DIV };
    let exp3 = Expressão::EXP { lhs: Box::new(Expressão::None), rhs: Box::new(exp2.clone()), op: Operator::NEG };
    let exp4 = Expressão::EXP { lhs: Box::new(exp1), rhs: Box::new(exp2), op: Operator::MULT };
    return exp4;
}

#[derive(Copy, Clone, PartialEq)]
enum Operator{
    PLUS,
    MINUS,
    NEG,
    MULT,
    DIV,
    MOD,
}
impl Operator{
    fn print_op(&self){
        match self{
            Operator::PLUS => print!(" + "),
            Operator::MINUS => print!(" - "),
            Operator::NEG => print!("-"),
            Operator::MULT => print!(" * "),
            Operator::DIV => print!(" / "),
            Operator::MOD => print!(" % "),
        }
    }
    fn print_op_nospace(&self){
        match self{
            Operator::PLUS => print!("+"),
            Operator::MINUS => print!("-"),
            Operator::NEG => print!("-"),
            Operator::MULT => print!("*"),
            Operator::DIV => print!("/"),
            Operator::MOD => print!("%"),
        }
    }
}
#[derive(Clone)]
enum Expressão{
    Numero(i64),
    EXP{lhs: Box<Expressão>, 
        rhs: Box<Expressão>, 
        op: Operator
    },
    None
}

impl Expressão{
    fn avaliar(&self) -> Option<i64>{
        match self{
            Expressão::Numero(n)=>{
                return Some(*n);
            }
            Expressão::EXP { lhs: lhs, rhs: rhs, op: op }=>{
                let lnum: i64;
                let mut rnum: i64 = 0;
                let mut rhs_was_set: bool = false;
                match lhs.avaliar(){
                    Some(n)=>{
                        lnum = n;
                    }
                    None=>{
                        //checking negation sign
                        match rhs.avaliar(){
                            Some(n)=>{
                                rnum = n;
                                rhs_was_set = true;
                            }
                            None =>{
                                rnum = 0;
                                rhs_was_set = false;
                                return None;
                            }
                        }
                        match op{
                            Operator::NEG =>{
                                match rnum.checked_neg(){
                                    Some(n) => {
                                        return Some(n);
                                    }
                                    None =>{
                                        return None;
                                    }
                                }
                            }
                            _ =>{
                                return None;
                            }
                        }
                    }
                }
                if !rhs_was_set{
                    match rhs.avaliar(){
                        Some(n)=>{
                            rnum = n;
                        }
                        None=>{
                            rnum = 0;
                            return None;
                        }
                    }
                }
                
                match op{
                    Operator::PLUS=>{
                        match lnum.checked_add(rnum){
                            Some(n)=>{
                                return Some(n);
                            }
                            None=>{
                                return None;
                            }
                        }
                    }
                    Operator::MINUS=>{
                        match lnum.checked_sub(rnum){
                            Some(n)=>{
                                return Some(n);
                            }
                            None=>{
                                return None;
                            }
                        }
                    }
                    Operator::DIV=>{
                        match lnum.checked_div(rnum){
                            Some(n)=>{
                                return Some(n);
                            }
                            None=>{
                                return None;
                            }
                        }
                    }
                    Operator::MULT=>{
                        match lnum.checked_mul(rnum){
                            Some(n)=>{
                                return Some(n);
                            }
                            None=>{
                                return None;
                            }
                        }
                    }
                    Operator::MOD=>{
                        match lnum.checked_rem(rnum){
                            Some(n)=>{
                                return Some(n);
                            }
                            None=>{
                                return None;
                            }
                        }
                    }
                    //NEG has been checked before
                    _ =>{
                        return None;
                    }
                }
            }
            Expressão::None=>{
                return None;
            }
        }
    }
    fn imprimir(&self){
        match self{
            Expressão::Numero(n)=>{
                print!("{n}");
            }
            Expressão::EXP{ lhs: lhs, rhs: rhs, op: op }=>{
                if *op == Operator::NEG{
                    print!("-");
                    let mut open_parenthesis: bool = false;
                    match **rhs{
                        Expressão::EXP { lhs: _, rhs: _, op: _ } => {
                            open_parenthesis = true;
                            print!("(");
                        },
                        _ => {}
                    }
                    rhs.imprimir();
                    if open_parenthesis { print!(")"); }
                }
                else{
                    match lhs.as_ref(){
                        Expressão::Numero(_) => {
                            lhs.imprimir();
                        }
                        Expressão::EXP { lhs: _, rhs: _, op: lhs_op } => {
                            let mut lhs_open_parenthesis = false;
                            if get_precedence(*lhs_op) < get_precedence(*op){
                                lhs_open_parenthesis = true;
                                print!("(");
                            }
                            lhs.imprimir();
                            if lhs_open_parenthesis{print!(")")}
                        },
                        Expressão::None => {},
                    }
                    op.print_op();
                    match rhs.as_ref(){
                        Expressão::Numero(_) => {
                            rhs.imprimir();
                        }
                        Expressão::EXP { lhs: _, rhs: _, op: rhs_op } => {
                            let mut rhs_open_parenthesis = false;
                            if get_precedence(*rhs_op) < get_precedence(*op){
                                rhs_open_parenthesis = true;
                                print!("(");
                            }
                            rhs.imprimir();
                            if rhs_open_parenthesis{print!(")")}
                        },
                        Expressão::None => {
                            return;
                        },
                    }
                }
            }
            Expressão::None=>{
                return;
            }
        }
    }
    fn imprimir_arvore(&self, depth: u64){
        match self{
            Expressão::Numero(n)=>{
                for _ in 0..depth{
                    print!("│");
                }
                if depth > 0{print!("├")}
                print!("{n}: {depth}");
                println!("");
            }
            Expressão::EXP{ lhs: lhs, rhs: rhs, op: op }=>{
                for _ in 0..depth{
                    print!("│");
                }
                if depth > 0 {print!("├")}
                op.print_op_nospace();
                println!(": {depth}");
                match lhs.as_ref(){
                    Expressão::Numero(_) => {
                        lhs.imprimir_arvore(depth+1);
                    }
                    Expressão::EXP { lhs: _, rhs: _, op: lhs_op } => {
                        lhs.imprimir_arvore(depth+1);
                    },
                    Expressão::None => {},
                }
                match rhs.as_ref(){
                    Expressão::Numero(_) => {
                        rhs.imprimir_arvore(depth+1);
                    }
                    Expressão::EXP { lhs: _, rhs: _, op: rhs_op } => {
                        let mut rhs_open_parenthesis = false;
                        if get_precedence(*rhs_op) < get_precedence(*op){
                            rhs_open_parenthesis = true;
                        }
                        rhs.imprimir_arvore(depth+1);
                        if rhs_open_parenthesis{}
                    },
                    Expressão::None => {
                        return;
                    },
                }
            }
            Expressão::None=>{
                return;
            }
        }
    }
    
}
fn get_precedence(op: Operator) -> i32{
    match op{
        Operator::PLUS => {
            0
        },
        Operator::MINUS => {
            0
        },
        Operator::MULT => {
            1
        },
        Operator::DIV => {
            1
        },
        Operator::MOD => {
            1
        },
        Operator::NEG =>{
            2
        }
    }
}
fn main() {
    // let mut literals_vec: Vec<String> = Vec::new();
    // let mut buf: String = String::new();
    // loop{
    //     match io::stdin().read_line(&mut buf){
    //         Ok(n) => {
    //             if n == 0 {
    //                 return Ok(());
    //             }
    //         }
    //         Err(e) => {
    //             println!("Erro no read_line");
    //             return Err(e);
    //         }
    //     }
    //     buf = buf.trim().to_string();
    //     buf = buf.trim_matches(&[ ' ', '\n', '\t']).to_string();
    //     let mut anl: Analisador = Analisador::novo(&buf, 0);
    //     while !anl.prox.is_empty(){
    //         // print!(" |{}| ", str_slice);
    //         match anl.proximo(){
    //             Ok(a) => {
    //                 literals_vec.push(a.1.to_string());
    //             }
    //             Err(e) =>{
    //                 // println!("Erro na posição {}", e.unwrap());
    //                 break;
    //             }
    //         }
    //     }
        // print!("{:?}", literals_vec);
        // buf.clear();
        let exp1 = Expressão::EXP { lhs: (Box::new(Expressão::Numero(10))), rhs: (Box::new(Expressão::Numero(20))), op: Operator::PLUS };
        let exp2 = Expressão::EXP { lhs: (Box::new(Expressão::Numero(40))), rhs: (Box::new(Expressão::Numero(20))), op: Operator::DIV };
        let exp3 = Expressão::EXP { lhs: Box::new(Expressão::None), rhs: Box::new(exp2.clone()), op: Operator::NEG };
        let exp4 = Expressão::EXP { lhs: Box::new(exp1.clone()), rhs: Box::new(exp2.clone()), op: Operator::MULT };
        
        exp1.imprimir();
        println!("");
        exp1.imprimir_arvore(0);
        println!("\nResultado = {:?}", exp1.avaliar());
        println!("");
        exp2.imprimir();
        println!("");
        exp2.imprimir_arvore(0);
        println!("\nResultado = {:?}", exp2.avaliar());
        println!("");
        exp3.imprimir();
        println!("");
        exp3.imprimir_arvore(0);
        println!("\nResultado = {:?}", exp3.avaliar());
        println!("");
        exp4.imprimir();
        println!("");
        exp4.imprimir_arvore(0);
        println!("\nResultado = {:?}", exp4.avaliar());
        println!("");
    // }
}
