use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

// fn Exo4_pipes(){        
//         let mut programme1 = Command::new("ls").arg("/")
//         .stdin(Stdio::piped())
//         .stdout(Stdio::piped())
//         .spawn()
//         .expect("Failed to spawn child process");

        
//         let entree = programme1.stdin.as_mut().expect("Failed to open stdin");
//         entree.write_all("Hello, world!".as_bytes()).expect("Failed to write to stdin");
        

//         let output = programme1.wait_with_output().expect("Failed to read stdout");
       
//         //assert_eq!(String::from_utf8_lossy(&output.stdout), "!dlrow ,olleH");

//         let mut programme2 = Command::new("grep").arg("Hello")
//          .stdin(output)
//          .spawn();


//           assert_eq!("hello",get_stdout(programme2).trim());

//     }


fn Exo4_pipesV2(){

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


fn main() -> std::io::Result<()> {

    let _stdin = io::stdin();
    use std::io::{self, Write};
    let stdout = io::stdout();
    let mut out = stdout;
    out.write(b"-->")?;
    // `?` sert à « propager l'erreur » dans la fonction appellante
    // c'est mieux que de crash avec un unwrap ou expect ;)
    out.flush()?;

    // On réupère la commande de l'utilisateur
    let mut user_input = String::with_capacity(256);
    _stdin.read_line(&mut user_input)?;

    
    // On execute la commande
    let status = Command::new(user_input.trim())
                     .status()
                     .expect("command not found");

    println!("status: {}", status);

    //assert!(status.success());

    

    Exo4_pipesV2();

    Ok(())

}
