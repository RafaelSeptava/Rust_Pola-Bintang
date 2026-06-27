use std::io::{ self, Write };

fn main() {
    println!("Program Belah Ketupat Bintang");
    println!("=============================");

    print!("Input lebar belah ketupat: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input_lebar = String::new();
    io::stdin()
        .read_line(&mut input_lebar)
        .expect("Gagal membaca input!");

    let lebar_belah_ketupat: usize = match input_lebar.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukan angka yang valid!");
            return;
        }
    };

    for i in 1..= lebar_belah_ketupat {
        for _j in 1..= lebar_belah_ketupat - i {
            print!(" ");
        }

        for _k in 1..= i {
            print!(" *");
        }

        println!();
    }

    for i in 1.. lebar_belah_ketupat {
        for _j in 1..= i {
            print!(" ");
        }

        for _k in 1..= lebar_belah_ketupat - i {
            print!(" *");
        }

        println!();
    }

    //  VERSI IDIOMATIC
    // for i in 1..= lebar_belah_ketupat {
    //     let spasi = " ".repeat(lebar_belah_ketupat - i);
    //     let bintang = " *".repeat(i);
    //     println!("{}{}", spasi, bintang);
    // }

    // for i in (1.. lebar_belah_ketupat).rev() {
    //     let spasi = " ".repeat(lebar_belah_ketupat - i);
    //     let bintang = " *".repeat(i);
    //     println!("{}{}", spasi, bintang);
    // }
}
