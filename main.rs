// ALUNOS
// DANTON TALLES TELLES DA COSTA
// MARIANE

use std::io::{self, Write};

struct Arquivo {

    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
    gid_grupo: u16,
}


struct Permissao {

    leitura: bool,
    escrita: bool,
    execucao: bool

}

struct Diretorio {

    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
}

struct Usuario {

    nome: String,
    uid: u16,
    grupos_uid: Vec<u16>

}

impl Usuario {
    fn new(nome: String, uid: u16) -> self {
        Usuario { 
            nome: nome,
            uid: uid,
            grupos_uid: Vec::new(),
        }
    }

    fn adiciona_grupo(&mut self, gid: u16) {
        self.grupos_uid.push(gid);
    }

    fn listar_grupos(&self) {
        if self.grupos_uid.len() == 0 {
            print!("Nenhum grupo encontrado.");
        
        } else {
             for n in &self.grupos_uid {
                println!("{}", n);
             }
        }
    }

    fn remove_grupo(&mut self, gid: u16) {
        for n in 0..self.grupos_uid.len(){
            if self.grupos_uid[n] == gid{
                self.grupos_uid.remove(n);
                break;
            }
        }
    }
}



struct Grupo {

    nome: String,
    uid: u16,
    membros_uid: Vec<u16>

}

impl Grupo {
    fn new(nome: String, gid: u16) ->  Self { 
        Grupo {
            nome: nome,
            gid: gid,
            membros_uid: Vec::new(),
           }
        }
   
    fn adiciona_membro(&mut self, uid: u16){
        self.membros_uid.push(uid);
    }

    fn listar_membros(&self){
        if self.membros_uid.len() == 0 {
            println!("Nenhum membro cadastrado.")
   
     } else {
        
        for n in &self.membros_uid{
            println!("{}",n);
        }
    }
    }
    
    fn remove_membro(&mut self, uid: u16) {
        for n in 0..self.membros_uid.len(){
            if self.membros_uid[n] == uid{
                self.membros_uid.remove(n);
                break;
            }
        }
    }
}

fn main() 
{
    loop 
    {

        println!("Olá! Use os comandos abaixo para navegar:");
        println!("------------------------------------------");
        println!("");
        println!("----------DIRETÓRIOS E ARQUIVOS-----------");
        println!("[1A] - Novo diretório");
        println!("[2A] - Novo arquivo");
        println!("[3A] - Remover arquivo de um diretório");
        println!("[4A] - Alterar permissões de um arquivo");
        println!("[5A] - Listar tudo dentro de um diretório");
        println!("");
        println!("------------USUÁRIOS E GRUPOS-------------");
        println!("[1B] - Novo usuário");
        println!("[2B] - Novo grupo");
        println!("[3B] - Vincular usuário a um grupo");
        println!("[4B] - Desvincular usuário de um grupo");
        println!("[5B] - Adicionar membro a um grupo");
        println!("[6B] - Remover membro de um grupo");
        println!("[7B] - Listar grupos de um usuário");
        println!("[8B] - Listar usuários de um grupo");
        println!("");
        println!("[0] - Finalizar execução");
        println!("");
        print!("eu@eu > ");

        io::stdout().flush().unwrap(); 

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); 

        let command = input.trim();

        match command 
        {
            "1A" =>println!("1"),
            "2A" =>println!("2"),
            "3A" =>println!("3"),
            "4A" =>println!("3"),
            "5A" =>println!("3"),
            "1B" =>println!("3"),
            "2B" =>println!("3"),
            "3B" =>println!("3"),
            "4B" =>println!("3"),
            "5B" =>println!("3"),
            "6B" =>println!("3"),
            "7B" =>println!("3"),
            "8B" =>println!("3"),
            "0" => break,
            _ => println!("system@admin > Por favor, digite um valor válido."),
        }

    }

}
