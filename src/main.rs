use reqwest::blocking::Client;
use reqwest::header::SERVER;
use std::{env, fs::File, io::Write};

fn mostrar_ayuda(nombre_programa: &str) {
    println!(
        "Uso: {} <dominio1> <dominio2> ... [--all] [--save archivo.txt]\n
Opciones:
  --all              Muestra todos los encabezados HTTP, no solo 'Server'.
  --save archivo     Guarda la salida en un archivo de texto.
  --help             Muestra este mensaje de ayuda.",
        nombre_programa
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--help") || args.len() < 2 {
        mostrar_ayuda(&args[0]);
        return;
    }

    // flags
    let show_all = args.iter().any(|arg| arg == "--all");
    let targets: Vec<String> = args
        .iter()
        .skip(1)
        .filter(|arg| *arg != "--all" && *arg != "--save" && !arg.ends_with(".txt"))
        .cloned()
        .collect();
    if targets.is_empty() {
        eprintln!("❌ Debes especificar al menos un dominio o IP.");
        std::process::exit(1);
    }

    let save_index = args.iter().position(|a| a == "--save");
    let save_file = save_index
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string());

    let mut all_output = String::new();

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .expect("No se pudo crear el cliente HTTP");

    for target in targets {
        let url = if target.starts_with("http://") || target.starts_with("https://") {
            target.clone()
        } else {
            format!("https://{}", target)
        };

        let mut output = format!("🔗 {}:\n", url);

        match client.get(&url).send() {
            Ok(resp) => {
                output.push_str(&format!("Status: {}\n", resp.status()));

                if show_all {
                    output.push_str("\n--- Headers completos ---\n");
                    for (key, value) in resp.headers().iter() {
                        let line = format!("{}: {}\n", key, value.to_str().unwrap_or("[binario]"));
                        output.push_str(&line);
                    }
                } else {
                    match resp.headers().get(SERVER) {
                        Some(server_value) => {
                            output.push_str(&format!(
                                "Servidor: {}\n",
                                server_value.to_str().unwrap_or("desconocido")
                            ));
                        }
                        None => output.push_str("No se encontró la cabecera 'Server'.\n"),
                    }
                }
            }
            Err(e) => {
                output.push_str(&format!("❌ Error en la petición: {}\n", e));
            }
        }

        // Mostrar individualmente
        println!("{}", output);

        // Acumular para guardar
        all_output.push_str(&format!("{}\n{}\n", "=".repeat(60), output));
    }

    // Guardar si se especificó
    if let Some(filename) = save_file {
        match File::create(&filename) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(all_output.as_bytes()) {
                    eprintln!("Error al escribir en el archivo: {}", e);
                } else {
                    println!("✅ Resultados guardados en '{}'", filename);
                }
            }
            Err(e) => {
                eprintln!("❌ No se pudo crear el archivo: {}", e);
            }
        }
    }
}
