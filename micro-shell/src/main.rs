fn main() -> std::io::Result<()> {
    //use std::io;
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
    Ok(())

}
