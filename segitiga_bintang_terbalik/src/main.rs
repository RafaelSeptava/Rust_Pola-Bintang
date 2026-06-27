use std::io::{ self, Write };

fn main() {
    println!("Program Segitiga Bintang Terbalik");
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

    for i in 0..= tinggi_segitiga {
        for _j in 1..= tinggi_segitiga - i {
            print!(" *");
        }

        println!();
    }


    //for i in (1..= tinggi_segitiga).rev() {
    //    println!("{}", " *".repeat(i));
    //}
}
