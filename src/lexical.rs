use std::{io, str::CharIndices};


fn treat_str_slice_as_char(st: &str) -> Option<char>{
    match st.chars().next(){
        Some(c)=>{
            return Some(c);
        }
        None=>{
            return None;
        }
    }
}
pub struct Analisador<'a> {
    ///é a posição inicial em caracteres do proximo elemento lexico a ser lido
    pub pos: usize,
    ///é a fatia contendo todos os elementos não lidos ainda
    pub prox: &'a str,
}

impl<'a> Analisador<'a>{
    pub fn novo(entrada: &'a str, pos: usize) -> Self{
        Self{pos: pos, prox: &entrada}
    }
    /**
     * Altera o pos para ser o inicio do elemento léxico que vem depois do elemento lido
     * Altera prox para ser uma subfatia, eliminando o que foi retornado como elemento lexico
     * Retorna o elemento léxico lido
     * Retorna a posição de inicio do elemento léxico lido
     */
    pub fn proximo(&mut self) -> Result<(usize, &str), Option<usize>>{ 
        let mut iter = self.prox.char_indices().enumerate().peekable();
        while let Some(((mut char_indx, (mut byte_indx, mut character)))) = iter.next(){
            if is_operation(character){
                match iter.peek(){
                    Some(it)=>{
                        let returnslice = &self.prox[byte_indx..it.1.0];
                        self.prox = &self.prox[it.1.0..];
                        return Ok((char_indx, returnslice));
                    }
                    None=>{
                        let returnslice = &self.prox[byte_indx..];
                        self.prox = &self.prox[..0];
                        return Ok((char_indx, returnslice));
                    }
                }
            }
            else if character.is_numeric(){
                let start_byte_indx = byte_indx;
                let start_char_indx = char_indx;

                while let Some(it) = iter.next(){
                    char_indx = it.0;
                    byte_indx = it.1.0;
                    character = it.1.1;
                    if !character.is_numeric(){
                        let returnslice = &self.prox[start_byte_indx..byte_indx];
                        self.prox = &self.prox[byte_indx..];
                        return Ok((start_char_indx, returnslice));
                    }
                    else{
                        match iter.peek(){
                            Some(i) =>{
                                continue;
                            }
                            None=>{
                                let returnslice = &self.prox[start_byte_indx..];
                                self.prox = &self.prox[..0];
                                return Ok((start_char_indx, returnslice));
                            }
                        }
                    }
                }
                let returnslice = &self.prox[start_byte_indx..];
                self.prox = &self.prox[..0];
                return Ok((start_char_indx, returnslice));
                // if byte_indx == start_byte_indx{
                //     let returnslice = &self.prox[start_byte_indx..=byte_indx];
                //     self.prox = &self.prox[..0];
                //     return Ok((start_char_indx, returnslice));    
                // }
                // let returnslice = &self.prox[start_byte_indx..byte_indx];
                // self.prox = &self.prox[byte_indx..];
                // return Ok((start_char_indx, returnslice));
            }
            else if character == ' '{
                continue;
            } 
            else{
                return Err(Some(char_indx));
            }
        }
        return Err(Some(self.pos));
    }
}

fn is_operation(character: char) -> bool{
    match character{
        '('|')' =>{
            return true;
        }
        '+'|'-' =>{
            return true;
        }
        '*' => {
            return true;
        }
        '/' => {
            return true;
        }
        '%' => {
            return true;
        }
        _ =>{
            return false;
        }

    }
}
