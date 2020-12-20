fn main() {
    // komentar sebaris
    /* 
        komentar multi baris
    */
    println!(); // membuat baris baru
    println!("Hello, world!");
    println!("format {} arguments", "some"); //prints format some arguments
    hash_map();
}
fn hash_map()/* ini mirip dictionary python */ {
    use std::collections::HashMap; 
    let mut v = HashMap::new();
    v.insert("1","satu");
    v.insert("2","dua");
    v.insert("3","tiga");

    println!("isi hashmap {:?}",v);
}
fn _vector() /* ini mirip list python */ {
    let mut arr = Vec::new();
    arr.push(10);
    arr.push(20);
    arr.push(30);
    arr.push(40);

    for e in &arr {
        println!("isi array : {}", e);
    }

    println!("array {:?}",arr);
}

fn _enum_lat() {
    #[derive(Debug)]
    enum JenisKelamin {
        Pria,
        Wanita
    };

    #[derive(Debug)]
    struct Orang {
        nama: String,
        jenis_kelamin : JenisKelamin
    };

    let p1 = Orang {
        nama :String::from("John"),
        jenis_kelamin : JenisKelamin::Pria
    };

    let p2 = Orang {
        nama :String::from("amy"),
        jenis_kelamin : JenisKelamin::Wanita
    };
    println!("Jenis Kelamin : {:?}",p1);
    println!("Jenis Kelamin : {:?}",p2);
}
fn _io() {
    let mut line = String::new();
    println!("Enter your name :");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line);
    println!("no of bytes read , {}", b1);
}