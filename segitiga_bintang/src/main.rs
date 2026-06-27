use std::io::{ self, Write };

fn main() {
    println!("Program Segitiga Bintang");
    println!("========================");

    print!("Input tinggi segitiga: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input!");

    let tinggi_segitiga: usize = match input.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukan angka yang valid!");
            return;
        }
    };

    for i in 1..= tinggi_segitiga {
        for _j in 1..= i {
            print!(" *");
        }

        println!();
    }


    // VERSI IDIOMATIC
    //for i in 1..= tinggi_segitiga {
    //    println!("{}", " *".repeat(i));
    //}
}
