use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};


fn exo4_pipes(){

    let ls = Command::new("ls")
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




fn main() -> std::io::Result<()> {
    use std::collections::VecDeque;

    let mut stock_prgm = VecDeque::new();
    
    loop{
        
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
        
        if user_input.trim() == "" {
            continue
            } else if user_input.trim() == "exit" {
            break
            }
    

        let mut user_input_split = user_input.trim().split(" ");

        

        let commande = user_input_split.next().unwrap();
        let arg = user_input_split;
        
        
        println!("{}",commande);

        let mut prg=Command::new(commande)
                    .args(arg)
                    .spawn()
                    .unwrap();

        //prg.wait().expect("failed to wait on child");
        
        // Partie 5

        match prg.try_wait() {
            Ok(Some(status)) => println!("exited with: {}", status),
            Ok(None) => {
                let res = prg.wait();
                println!("Etat: {:?}", res);
                stock_prgm.push_back(prg);
                // On vérifie les processus en cours
                println!("Stockage: {:?}", stock_prgm);

            }
            Err(e) => println!("Erreur lors de l'attente du wait: {}", e),
        }

        
        

    
    }
    exo4_pipes();
    Ok(())
    

}
