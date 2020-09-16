use std::io;
use std::io::prelude::*;

fn main() {
    
    loop {
        println!("Digite a operação desejada: ");
    
        println!("0 - Cancelar");
        println!("1 - Comprar");

        ler_entrada();
        let quantidade = fluxo_compra();

        match quantidade {
            Some(qtd) => {
                match qtd.as_str() {
                    "0" => {
                        continue
                    }
                    _ => {
                        println!("Você confirma a compra de {} unidades?", qtd);
                        println!("0 - Cancelar");
                        println!("1 - Aceitar");
                    }
                }
            }
            None => continue
        }

        fluxo_aceite_compra();
    }
}

fn ler_entrada() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {

                match line.as_str() {
                    "1" => {
                        break;
                    },
                    "0" => {
                        std::process::exit(0);
                    }
                    _ => {
                        println!("Opção não encontrada, digite novamente")
                    }
                }

            } Err(_err) => println!("Não foi possível processar seu pedido, favor enrar novamente com a opção desejada")
        }
    }
}

fn fluxo_compra() -> Option<String> {
    let stdin = io::stdin();
    println!("Digite a quantidade de passagens que deseja comprar, cada uma custa 1 unidade: ");

    for line in stdin.lock().lines() {
        let quantidade = match line {
            Ok(quantidade) => Some(quantidade),
            Err(_err) => {
                println!("Não foi possível processar seu pedido, favor enrar novamente com a opção desejada, ou 0 para cancelar");

                None
            }
        };
        
        return quantidade;
    }

    None
}

fn fluxo_aceite_compra() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                match line.as_str() {
                    "1" => {
                        println!("Realizando a impressão do comprovante\n");
                        break;
                    },
                    _ => {
                        println!("Transação cancelada\n");
                        break;
                    }
                }

            } Err(_err) => println!("Não foi possível processar seu pedido, favor enrar novamente com a opção desejada")
        }
    }
}
