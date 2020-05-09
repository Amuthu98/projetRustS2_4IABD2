use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};


fn Exo4_pipes(){

    let ls = Command::new("ls")
    .arg("-alh")
    .stdout(Stdio::piped())
    .spawn()
    .expect(" ls? What is it?");

    let ls_stdout = Stdio::from(ls.stdout.expect("Failed"));
    let process = Command::new("grep")
    .arg("Hello")
    .stdin(ls_stdout)
    .spawn()
    .expect("Not hello? something wrong with grep");
}


fn Exo5_taches_fond(){

    // Amrta
}


fn main() -> std::io::Result<()> {

    let _stdin = io::stdin();
    use std::io::{self, Write};
    let stdout = io::stdout();
    let mut out = stdout;
    out.write(b"-->")?;
    // `?` sert à « propager l'erreur » dans la fonction appellante
    // c'est mieux que de crash avec un unwrap ou expect ;)
    out.flush()?;

    // On récupère la commande de l'utilisateur
    let mut user_input = String::with_capacity(256);
    _stdin.read_line(&mut user_input)?;

    
    // On execute la commande
    let status = Command::new(user_input.trim())
                     .status()
                     .expect("command not found");

    println!("status: {}", status);

    //assert!(status.success());

    

    Exo4_pipes();

    Ok(())

}
