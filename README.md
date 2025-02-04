# Concurrency in Rust

Este é um projeto de exemplo que explora conceitos de concorrência em Rust. O objetivo é demonstrar como utilizar as funcionalidades de concorrência do Rust, como threads, `async`/`await`, e `Arc`/`Mutex` para gerenciar estados compartilhados entre múltiplas threads.

## Tecnologias Utilizadas

- Rust (versão 1.x)
- Biblioteca padrão de concorrência do Rust

## Funcionalidades

O projeto implementa várias técnicas de concorrência, incluindo:

- **Thread-based concurrency**: Utilizando `std::thread` para criar threads e compartilhar dados entre elas.
- **Async concurrency**: Usando `async`/`await` para realizar tarefas assíncronas.
- **Shared state with `Arc` and `Mutex`**: Mostra como compartilhar estados entre threads de forma segura.

## Como Rodar

### Pré-requisitos

Certifique-se de ter o Rust instalado em seu sistema. Caso não tenha, instale-o utilizando [rustup](https://rustup.rs/).

### Instalação

Clone este repositório para o seu ambiente local:

```bash
git clone https://github.com/luqueta-DEV/rust.git
cd rust
