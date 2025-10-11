use std::io;

fn main() {
    loop{
        println!("/Kalkulator Rust");
        println!("Pilih Operasi:");
        println!("1. Penjumlahan");
        println!("2. Pengurangan");
        println!("3. Pembagian");
        println!("4. Keluar");
        println!("Input :");
        let mut pilihan = String::new();
        io::stdin()
        .read_line(&mut pilihan)
        .expect("Angka tidak valid");

        match pilihan.trim() {
            "1" => {
                let hasil = tambah();
                println!("Hasil Penjumlahan {}",hasil);
            }
            "2" => {
                let hasil = kurang();
                println!("Hasil Penjumlahan {}",hasil);
            }
            "3" => {
                let hasil = bagi();
                println!("Hasil Pembagian {}", hasil);

            }
            "4" => {
                println!("Keluar dari kalkulator");
                break;        

            }
            _ => println!("Pilihan tidak Valid"),
        }
    }
}
fn tambah() -> i32{
    println!("Operasi Penjumlahan Masukan angka pertama");

    let mut a = String::new();
    
    io::stdin()
    .read_line(&mut a)
    .expect("Angka tidak terbaca masukan ulang");

    println!("Masukan angka kedua");

    let mut b= String::new();
    io::stdin()
    .read_line(&mut b)
    .expect("Angka tidak terbaca masukan ulang");

    let angka_a: i32 = a.trim().parse().expect("input bukan angka");
    let angka_b: i32 = b.trim().parse().expect("input bukan angka");

    let hasil = angka_a + angka_b;

    println!("Hasil Penjumlahan {} + {} = {}", angka_a,angka_b,hasil);
    println!("-------------------------------");
    hasil
}

fn kurang()->i32{
    println!("Operasi Pengurangan Masukan Angka Pertama");

    let mut a = String::new();
    io::stdin()
    .read_line(&mut a)
    .expect("Angka tidak terbaca masukan ulang");

    println!("Masukan angka ke dua");
    let mut b = String::new();
    io::stdin()
    .read_line(&mut b)
    .expect("Angka tidak terbaca masukan ulang");

    let angka_a: i32 = a.trim().parse().expect("input bukan angka");
    let angka_b: i32 = b.trim().parse().expect("input bukan angka");

    let hasil = angka_a - angka_b;

    println!("Hasil Pengurangan {} - {} = {}", angka_a,angka_b,hasil);
    println!("-------------------------------");
    hasil
}

fn bagi()-> f32{
    println!("Operasi Pembagian Masukan Angka Pertama");
    let mut a = String::new();
    io::stdin()
    .read_line(&mut a)
    .expect("Input bukan angka");

    println!("Masukan angka kedua");
    let mut b = String::new();
    io::stdin()
    .read_line(&mut b)
    .expect("input bukan angka");

    let angka_a: i32 = a.trim().parse().expect("Input bukan angka");
    let angka_b: i32= b.trim().parse().expect("Input bukan angka");

    let hasil = angka_a as f32 / angka_b as f32;
    println!("Hasil Pembagian {} : {} = {}", angka_a,angka_b,hasil);
    println!("-------------------------------");
    hasil
}
