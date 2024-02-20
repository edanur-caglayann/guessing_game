use std::io; /* ioKullanıcı girdisini elde etmek ve 
ardından sonucu çıktı olarak yazdırmak için girdi/çıktı kütüphanesini kapsama dahil etmemiz gerekir . */
use rand:: Rng; // Rastgele sayı üretmek için kullandığımız kütüphane
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    /*rand::thread_rng(): Bu ifade, mevcut iş parçacığı için bir rastgele sayı üreteci oluşturur.
    .gen_range(1..=100): Bu ifade, belirtilen aralıkta (1 ile 100 arasında, her iki sayı da dahil) rastgele bir sayı üretir.
    let secret_number = ...: Bu ifade, üretilen rastgele sayıyı secret_number adlı değişkene atar.
     */

    loop {
    println!("Please input your guess.");

    // Kullanıcı girişini depoalamak için bir değişken oluşturalım
    let mut guess= String:: new();

    io::stdin()
    // kullanıcıdan bir satır girişi alır ve bu girişi guess değişkenine kaydeder.
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() { 
    /*guess.trim(): Bu işlem, guess değişkeninin başında ve sonunda bulunan boşlukları kaldırır.
     Bu genellikle kullanıcıdan alınan girdinin temizlenmesi için kullanılır.
    .parse(): Bu işlem, guess değişkenini bir sayıya çevirmeye çalışır. Rust’ta, parse() fonksiyonu bir metni belirli bir veri tipine çevirir.
     */

    Ok(num) => num, // İşlem başarılı ise num değeri guess değişkenine atanır.
    Err(_) => continue, // Başarısız ise mevcut iterasyonu atlar ve bir sonraki iterasyona geçer
    };


    // {}bir yer tutucudur
    println!("You guessed: {}",guess);


    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
        println!("Too win!");
        break; // Doğru tahminden sonra çıkalım
        }
    }
}
}
