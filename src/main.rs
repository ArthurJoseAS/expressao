use std::io;
mod lexical;
use lexical::*;


/**
 * Gera a expressão a partir do vetor de simbolos
 */
fn generate_expression_from_vec(vec: &Vec<String>) -> Expressão{
    todo!()
}

#[derive(Copy, Clone)]
enum Operator{
    PLUS,
    MINUS,
    NEG,
    MULT,
    DIV,
    MOD,
}
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
        
    }
    fn imprimir_arvore(&self){
        todo!()
    }
}

fn main() -> io::Result<()> {
    let mut literals_vec: Vec<String> = Vec::new();
    let mut buf: String = String::new();
    loop{
        match io::stdin().read_line(&mut buf){
            Ok(n) => {
                if n == 0 {
                    return Ok(());
                }
            }
            Err(e) => {
                println!("Erro no read_line");
                return Err(e);
            }
        }
        buf = buf.trim().to_string();
        buf = buf.trim_matches(&[ ' ', '\n', '\t']).to_string();
        let mut anl: Analisador = Analisador::novo(&buf, 0);
        while !anl.prox.is_empty(){
            // print!(" |{}| ", str_slice);
            match anl.proximo(){
                Ok(a) => {
                    literals_vec.push(a.1.to_string());
                }
                Err(e) =>{
                    println!("Erro na posição {}", e.unwrap());
                    break;
                }
            }
        }
        print!("{:?}", literals_vec);
        buf.clear();
        //Não consegui fazer a lógica do parsing para a análise sintática 
        //então o trabalho ficou incompleto
        let exec_expression: Expressão = generate_expression_from_vec(&literals_vec);
        literals_vec.clear();
        println!("");
    }
}
