use std::thread;
use std::sync::{Arc, Mutex};

// Função para calcular o comprimento de uma string
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    // Exemplo de Ownership e Borrowing
    let s1 = String::from("hello");
    let s2 = s1; // s1 é movido para s2, s1 não é mais válido

    println!("{}", s2); // Isso funciona, s2 é o novo dono

    let s3 = String::from("world");
    let len = calculate_length(&s3); // Emprestamos s3 como uma referência imutável

    println!("Comprimento de '{}' é {}.", s3, len); // s3 ainda é válido aqui

    // Exemplo de Concurrency
    let counter = Arc::new(Mutex::new(0)); // Shared state entre threads
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Resultado final: {}", *counter.lock().unwrap());
}