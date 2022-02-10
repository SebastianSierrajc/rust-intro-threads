use std::io;
use std::thread;
use std::time::Duration;

fn main() {

    println!("Contadores con hilos!");

    loop {
        let mut handles = vec![]; // referencias de los hilos
        let mut num_t = String::new();
        let mut counter = String::new();

        println!("\nInserte el numero de hilos: ");
        io::stdin().read_line(&mut num_t).expect("Failed to read line");

        let num_t: u32 = match num_t.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nInserte el tamano del contador: ");
        io::stdin().read_line(&mut counter).expect("Failed to read line");

        let counter: u32 = match counter.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // creacion de los hilos
        for i in 1..num_t+1 {
            let handle = thread::Builder::new()
                .name(format!("T{}", i)) //identificador del hilo
                .spawn(move || {
                    for c in 1..counter+1 {
                        // mostrar el conteo en consola bonito :)
                        let curr = thread::current();
                        println!("Hilo {:#?}. Contador: {}", curr.name().unwrap(), c);

                        // pausar el hilo por 500ms
                        thread::sleep(Duration::from_millis(500));
                    }
                })
                .unwrap();
            handles.push(handle);
        }


        // esperar a que todos los hilos terminen
        for handle in handles {
            handle.join().unwrap();
        }

        println!("\nSalir (Y/n)?");

        let mut quit = String::new();
        io::stdin().read_line(&mut quit).expect("Falied to read line");

        if quit.trim() == "Y" || quit == "\n" {
            break;
        }
    }
}
