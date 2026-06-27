use std::io::{ self, Write };

fn main() {
    println!("Program Piramida Bintang Terbalik");
    println!("=================================");

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

    for i in 0.. tinggi_segitiga {
        for _j in 1..= i {
            print!(" ");
        }

        for _k in 1..= (tinggi_segitiga - i) {
            print!(" *");
        }

        println!();
    }


    //  VERSI IDIOMATIC
    //for i in (1..= tinggi_segitiga).rev() {
    //    let spasi = " ".repeat(tinggi_segitiga - i);
    //    let bintang = " *".repeat(i);

    //    println!("{}{}", spasi, bintang);
    //}
}
