use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::env;
use std::collections::HashMap;

fn Environnement(){

    let _stdin = io::stdin();
    use std::io::{self, Write};
    let stdout = io::stdout();
    let mut out = stdout;
    out.write(b"-->")?;
    out.flush()?;

    // On récupère la commande de l'utilisateur
    let mut user_input = String::with_capacity(256);
    _stdin.read_line(&mut user_input)?;

    
    // On execute la commande
    let filtered_env : HashMap<String, String> =
    env::vars().filter(|&(ref k, _)|
        k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH"
    ).collect();

    let status = Command::new(user_input.trim())
                     .env_clear()
                     .envs(&filtered_env)
                     .status()
                     .expect("command not found or printenv failed to start");

    println!("status: {}", status);
}


fn main() -> std::io::Result<()> {

    Environnement()
    Ok(())

}