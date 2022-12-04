// Ajout de clippy
#![deny(clippy::all)]

fn main() {
    // Variable into Rust
    let _name = "john";
    // mut to make it mutable
    let mut _mutable_name = "john";
    _mutable_name = "Max";
    // Shadowing
    {
        let _name = "Sylvain";
        println!("Your name is  {}", _name)
    }

    // Constante
    const MY_AGE: i32 = 28;
    println!("Your name is  {}", MY_AGE);
    println!("Your name is  {}", _name);
}

// cargo-watch -qc -x run -x clippy
