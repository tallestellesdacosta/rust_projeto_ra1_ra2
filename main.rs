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
    fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Self {
        Arquivo {
            nome: nome,
            tamanho: tamanho,
            permissoes: (
                Permissao::new(true, true, false),
                Permissao::new(true, true, false),
                Permissao::new(true, false, false),
            ),
            uid_dono: uid,
            gid_grupo: gid,
        }
    }

    fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {
        self.permissoes = permissoes;
    }

    fn stat(&self) {
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        println!(
            "Permissoes: ({}{}{} / {}{}{}{}{}{}{}{}{})",
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

struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool,
}

impl Permissao {
    fn new(leitura: bool, escrita: bool, execucao: bool) -> Self {
        Permissao {
            leitura: leitura,
            escrita: escrita,
            execucao: execucao,
        }
    }

    fn octal(&self) -> u8 {
        (self.leitura as u8 * 4) + (self.escrita as u8 * 2) + (self.execucao as u8 * 1)
    }
}

struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
}

impl Diretorio {
    fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), uid_dono: u16) -> Self {
        Diretorio {
            nome: nome,
            arquivos: Vec::new(),
            permissoes: permissoes,
            uid_dono: uid_dono,
        }
    }

    fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    fn remove_arquivo(&mut self, nome_arquivo: String) {
        for i in 0..self.arquivos.len() {
            if self.arquivos[i].nome == nome_arquivo {
                self.arquivos.remove(i);
                break;
            }
        }
    }

    fn listar_conteudo(&self) {
        println!("DIRETORIO {}:", self.nome);
        if self.arquivos.len() == 0 {
            println!("> > > Não há nenhum arquivo neste diretório. < < <");
        } else {
            for arquivo in &self.arquivos {
                println!("- {}", arquivo.nome);
            }
        }
    }
}

struct Usuario {
    nome: String,
    uid: u16,
    grupos_ids: Vec<u16>,
}

impl Usuario {
    fn new(nome: String, uid: u16) -> Self {
        Usuario {
            nome: nome,
            uid: uid,
            grupos_ids: Vec::new(),
        }
    }

    fn adiciona_grupo(&mut self, gid: u16) {
        self.grupos_ids.push(gid);
    }

    fn remove_grupo(&mut self, gid: u16) {
        for i in 0..self.grupos_ids.len() {
            if self.grupos_ids[i] == gid {
                self.grupos_ids.remove(i);
                break;
            }
        }
    }

    fn listar_grupos(&self) {
        if self.grupos_ids.len() == 0 {
            println!("Nenhum grupo encontrado.");
        } else {
            for gid in &self.grupos_ids {
                println!("{}", gid);
            }
        }
    }
}

struct Grupo {
    nome: String,
    gid: u16,
    membros_uids: Vec<u16>,
}

impl Grupo {
    fn new(nome: String, gid: u16) -> Self {
        Grupo {
            nome: nome,
            gid: gid,
            membros_uids: Vec::new(),
        }
    }

    fn adiciona_membro(&mut self, uid: u16) {
        self.membros_uids.push(uid);
    }

    fn remove_membro(&mut self, uid: u16) {
        for i in 0..self.membros_uids.len() {
            if self.membros_uids[i] == uid {
                self.membros_uids.remove(i);
                break;
            }
        }
    }

    fn listar_membros(&self) {
        if self.membros_uids.len() == 0 {
            println!("Nenhum membro cadastrado.");
        } else {
            for uid in &self.membros_uids {
                println!("{}", uid);
            }
        }
    }
}

fn ler_texto() -> String {
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().to_string()
}

fn ler_u16() -> u16 {
    ler_texto().parse().unwrap_or(0)
}

fn ler_u64() -> u64 {
    ler_texto().parse().unwrap_or(0)
}

fn ler_usize() -> usize {
    ler_texto().parse().unwrap_or(99999)
}

fn ler_bool(prompt: &str) -> bool {
    println!("{PURPLE}{} (1 para Sim / 0 para Não):{RESET}", prompt);
    print!("{DIM}user@user ➜  {RESET}");
    ler_texto() == "1"
}

fn pausar_gato() {
    println!("");
    println!(r#"       |\---/|
       | ,_, |
        \_`_/-..----.
    ___/ `   ' ,""+ ""
    (__...'   __\    |`.___.';
    (_,...'(_,.`__)/'.....+"#);
    println!("");
    println!("{DIM}Nosso coordena-gato, muito ocupado, te levará de volta ao menu em poucos segundos.{RESET}");
    thread::sleep(Duration::from_secs(3));
}

fn main() {
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
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [1A] Novo usuário            [1C] Editar usuário              {DIM}  |  ,-.    ,-'         |  / {RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [1B] Listar usuários         [1D] Apagar usuário              {DIM}  |  | (   |            | /  {RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET}                                                                 {DIM}  )  |  \\  `.___________|/   {RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}{UNDERLINE}2. Grupos{RESET}                                                       {DIM}  `--'   `--'                {RESET} {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [2A] Novo grupo              [2E] Vincular usr a grupo                                      {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [2B] Listar grupos           [2F] Desvincular usr de grupo                                  {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [2C] Editar grupo            [2G] Adicionar membro a grupo                                  {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [2D] Apagar grupo            [2H] Remover membro de grupo                                   {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [2I] Listar grupos de um usr [2J] Listar membros de grupo                                   {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET}                                                                                               {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}{UNDERLINE}3. Diretórios{RESET}                                                                                 {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [3A] Novo diretório          [3C] Editar diretório                                          {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [3B] Listar diretório        [3D] Apagar diretório                                          {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET}                                                                                               {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}{UNDERLINE}4. Arquivos{RESET}                                                                                   {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [4A] Novo arquivo            [4C] Alterar permissões arquivo                                {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}●{RESET} [4B] Remover arquivo                                                                        {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET}                                                                                               {PURPLE}│{RESET}");
        println!("{PURPLE}│{RESET} {PURPLE}▌{RESET} {DIM}Dica: Digite [0] para finalizar a execução.{RESET}                                                 {PURPLE}│{RESET}");
        println!("{PURPLE}└───────────────────────────────────────────────────────────────────────────────────────────────┘{RESET}");
        println!();

        print!("{PURPLE}user@user ➜  {RESET}");
        let command = ler_texto().to_uppercase();

        match command.as_str() {
            "1A" => terminal_novo_usuario(&mut usuarios),
            "1B" => terminal_listar_todos_usuarios(&usuarios),
            "1C" => terminal_editar_usuario(&mut usuarios),
            "1D" => terminal_apagar_usuario(&mut usuarios),
            
            "2A" => terminal_novo_grupo(&mut grupos),
            "2B" => terminal_listar_todos_grupos(&grupos),
            "2C" => terminal_editar_grupo(&mut grupos),
            "2D" => terminal_apagar_grupo(&mut grupos),
            "2E" => terminal_vincular_usuario_grupo(&mut usuarios, &grupos),
            "2F" => terminal_desvincular_usuario_grupo(&mut usuarios, &grupos),
            "2G" => terminal_adicionar_membro(&mut grupos, &usuarios),
            "2H" => terminal_remover_membro(&mut grupos, &usuarios),
            "2I" => terminal_listar_grupos(&usuarios),
            "2J" => terminal_listar_usuarios(&grupos),
            
            "3A" => terminal_novo_diretorio(&mut diretorios),
            "3B" => terminal_listar_diretorio(&diretorios),
            "3C" => terminal_editar_diretorio(&mut diretorios),
            "3D" => terminal_apagar_diretorio(&mut diretorios),
            
            "4A" => terminal_novo_arquivo(&mut diretorios, &grupos, &usuarios),
            "4B" => terminal_remover_arquivo(&mut diretorios),
            "4C" => terminal_alterar_permissao_arquivo(&mut diretorios),
            
            "0" => break,
            _ => {
                println!("\n{DIM}Comando não reconhecido. Pressione ENTER para tentar novamente.{RESET}");
                ler_texto();
            }
        }
    }
}

fn terminal_novo_usuario(usuarios: &mut Vec<Usuario>) {
    println!("\n{PURPLE}Digite o nome do novo usuário:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    let nome_usuario = ler_texto();

    println!("{PURPLE}Digite o UID do novo usuário:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    let uid = ler_u16();

    usuarios.push(Usuario::new(nome_usuario.clone(), uid));
    println!("\n{PURPLE}O usuário {} foi criado com sucesso!{RESET}", nome_usuario);
    pausar_gato();
}

fn terminal_listar_todos_usuarios(usuarios: &Vec<Usuario>) {
    println!("");
    if usuarios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum usuário cadastrado. < < <{RESET}");
    } else {
        println!("{PURPLE}Usuários cadastrados:{RESET}");
        for (i, u) in usuarios.iter().enumerate() {
            println!("{DIM}{} - Nome: {} | UID: {}{RESET}", i, u.nome, u.uid);
        }
    }
    pausar_gato();
}

fn terminal_editar_usuario(usuarios: &mut Vec<Usuario>) {
    println!("");
    if usuarios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum usuário cadastrado. < < <{RESET}");
        pausar_gato();
        return;
    }
    println!("{PURPLE}Selecione o índice do usuário para editar:{RESET}");
    for (i, u) in usuarios.iter().enumerate() {
        println!("{DIM}{} - {} (UID: {}){RESET}", i, u.nome, u.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let idx = ler_usize();
    if idx >= usuarios.len() {
        println!("{PURPLE}Índice inválido!{RESET}");
        pausar_gato();
        return;
    }
    println!("{PURPLE}Digite o novo nome do usuário:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    usuarios[idx].nome = ler_texto();

    println!("{PURPLE}Digite o novo UID do usuário:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    usuarios[idx].uid = ler_u16();

    println!("\n{PURPLE}Usuário atualizado com sucesso!{RESET}");
    pausar_gato();
}

fn terminal_apagar_usuario(usuarios: &mut Vec<Usuario>) {
    println!("");
    if usuarios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum usuário cadastrado. < < <{RESET}");
        pausar_gato();
        return;
    }
    println!("{PURPLE}Selecione o índice do usuário para apagar:{RESET}");
    for (i, u) in usuarios.iter().enumerate() {
        println!("{DIM}{} - {} (UID: {}){RESET}", i, u.nome, u.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let idx = ler_usize();
    if idx < usuarios.len() {
        usuarios.remove(idx);
        println!("\n{PURPLE}Usuário removido com sucesso!{RESET}");
    } else {
        println!("{PURPLE}Índice inválido!{RESET}");
    }
    pausar_gato();
}

fn terminal_novo_grupo(grupos: &mut Vec<Grupo>) {
    println!("\n{PURPLE}Digite o nome do novo grupo:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    let nome_grupo = ler_texto();

    println!("{PURPLE}Digite o GID do novo grupo:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    let gid = ler_u16();

    grupos.push(Grupo::new(nome_grupo.clone(), gid));
    println!("\n{PURPLE}O grupo {} foi criado com sucesso!{RESET}", nome_grupo);
    pausar_gato();
}

fn terminal_listar_todos_grupos(grupos: &Vec<Grupo>) {
    println!("");
    if grupos.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum grupo cadastrado. < < <{RESET}");
    } else {
        println!("{PURPLE}Grupos cadastrados:{RESET}");
        for (i, g) in grupos.iter().enumerate() {
            println!("{DIM}{} - Nome: {} | GID: {}{RESET}", i, g.nome, g.gid);
        }
    }
    pausar_gato();
}

fn terminal_editar_grupo(grupos: &mut Vec<Grupo>) {
    println!("");
    if grupos.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum grupo cadastrado. < < <{RESET}");
        pausar_gato();
        return;
    }
    println!("{PURPLE}Selecione o índice do grupo para editar:{RESET}");
    for (i, g) in grupos.iter().enumerate() {
        println!("{DIM}{} - {} (GID: {}){RESET}", i, g.nome, g.gid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let idx = ler_usize();
    if idx >= grupos.len() {
        println!("{PURPLE}Índice inválido!{RESET}");
        pausar_gato();
        return;
    }
    println!("{PURPLE}Digite o novo nome do grupo:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    grupos[idx].nome = ler_texto();

    println!("{PURPLE}Digite o novo GID do grupo:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    grupos[idx].gid = ler_u16();

    println!("\n{PURPLE}Grupo atualizado com sucesso!{RESET}");
    pausar_gato();
}

fn terminal_apagar_grupo(grupos: &mut Vec<Grupo>) {
    println!("");
    if grupos.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum grupo cadastrado. < < <{RESET}");
        pausar_gato();
        return;
    }
    println!("{PURPLE}Selecione o índice do grupo para apagar:{RESET}");
    for (i, g) in grupos.iter().enumerate() {
        println!("{DIM}{} - {} (GID: {}){RESET}", i, g.nome, g.gid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let idx = ler_usize();
    if idx < grupos.len() {
        grupos.remove(idx);
        println!("\n{PURPLE}Grupo removido com sucesso!{RESET}");
    } else {
        println!("{PURPLE}Índice inválido!{RESET}");
    }
    pausar_gato();
}

fn terminal_vincular_usuario_grupo(usuarios: &mut Vec<Usuario>, grupos: &Vec<Grupo>) {
    println!("");
    if usuarios.len() == 0 || grupos.len() == 0 {
        println!("{PURPLE}> > > É necessário ter ao menos um usuário e um grupo cadastrados. < < <{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Selecione o índice do usuário:{RESET}");
    for (indice, usuario) in usuarios.iter().enumerate() {
        println!("{DIM}{} - {} (UID: {}){RESET}", indice, usuario.nome, usuario.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let usr_idx = ler_usize();

    println!("{PURPLE}Selecione o índice do grupo para vincular:{RESET}");
    for (indice, grupo) in grupos.iter().enumerate() {
        println!("{DIM}{} - {} (GID: {}){RESET}", indice, grupo.nome, grupo.gid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let grp_idx = ler_usize();

    if usr_idx < usuarios.len() && grp_idx < grupos.len() {
        let gid = grupos[grp_idx].gid;
        usuarios[usr_idx].adiciona_grupo(gid);
        println!("\n{PURPLE}O usuário {} foi vinculado ao grupo {} com sucesso!{RESET}", usuarios[usr_idx].nome, grupos[grp_idx].nome);
    } else {
        println!("{PURPLE}Índice inválido!{RESET}");
    }
    pausar_gato();
}

fn terminal_desvincular_usuario_grupo(usuarios: &mut Vec<Usuario>, grupos: &Vec<Grupo>) {
    println!("");
    if usuarios.len() == 0 || grupos.len() == 0 {
        println!("{PURPLE}> > > É necessário ter ao menos um usuário e um grupo cadastrados. < < <{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Selecione o índice do usuário:{RESET}");
    for (indice, usuario) in usuarios.iter().enumerate() {
        println!("{DIM}{} - {} (UID: {}){RESET}", indice, usuario.nome, usuario.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let usr_idx = ler_usize();

    if usr_idx >= usuarios.len() {
        println!("{PURPLE}Índice inválido!{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Digite o GID do grupo que deseja desvincular deste usuário:{RESET}");
    usuarios[usr_idx].listar_grupos();
    print!("{DIM}user@user ➜  {RESET}");
    let gid = ler_u16();

    usuarios[usr_idx].remove_grupo(gid);
    println!("\n{PURPLE}O grupo (GID: {}) foi desvinculado do usuário {} com sucesso!{RESET}", gid, usuarios[usr_idx].nome);
    pausar_gato();
}

fn terminal_adicionar_membro(grupos: &mut Vec<Grupo>, usuarios: &Vec<Usuario>) {
    println!("");
    if grupos.len() == 0 || usuarios.len() == 0 {
        println!("{PURPLE}> > > É necessário ter ao menos um grupo e um usuário cadastrados. < < <{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Selecione o índice do grupo:{RESET}");
    for (indice, grupo) in grupos.iter().enumerate() {
        println!("{DIM}{} - {} (GID: {}){RESET}", indice, grupo.nome, grupo.gid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let grp_idx = ler_usize();

    println!("{PURPLE}Selecione o índice do usuário para adicionar como membro:{RESET}");
    for (indice, usuario) in usuarios.iter().enumerate() {
        println!("{DIM}{} - {} (UID: {}){RESET}", indice, usuario.nome, usuario.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let usr_idx = ler_usize();

    if grp_idx < grupos.len() && usr_idx < usuarios.len() {
        let uid = usuarios[usr_idx].uid;
        grupos[grp_idx].adiciona_membro(uid);
        println!("\n{PURPLE}O membro {} foi adicionado ao grupo {} com sucesso!{RESET}", usuarios[usr_idx].nome, grupos[grp_idx].nome);
    } else {
        println!("{PURPLE}Índice inválido!{RESET}");
    }
    pausar_gato();
}

fn terminal_remover_membro(grupos: &mut Vec<Grupo>, usuarios: &Vec<Usuario>) {
    println!("");
    if grupos.len() == 0 || usuarios.len() == 0 {
        println!("{PURPLE}> > > É necessário ter ao menos um grupo e um usuário cadastrados. < < <{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Selecione o índice do grupo:{RESET}");
    for (indice, grupo) in grupos.iter().enumerate() {
        println!("{DIM}{} - {} (GID: {}){RESET}", indice, grupo.nome, grupo.gid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let grp_idx = ler_usize();

    if grp_idx >= grupos.len() {
        println!("{PURPLE}Índice inválido!{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Digite o UID do membro que deseja remover deste grupo:{RESET}");
    grupos[grp_idx].listar_membros();
    print!("{DIM}user@user ➜  {RESET}");
    let uid = ler_u16();

    grupos[grp_idx].remove_membro(uid);
    println!("\n{PURPLE}O membro (UID: {}) foi removido do grupo {} com sucesso!{RESET}", uid, grupos[grp_idx].nome);
    pausar_gato();
}

fn terminal_listar_grupos(usuarios: &Vec<Usuario>) {
    println!("");
    if usuarios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum usuário cadastrado. < < <{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Selecione o índice do usuário para listar seus grupos:{RESET}");
    for (indice, usuario) in usuarios.iter().enumerate() {
        println!("{DIM}{} - {} (UID: {}){RESET}", indice, usuario.nome, usuario.uid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let usr_idx = ler_usize();

    if usr_idx < usuarios.len() {
        println!("\n{PURPLE}Grupos vinculados ao usuário {}:{RESET}", usuarios[usr_idx].nome);
        usuarios[usr_idx].listar_grupos();
    } else {
        println!("{PURPLE}Índice inválido!{RESET}");
    }
    pausar_gato();
}

fn terminal_listar_usuarios(grupos: &Vec<Grupo>) {
    println!("");
    if grupos.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum grupo cadastrado. < < <{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Selecione o índice do grupo para listar seus membros:{RESET}");
    for (indice, grupo) in grupos.iter().enumerate() {
        println!("{DIM}{} - {} (GID: {}){RESET}", indice, grupo.nome, grupo.gid);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let grp_idx = ler_usize();

    if grp_idx < grupos.len() {
        println!("\n{PURPLE}Membros pertencentes ao grupo {}:{RESET}", grupos[grp_idx].nome);
        grupos[grp_idx].listar_membros();
    } else {
        println!("{PURPLE}Índice inválido!{RESET}");
    }
    pausar_gato();
}

fn terminal_novo_diretorio(diretorios: &mut Vec<Diretorio>) {
    println!("\n{PURPLE}Digite o nome do novo diretório:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    let nome = ler_texto();

    println!("{PURPLE}Digite o UID do dono do diretório:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    let uid_dono = ler_u16();

    let novo_diretorio = Diretorio::new(
        nome.clone(),
        (
            Permissao::new(true, true, true),
            Permissao::new(true, true, true),
            Permissao::new(true, true, true),
        ),
        uid_dono,
    );

    diretorios.push(novo_diretorio);
    println!("\n{PURPLE}O diretório {} foi criado com sucesso!{RESET}", nome);
    pausar_gato();
}

fn terminal_listar_diretorio(diretorios: &Vec<Diretorio>) {
    println!("");
    if diretorios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum diretório criado para listagem. < < <{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Selecione o índice do diretório que deseja listar:{RESET}");
    for (indice, diretorio) in diretorios.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, diretorio.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let dir_idx = ler_usize();

    if dir_idx < diretorios.len() {
        println!("");
        diretorios[dir_idx].listar_conteudo();

        if diretorios[dir_idx].arquivos.len() > 0 {
            println!("\n{PURPLE}Detalhes (stat) dos arquivos no diretório:{RESET}");
            for arquivo in &diretorios[dir_idx].arquivos {
                println!("{DIM}--------------------------------------------{RESET}");
                arquivo.stat();
            }
        }
    } else {
        println!("{PURPLE}Índice inválido!{RESET}");
    }
    pausar_gato();
}

fn terminal_editar_diretorio(diretorios: &mut Vec<Diretorio>) {
    println!("");
    if diretorios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum diretório cadastrado. < < <{RESET}");
        pausar_gato();
        return;
    }
    println!("{PURPLE}Selecione o índice do diretório para editar:{RESET}");
    for (i, d) in diretorios.iter().enumerate() {
        println!("{DIM}{} - {} (UID Dono: {}){RESET}", i, d.nome, d.uid_dono);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let idx = ler_usize();
    if idx >= diretorios.len() {
        println!("{PURPLE}Índice inválido!{RESET}");
        pausar_gato();
        return;
    }
    println!("{PURPLE}Digite o novo nome do diretório:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    diretorios[idx].nome = ler_texto();

    println!("{PURPLE}Digite o novo UID do dono do diretório:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    diretorios[idx].uid_dono = ler_u16();

    println!("\n{PURPLE}Diretório atualizado com sucesso!{RESET}");
    pausar_gato();
}

fn terminal_apagar_diretorio(diretorios: &mut Vec<Diretorio>) {
    println!("");
    if diretorios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum diretório cadastrado. < < <{RESET}");
        pausar_gato();
        return;
    }
    println!("{PURPLE}Selecione o índice do diretório para apagar:{RESET}");
    for (i, d) in diretorios.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", i, d.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let idx = ler_usize();
    if idx < diretorios.len() {
        diretorios.remove(idx);
        println!("\n{PURPLE}Diretório removido com sucesso!{RESET}");
    } else {
        println!("{PURPLE}Índice inválido!{RESET}");
    }
    pausar_gato();
}

fn terminal_novo_arquivo(diretorios: &mut Vec<Diretorio>, grupos: &Vec<Grupo>, usuarios: &Vec<Usuario>) {
    println!("");
    if diretorios.len() == 0 || grupos.len() == 0 || usuarios.len() == 0 {
        println!("{PURPLE}> > > Crie ao menos um diretório, um grupo e um usuário antes de criar arquivos. < < <{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Digite o número do diretório pai:{RESET}");
    for (indice, diretorio) in diretorios.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, diretorio.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let diretorio_idx = ler_usize();

    println!("{PURPLE}Digite o número do grupo para o arquivo:{RESET}");
    for (indice, grupo) in grupos.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, grupo.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let grupo_idx = ler_usize();

    println!("{PURPLE}Digite o número do usuário dono do arquivo:{RESET}");
    for (indice, usuario) in usuarios.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, usuario.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let usuario_idx = ler_usize();

    if diretorio_idx >= diretorios.len() || grupo_idx >= grupos.len() || usuario_idx >= usuarios.len() {
        println!("{PURPLE}Um dos índices informados é inválido!{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Digite o nome para o novo arquivo:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    let arquivo_nome = ler_texto();

    println!("{PURPLE}Digite o tamanho para o novo arquivo (em bytes):{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    let tamanho = ler_u64();

    diretorios[diretorio_idx].adiciona_arquivo(Arquivo::new(
        arquivo_nome.clone(),
        tamanho,
        usuarios[usuario_idx].uid,
        grupos[grupo_idx].gid,
    ));

    println!("\n{PURPLE}O arquivo {} foi criado com sucesso!{RESET}", arquivo_nome);
    pausar_gato();
}

fn terminal_remover_arquivo(diretorios: &mut Vec<Diretorio>) {
    println!("");
    if diretorios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum diretório cadastrado. < < <{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Selecione o índice do diretório de onde deseja remover o arquivo:{RESET}");
    for (indice, diretorio) in diretorios.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, diretorio.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let dir_idx = ler_usize();

    if dir_idx >= diretorios.len() {
        println!("{PURPLE}Índice inválido!{RESET}");
        pausar_gato();
        return;
    }

    if diretorios[dir_idx].arquivos.len() == 0 {
        println!("{PURPLE}> > > Este diretório já está vazio. < < <{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Arquivos presentes no diretório:{RESET}");
    diretorios[dir_idx].listar_conteudo();

    println!("{PURPLE}Digite o nome exato do arquivo que deseja remover:{RESET}");
    print!("{DIM}user@user ➜  {RESET}");
    let nome_arq = ler_texto();

    diretorios[dir_idx].remove_arquivo(nome_arq.clone());
    println!("\n{PURPLE}Operação de remoção para o arquivo {} finalizada!{RESET}", nome_arq);
    pausar_gato();
}

fn terminal_alterar_permissao_arquivo(diretorios: &mut Vec<Diretorio>) {
    println!("");
    if diretorios.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum diretório cadastrado. < < <{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Selecione o índice do diretório onde está o arquivo:{RESET}");
    for (indice, diretorio) in diretorios.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, diretorio.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let dir_idx = ler_usize();

    if dir_idx >= diretorios.len() {
        println!("{PURPLE}Índice inválido!{RESET}");
        pausar_gato();
        return;
    }

    if diretorios[dir_idx].arquivos.len() == 0 {
        println!("{PURPLE}> > > Não há nenhum arquivo neste diretório. < < <{RESET}");
        pausar_gato();
        return;
    }

    println!("{PURPLE}Selecione o índice do arquivo para alterar as permissões:{RESET}");
    for (indice, arquivo) in diretorios[dir_idx].arquivos.iter().enumerate() {
        println!("{DIM}{} - {}{RESET}", indice, arquivo.nome);
    }
    print!("{DIM}user@user ➜  {RESET}");
    let arq_idx = ler_usize();

    if arq_idx >= diretorios[dir_idx].arquivos.len() {
        println!("{PURPLE}Índice inválido!{RESET}");
        pausar_gato();
        return;
    }

    println!("\n{PURPLE}--- Permissões do Dono (User) ---{RESET}");
    let u_r = ler_bool("Leitura (r)");
    let u_w = ler_bool("Escrita (w)");
    let u_x = ler_bool("Execução (x)");

    println!("\n{PURPLE}--- Permissões do Grupo (Group) ---{RESET}");
    let g_r = ler_bool("Leitura (r)");
    let g_w = ler_bool("Escrita (w)");
    let g_x = ler_bool("Execução (x)");

    println!("\n{PURPLE}--- Permissões de Outros (Others) ---{RESET}");
    let o_r = ler_bool("Leitura (r)");
    let o_w = ler_bool("Escrita (w)");
    let o_x = ler_bool("Execução (x)");

    let novas_permissoes = (
        Permissao::new(u_r, u_w, u_x),
        Permissao::new(g_r, g_w, g_x),
        Permissao::new(o_r, o_w, o_x),
    );

    diretorios[dir_idx].arquivos[arq_idx].alterar_permissao(novas_permissoes);

    println!("\n{PURPLE}Permissões do arquivo {} alteradas com sucesso! Novo status:{RESET}", diretorios[dir_idx].arquivos[arq_idx].nome);
    diretorios[dir_idx].arquivos[arq_idx].stat();
    pausar_gato();
}