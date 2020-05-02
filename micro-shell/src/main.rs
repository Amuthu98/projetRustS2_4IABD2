fn main() -> std::io::Result<()> {
    //use std::io;
    use std::process::Command;
    

    //let mut list_dir = Command::new("ls");

    //list_dir.status().expect("process failed to execute");
    //println!("status: {}", list_dir.status);

    let status = Command::new("ls")
                     .status()
                     .expect("failed to execute process");

    println!("status: {}", status);

    //assert!(status.success());

    let _stdin = io::stdin();
    use std::io::{self, Write};
    let stdout = io::stdout();
    let mut out = stdout;
    out.write(b"-->")?;
    // `?` sert à « propager l'erreur » dans la fonction appellante
    // c'est mieux que de crash avec un unwrap ou expect ;)
    out.flush()?;

    let mut user_input = String::with_capacity(256);
    _stdin.read_line(&mut user_input)?;

    fn Exo4_pipes(){
        use std::process::Stdio;
        

        let mut programme1 = Command::new("ls").arg("/")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

        
        let entree = programme1.stdin.as_mut().expect("Failed to open stdin");
        entree.write_all("Hello, world!".as_bytes()).expect("Failed to write to stdin");
        

        let output = programme1.wait_with_output().expect("Failed to read stdout");
       
        //assert_eq!(String::from_utf8_lossy(&output.stdout), "!dlrow ,olleH");

        let mut programme2 = Command::new("grep").arg("Hello")
         .stdin(output)
         .spawn();


          assert_eq!("hello",get_stdout(programme2).trim());

    }

    Exo4_pipes();
        
    /*
    fn Exo4_pipes(){
        use std::process::Stdio;

        let programme1 = Command::new("ls").arg("/")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

        let programme2 = Command::new("grep").arg("Hello")
         .stdin(programme1)
         .spawn();
         //.unwrap_or_else(|e| { panic!("Error to execute process: {}", e) })
    }*/

    Ok(())

}
