use std::io::{ self, Write };

fn main() {
    println!("Program Persegi Bintang");
    println!("=======================");

    print!("Input besar persegi: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input!");

    let besar_persegi: usize = match input.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukan angka yang valid!");
            return;
        }
    };

    for _i in 1..= besar_persegi {
        for _j in 1..= besar_persegi {
            print!(" *");
        }

        println!();
    }


    // VERSI IDIOMATIC
    // let satu_baris = " *".repeat(besar_persegi);

    // for _ in 0.. besar_persegi {
    //     println!("{}", satu_baris);
    // }
}
