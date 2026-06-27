use::std::io::{ self, Write };

fn main() {
    println!("Program Persegi Panjang Bintang");
    println!("===============================");

    print!("Input tinggi persegi: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut tinggi = String::new();
    io::stdin()
        .read_line(&mut tinggi)
        .expect("Gagal membaca input!");

    let tinggi_persegi_panjang: usize = match tinggi.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukan angka yang valid!");
            return;
        }
    };

    print!("Input lebar persegi: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut lebar = String::new();
    io::stdin()
        .read_line(&mut lebar)
        .expect("Gagal membaca input!");

    let lebar_persegi_panjang: usize = match lebar.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukan angka yang valid!");
            return;
        }
    };

    for _i in 1..= tinggi_persegi_panjang {
        for _j in 1..= lebar_persegi_panjang {
            print!(" *");
        }

        println!();
    }


    //  VERSI IDIOMATIC
    //let satu_baris = " *".repeat(tinggi_persegi_panjang);

    //for _ in 0.. lebar_persegi_panjang {
    //    println!("{}", satu_baris);
    //}
}