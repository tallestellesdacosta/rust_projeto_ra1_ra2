// ALUNOS
// DANTON TALLES TELLES DA COSTA
// MARIANE PARANÁ


use std::thread;
use std::time::Duration;
use std::io::{self, Write};


const CLEAR: &str = "\x1B[2J\x1B[1;1H";
    const RESET: &str = "\x1B[0m";
    const PURPLE: &str = "\x1B[38;5;135m";
    const DIM: &str = "\x1B[38;5;244m";
    const UNDERLINE: &str = "\x1B[4m";
    const BOLD: &str = "\x1B[1m";

struct Arquivo {

    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
    gid_grupo: u16,
}

impl Arquivo {

    fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Self 
    {
        Arquivo 
        { 
            nome: nome,
            tamanho: tamanho,
            permissoes: ( 
                Permissao::new(true, true, false),  
                Permissao::new(true, true, false),  
                Permissao::new(true, false, false)
            ),
            uid_dono: uid,
            gid_grupo: gid,
        }
    }

    fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) 
    {
        self.permissoes = permissoes;
    }

    fn stat(&self) 
    {
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        println!("Permissoes: ({}{}{} / {}{}{}{}{}{}{}{}{})", 
            self.permissoes.0.octal(), 
            self.permissoes.1.octal(), 
            self.permissoes.2.octal(),
            if self.permissoes.0.leitura { 'r' } else { '-' },
            if self.permissoes.0.escrita { 'w' } else { '-' },
            if self.permissoes.0.execucao { 'x' } else { '-' },
            if self.permissoes.1.leitura { 'r' } else { '-' },
            if self.permissoes.1.escrita { 'w' } else { '-' },
            if self.permissoes.1.execucao { 'x' } else { '-' },
            if self.permissoes.2.leitura { 'r' } else { '-' },
            if self.permissoes.2.escrita { 'w' } else { '-' },
            if self.permissoes.2.execucao { 'x' } else { '-' }
        );
        println!("Uid: {}", self.uid_dono);
        println!("Gid: {}", self.gid_grupo);
    }
}

struct Permissao 
{

    leitura: bool,
    escrita: bool,
    execucao: bool

}

impl Permissao 
{

    fn new(leitura: bool, escrita: bool, execucao: bool) -> Self 
    {

        Permissao 
        {

            leitura: leitura,
            escrita: escrita,
            execucao: execucao

        }

    }

    fn octal(&self) -> u8 
    {
        // converte true false p/ 0 1 e multiplica de acordo com pesos. se leitura, 4; escrita, 2; exec, 1. 
        // soma tudo p/ gerar o octal correspondente.
        (self.leitura as u8 * 4) + (self.escrita as u8 * 2) + (self.execucao as u8 * 1)

    }

}

struct Diretorio 
{
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
}

impl Diretorio 
{

    fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), uid_dono: u16) ->  Self 
    { 
        Diretorio 
        {
            nome: nome,
            arquivos: Vec::new(),
            permissoes: permissoes,
            uid_dono: uid_dono
        }
    }

    fn adiciona_arquivo(&mut self, arquivo: Arquivo)
    {
        self.arquivos.push(arquivo);
    }

    fn remove_arquivo(&mut self, nome: String) 
    {

        for arquivo in 0..self.arquivos.len()
        {

            if self.arquivos[arquivo].nome == nome
            {

                self.arquivos.remove(arquivo);
                break;

            }

        }

    }

    fn listar_conteudo(&self)
    {
        println!("DIRETORIO {}:", self.nome);

        if self.arquivos.len() == 0 
        {

            println!("> > > Não há nenhum arquivo neste diretório. < < <")

        } 
        else 
        {

            for arquivo in &self.arquivos
            {

                println!("- {}", arquivo.nome);

            }

        }

    }

}

struct Usuario {

    nome: String,
    uid: u16,
    grupos_uid: Vec<u16>

}

impl Usuario {
    fn new(nome: String, uid: u16) -> Self {
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
    fn new(nome: String, uid: u16) ->  Self { 
        Grupo {
            nome: nome,
            uid: uid,
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

    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();
    let mut diretorios: Vec<Diretorio> = Vec::new();

    loop {
        print!("{}", CLEAR);
        println!("");

        println!("{PURPLE}┌───────────────────────────────────────────────────────────────────────────────────────────────┐{RESET}");
        
        println!("{PURPLE}│{RESET} {PURPLE}{BOLD}> Cadê minha caixa? >.<{RESET}                                            {DIM} ,-.       _,---._ __  / \\{RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} Um sistema de gerenciamento de arquivos rudimentar                {DIM}/  )    .-'       `./ /   \\{RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} feito em Rust para matéria de Programação Imperativa.           {DIM}(  (   ,'            `/    /|{RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET}                                                                 {DIM} \\  `-\"             \\'\\   / |{RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {DIM}───────────────────────────────────────────────────{RESET}             {DIM}  `.              ,  \\ \\ /  |{RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET}                                                                 {DIM}   /`.          ,'-`----Y   |{RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}{UNDERLINE}1. Usuários{RESET}                                                     {DIM}  (            ;        |   '{RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [1B] Novo usuário                                             {DIM}  |  ,-.    ,-'         |  / {RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET}                                                                 {DIM}  |  | (   |            | /  {RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}{UNDERLINE}2. Grupos{RESET}                                                       {DIM}  )  |  \\  `.___________|/   {RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [2B] Novo grupo                                               {DIM}  `--'   `--'                {RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [3B] Vincular usuário a um grupo                                                            {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [4B] Desvincular usuário de um grupo                                                        {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [5B] Adicionar membro a um grupo                                                            {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [6B] Remover membro de um grupo                                                             {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [7B] Listar grupos de um usuário                                                            {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [8B] Listar usuários de um grupo                                                            {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET}                                                                                               {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}{UNDERLINE}3. Diretórios{RESET}                                                                                 {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [1A] Novo diretório                                                                         {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [2A] Listar tudo dentro de um diretório                                                     {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET}                                                                                               {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}{UNDERLINE}4. Arquivos{RESET}                                                                                   {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [3A] Novo arquivo                                                                           {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [4A] Remover arquivo de um diretório                                                        {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [5A] Alterar permissões de um arquivo                                                       {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET}                                                                                               {PURPLE}│{RESET}");
        
        println!("{PURPLE}│{RESET} {PURPLE}▌{RESET} {DIM}Dica: Digite [0] para finalizar a execução.{RESET}                                                 {PURPLE}│{RESET}");
        
        println!("{PURPLE}└───────────────────────────────────────────────────────────────────────────────────────────────┘{RESET}");
        println!();

        print!("{PURPLE}user@user ➜  ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim().to_uppercase();

        match command.as_str() {
            "1B" => terminal_novo_usuario(&mut usuarios),
            "2B" => terminal_novo_grupo(&mut grupos),
            "3B" => terminal_vincular_usuario_grupo(&mut usuarios, &grupos),
            "4B" => terminal_desvincular_usuario_grupo(&mut usuarios, &grupos),
            "5B" => terminal_adicionar_membro(&mut grupos, &usuarios),
            "6B" => terminal_remover_membro(&mut grupos, &usuarios),
            "7B" => terminal_listar_grupos(&usuarios),
            "8B" => terminal_listar_usuarios(&grupos),
            "1A" => terminal_novo_diretorio(&mut diretorios, &usuarios),
            "2A" => terminal_listar_diretorio(&diretorios),
            "3A" => terminal_novo_arquivo(&mut diretorios, &grupos, &usuarios),
            "4A" => terminal_remover_arquivo(&mut diretorios),
            "5A" => terminal_alterar_permissao_arquivo(&mut diretorios),
            "0"  => break,
            _ => {
                println!("\n{DIM}Comando não reconhecido. Pressione ENTER para tentar novamente.{RESET}");
                io::stdin().read_line(&mut String::new()).unwrap();
            }
        }
    }
}

fn terminal_novo_usuario(usuarios: &mut Vec<Usuario>) {
    println!("");
    println!("{PURPLE}Digite o nome do novo usuário:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");

    io::stdout().flush().unwrap();
    let mut nome_usuario = String::new();
    io::stdin().read_line(&mut nome_usuario).unwrap();
    let nome_usuario = nome_usuario.trim();

    println!("{PURPLE}Digite o UID do novo usuário:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");

    io::stdout().flush().unwrap();
    let mut uid = String::new();
    io::stdin().read_line(&mut uid).unwrap();
    let uid: u16 = uid.trim().parse().unwrap();

    let novo_usuario = Usuario::new(nome_usuario.to_string(), uid);
    usuarios.push(novo_usuario);
    thread::sleep(Duration::from_secs(2));

    println!("");
    println!("{PURPLE}O usuário {} foi criado com sucesso!{RESET}", nome_usuario);
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}

fn terminal_novo_grupo(grupos: &mut Vec<Grupo>) {
    println!("");
    println!("{PURPLE}Digite o nome do novo grupo:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");

    io::stdout().flush().unwrap();
    let mut nome_grupo = String::new();
    io::stdin().read_line(&mut nome_grupo).unwrap();
    let nome_grupo = nome_grupo.trim();

    println!("{PURPLE}Digite o GID (UID) do novo grupo:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");

    io::stdout().flush().unwrap();
    let mut gid = String::new();
    io::stdin().read_line(&mut gid).unwrap();
    let gid: u16 = gid.trim().parse().unwrap();

    let novo_grupo = Grupo::new(nome_grupo.to_string(), gid);
    grupos.push(novo_grupo);
    thread::sleep(Duration::from_secs(2));

    println!("");
    println!("{PURPLE}O grupo {} foi criado com sucesso!{RESET}", nome_grupo);
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}

fn terminal_vincular_usuario_grupo(usuarios: &mut Vec<Usuario>, grupos: &Vec<Grupo>) {
    println!("");
    if usuarios.len() == 0 || grupos.len() == 0 {
        println!("{PURPLE}> > > É necessário ter ao menos um usuário e um grupo cadastrados. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }

    println!("{PURPLE}Selecione o índice do usuário:{RESET}");
    for (indice, usuario) in usuarios.iter().enumerate() {
        println!("{DIM}{} - {} (UID: {}){RESET}", indice, usuario.nome, usuario.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut usr_idx = String::new();
    io::stdin().read_line(&mut usr_idx).unwrap();
    let usr_idx: usize = usr_idx.trim().parse().unwrap();

    println!("{PURPLE}Selecione o índice do grupo para vincular:{RESET}");
    for (indice, grupo) in grupos.iter().enumerate() {
        println!("{DIM}{} - {} (GID: {}){RESET}", indice, grupo.nome, grupo.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut grp_idx = String::new();
    io::stdin().read_line(&mut grp_idx).unwrap();
    let grp_idx: usize = grp_idx.trim().parse().unwrap();

    let gid = grupos[grp_idx].uid;
    usuarios[usr_idx].adiciona_grupo(gid);
    thread::sleep(Duration::from_secs(2));

    println!("");
    println!("{PURPLE}O usuário {} foi vinculado ao grupo {} com sucesso!{RESET}", usuarios[usr_idx].nome, grupos[grp_idx].nome);
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}

fn terminal_desvincular_usuario_grupo(usuarios: &mut Vec<Usuario>, grupos: &Vec<Grupo>) {
    println!("");
    if usuarios.len() == 0 || grupos.len() == 0 {
        println!("{PURPLE}> > > É necessário ter ao menos um usuário e um grupo cadastrados. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }

    println!("{PURPLE}Selecione o índice do usuário:{RESET}");
    for (indice, usuario) in usuarios.iter().enumerate() {
        println!("{DIM}{} - {} (UID: {}){RESET}", indice, usuario.nome, usuario.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut usr_idx = String::new();
    io::stdin().read_line(&mut usr_idx).unwrap();
    let usr_idx: usize = usr_idx.trim().parse().unwrap();

    println!("{PURPLE}Digite o GID do grupo que deseja desvincular deste usuário:{RESET}");
    usuarios[usr_idx].listar_grupos();
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut gid_input = String::new();
    io::stdin().read_line(&mut gid_input).unwrap();
    let gid: u16 = gid_input.trim().parse().unwrap();

    usuarios[usr_idx].remove_grupo(gid);
    thread::sleep(Duration::from_secs(2));

    println!("");
    println!("{PURPLE}O grupo (GID: {}) foi desvinculado do usuário {} com sucesso!{RESET}", gid, usuarios[usr_idx].nome);
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}

fn terminal_adicionar_membro(grupos: &mut Vec<Grupo>, usuarios: &Vec<Usuario>) {
    println!("");
    if grupos.len() == 0 || usuarios.len() == 0 {
        println!("{PURPLE}> > > É necessário ter ao menos um grupo e um usuário cadastrados. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }

    println!("{PURPLE}Selecione o índice do grupo:{RESET}");
    for (indice, grupo) in grupos.iter().enumerate() {
        println!("{DIM}{} - {} (GID: {}){RESET}", indice, grupo.nome, grupo.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut grp_idx = String::new();
    io::stdin().read_line(&mut grp_idx).unwrap();
    let grp_idx: usize = grp_idx.trim().parse().unwrap();

    println!("{PURPLE}Selecione o índice do usuário para adicionar como membro:{RESET}");
    for (indice, usuario) in usuarios.iter().enumerate() {
        println!("{DIM}{} - {} (UID: {}){RESET}", indice, usuario.nome, usuario.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut usr_idx = String::new();
    io::stdin().read_line(&mut usr_idx).unwrap();
    let usr_idx: usize = usr_idx.trim().parse().unwrap();

    let uid = usuarios[usr_idx].uid;
    grupos[grp_idx].adiciona_membro(uid);
    thread::sleep(Duration::from_secs(2));

    println!("");
    println!("{PURPLE}O membro {} foi adicionado ao grupo {} com sucesso!{RESET}", usuarios[usr_idx].nome, grupos[grp_idx].nome);
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}

fn terminal_remover_membro(grupos: &mut Vec<Grupo>, usuarios: &Vec<Usuario>) {
    println!("");
    if grupos.len() == 0 || usuarios.len() == 0 {
        println!("{PURPLE}> > > É necessário ter ao menos um grupo e um usuário cadastrados. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }

    println!("{PURPLE}Selecione o índice do grupo:{RESET}");
    for (indice, grupo) in grupos.iter().enumerate() {
        println!("{DIM}{} - {} (GID: {}){RESET}", indice, grupo.nome, grupo.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut grp_idx = String::new();
    io::stdin().read_line(&mut grp_idx).unwrap();
    let grp_idx: usize = grp_idx.trim().parse().unwrap();

    println!("{PURPLE}Digite o UID do membro que deseja remover deste grupo:{RESET}");
    grupos[grp_idx].listar_membros();
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut uid_input = String::new();
    io::stdin().read_line(&mut uid_input).unwrap();
    let uid: u16 = uid_input.trim().parse().unwrap();

    grupos[grp_idx].remove_membro(uid);
    thread::sleep(Duration::from_secs(2));

    println!("");
    println!("{PURPLE}O membro (UID: {}) foi removido do grupo {} com sucesso!{RESET}", uid, grupos[grp_idx].nome);
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}

fn terminal_listar_grupos(usuarios: &Vec<Usuario>) {
    println!("");
    if usuarios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum usuário cadastrado. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }

    println!("{PURPLE}Selecione o índice do usuário para listar seus grupos:{RESET}");
    for (indice, usuario) in usuarios.iter().enumerate() {
        println!("{DIM}{} - {} (UID: {}){RESET}", indice, usuario.nome, usuario.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut usr_idx = String::new();
    io::stdin().read_line(&mut usr_idx).unwrap();
    let usr_idx: usize = usr_idx.trim().parse().unwrap();

    println!("");
    println!("{PURPLE}Grupos vinculados ao usuário {}:{RESET}", usuarios[usr_idx].nome);
    usuarios[usr_idx].listar_grupos();
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}

fn terminal_listar_usuarios(grupos: &Vec<Grupo>) {
    println!("");
    if grupos.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum grupo cadastrado. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }

    println!("{PURPLE}Selecione o índice do grupo para listar seus membros:{RESET}");
    for (indice, grupo) in grupos.iter().enumerate() {
        println!("{DIM}{} - {} (GID: {}){RESET}", indice, grupo.nome, grupo.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut grp_idx = String::new();
    io::stdin().read_line(&mut grp_idx).unwrap();
    let grp_idx: usize = grp_idx.trim().parse().unwrap();

    println!("");
    println!("{PURPLE}Membros pertencentes ao grupo {}:{RESET}", grupos[grp_idx].nome);
    grupos[grp_idx].listar_membros();
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}

fn terminal_novo_diretorio(diretorios: &mut Vec<Diretorio>, _usuarios: &Vec<Usuario>) {
    println!("");
    println!("{PURPLE}Digite o nome do novo diretório:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");

    io::stdout().flush().unwrap();
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome = nome.trim();

    println!("{PURPLE}Digite o UID do dono do diretório:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut uid_dono_input = String::new();
    io::stdin().read_line(&mut uid_dono_input).unwrap();

    let uid_dono: u16 = uid_dono_input.trim().parse().unwrap();

    let novo_diretorio = Diretorio::new(
        nome.to_string(),
        (
            Permissao::new(true, true, true),
            Permissao::new(true, true, true),
            Permissao::new(true, true, true),
        ),
        uid_dono,
    );

    diretorios.push(novo_diretorio);
    thread::sleep(Duration::from_secs(2));

    println!("");
    println!("{PURPLE}O diretório {} foi criado com sucesso!{RESET}", nome);
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}

fn terminal_listar_diretorio(diretorios: &Vec<Diretorio>) {
    println!("");
    if diretorios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum diretório criado para listagem. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }

    println!("{PURPLE}Selecione o índice do diretório que deseja listar:{RESET}");
    for (indice, diretorio) in diretorios.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, diretorio.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut dir_idx = String::new();
    io::stdin().read_line(&mut dir_idx).unwrap();
    let dir_idx: usize = dir_idx.trim().parse().unwrap();

    println!("");
    diretorios[dir_idx].listar_conteudo();

    if diretorios[dir_idx].arquivos.len() > 0 {
        println!("");
        println!("{PURPLE}Detalhes (stat) dos arquivos no diretório:{RESET}");
        for arquivo in &diretorios[dir_idx].arquivos {
            println!("{DIM}--------------------------------------------{RESET}");
            arquivo.stat();
        }
    }
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}

fn terminal_novo_arquivo(diretorios: &mut Vec<Diretorio>, grupos: &Vec<Grupo>, usuarios: &Vec<Usuario>) {
    println!("");
    if diretorios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum diretório criado para uso. Por favor, crie um antes de criar arquivos. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }
    if grupos.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum grupo criado para uso. Por favor, crie um antes de criar arquivos. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }
    if usuarios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum usuário criado para associação. Por favor, crie um antes de criar arquivos. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }

    println!("{PURPLE}Digite o número do diretório pai (opções disponíveis abaixo):{RESET}");
    for (indice, diretorio) in diretorios.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, diretorio.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut diretorio_idx = String::new();
    io::stdin().read_line(&mut diretorio_idx).unwrap();
    let diretorio_idx: usize = diretorio_idx.trim().parse().unwrap();

    println!("{PURPLE}Digite o número do grupo para o arquivo (opções disponíveis abaixo):{RESET}");
    for (indice, grupo) in grupos.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, grupo.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut grupo_idx = String::new();
    io::stdin().read_line(&mut grupo_idx).unwrap();
    let grupo_idx: usize = grupo_idx.trim().parse().unwrap();

    println!("{PURPLE}Digite o número do usuário dono do arquivo (opções disponíveis abaixo):{RESET}");
    for (indice, usuario) in usuarios.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, usuario.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut usuario_idx = String::new();
    io::stdin().read_line(&mut usuario_idx).unwrap();
    let usuario_idx: usize = usuario_idx.trim().parse().unwrap();

    println!("{PURPLE}Digite o nome para o novo arquivo:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut arquivo_nome = String::new();
    io::stdin().read_line(&mut arquivo_nome).unwrap();
    let arquivo_nome = arquivo_nome.trim();

    println!("{PURPLE}Digite o tamanho para o novo arquivo (em bytes):{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut tamanho = String::new();
    io::stdin().read_line(&mut tamanho).unwrap();
    let tamanho: u64 = tamanho.trim().parse().unwrap();

    diretorios[diretorio_idx].adiciona_arquivo(Arquivo::new(
        arquivo_nome.to_string(),
        tamanho,
        usuarios[usuario_idx].uid,
        grupos[grupo_idx].uid,
    ));

    thread::sleep(Duration::from_secs(2));
    println!("");
    println!("{PURPLE}O arquivo {} foi criado com sucesso!{RESET}", arquivo_nome);
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}

fn terminal_remover_arquivo(diretorios: &mut Vec<Diretorio>) {
    println!("");
    if diretorios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum diretório cadastrado. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }

    println!("{PURPLE}Selecione o índice do diretório de onde deseja remover o arquivo:{RESET}");
    for (indice, diretorio) in diretorios.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, diretorio.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut dir_idx = String::new();
    io::stdin().read_line(&mut dir_idx).unwrap();
    let dir_idx: usize = dir_idx.trim().parse().unwrap();

    if diretorios[dir_idx].arquivos.len() == 0 {
        println!("{PURPLE}> > > Este diretório já está vazio. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }

    println!("{PURPLE}Arquivos presentes no diretório:{RESET}");
    diretorios[dir_idx].listar_conteudo();

    println!("{PURPLE}Digite o nome exato do arquivo que deseja remover:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut nome_arq = String::new();
    io::stdin().read_line(&mut nome_arq).unwrap();
    let nome_arq = nome_arq.trim();

    diretorios[dir_idx].remove_arquivo(nome_arq.to_string());
    thread::sleep(Duration::from_secs(2));

    println!("");
    println!("{PURPLE}Operação de remoção para o arquivo {} concluída com sucesso!{RESET}", nome_arq);
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}

fn terminal_alterar_permissao_arquivo(diretorios: &mut Vec<Diretorio>) {
    println!("");
    if diretorios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum diretório cadastrado. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }

    println!("{PURPLE}Selecione o índice do diretório onde está o arquivo:{RESET}");
    for (indice, diretorio) in diretorios.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, diretorio.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut dir_idx = String::new();
    io::stdin().read_line(&mut dir_idx).unwrap();
    let dir_idx: usize = dir_idx.trim().parse().unwrap();

    if diretorios[dir_idx].arquivos.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum arquivo neste diretório. < < <{RESET}");
        thread::sleep(Duration::from_secs(3));
        return;
    }

    println!("{PURPLE}Selecione o índice do arquivo para alterar as permissões:{RESET}");
    for (indice, arquivo) in diretorios[dir_idx].arquivos.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, arquivo.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    io::stdout().flush().unwrap();
    let mut arq_idx = String::new();
    io::stdin().read_line(&mut arq_idx).unwrap();
    let arq_idx: usize = arq_idx.trim().parse().unwrap();

    let ler_bool = |prompt: &str| -> bool {
        println!("{PURPLE}{} (1 para Sim / 0 para Não):{RESET}", prompt);
        print!("{DIM}user@user ➜  {RESET}");
        io::stdout().flush().unwrap();
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).unwrap();
        inp.trim() == "1"
    };

    println!("");
    println!("{PURPLE}--- Permissões do Dono (User) ---{RESET}");
    let u_r = ler_bool("Leitura (r)");
    let u_w = ler_bool("Escrita (w)");
    let u_x = ler_bool("Execução (x)");

    println!("");
    println!("{PURPLE}--- Permissões do Grupo (Group) ---{RESET}");
    let g_r = ler_bool("Leitura (r)");
    let g_w = ler_bool("Escrita (w)");
    let g_x = ler_bool("Execução (x)");

    println!("");
    println!("{PURPLE}--- Permissões de Outros (Others) ---{RESET}");
    let o_r = ler_bool("Leitura (r)");
    let o_w = ler_bool("Escrita (w)");
    let o_x = ler_bool("Execução (x)");

    let novas_permissoes = (
        Permissao::new(u_r, u_w, u_x),
        Permissao::new(g_r, g_w, g_x),
        Permissao::new(o_r, o_w, o_x),
    );

    diretorios[dir_idx].arquivos[arq_idx].alterar_permissao(novas_permissoes);
    thread::sleep(Duration::from_secs(2));

    println!("");
    println!("{PURPLE}Permissões do arquivo {} alteradas com sucesso! Novo status:{RESET}", diretorios[dir_idx].arquivos[arq_idx].nome);
    diretorios[dir_idx].arquivos[arq_idx].stat();
    println!("");

    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(5));
}