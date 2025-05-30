use dotenvy::dotenv;
use reqwest::blocking::Client;
use reqwest::header::SERVER;
use std::{env, fs::File, io::Write};
use utilities::{
    ia_integration::{get_recomender_ia, parse_response},
    validator::{es_direccion_ip_con_puerto, es_url_valida},
};
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod utilities;

fn mostrar_ayuda(nombre_programa: &str) {
    println!(
        "Uso: {} <dominio1> <dominio2> ... [--all] [--save archivo.txt] [--ia]\n
        version: {}\n
Opciones:
  --version -v      Muestra la versión del programa.
  --all              Muestra todos los encabezados HTTP, no solo 'Server'.
  --save archivo     Guarda la salida en un archivo de texto en txt.
  --ia               Consulta a la IA para recomendaciones.
  --help             Muestra este mensaje de ayuda.",
        nombre_programa, VERSION
    );
}

fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--help") || args.len() < 2 {
        mostrar_ayuda(&args[0]);
        return;
    }

    if args.iter().any(|arg| arg == "--version") || args.iter().any(|arg| arg == "-v") {
        println!("svcheck {}", VERSION);
        return;
    }

    // flags
    let show_all = args.iter().any(|arg| arg == "--all");
    let targets: Vec<String> = args
        .iter()
        .skip(1)
        .filter(|arg| {
            *arg != "--all"
                && *arg != "--save"
                && *arg != "--ia"
                && !arg.ends_with(".txt")
                && (es_url_valida(arg) || es_direccion_ip_con_puerto(arg))
        })
        .cloned()
        .collect();
    if targets.is_empty() {
        eprintln!("❌ Debes especificar al menos un dominio o IP validos.");
        std::process::exit(1);
    }

    let save_index = args.iter().position(|a| a == "--save");
    let save_file = save_index
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string());
    let consultar_ia = args.iter().any(|arg| arg == "--ia");

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
    // Consultar IA
    if consultar_ia {
        println!("Consulta a la IA. Esto puede tardar un momento...");
        let mut is_no_valibales = false;
        // instructivo
        let api_key_ia = env::var("API_KEY").unwrap_or_else(|_| {
            eprintln!("❌ No se encontró la variable de entorno OPENROUTER_API_KEY puedes generarla en https://openrouter.ai");
            is_no_valibales= true;
            String::new()
        });
        let api_url = env::var("API_URL").unwrap_or_else(|_| {
            eprintln!("❌ No se encontró la variable de entorno API_URL puede encontrarlo en https://openrouter.ai/");
            is_no_valibales= true;
            String::new()
        });

        let pront_ia = env::var("PRONT_IA").unwrap_or_else(|_| {
            eprintln!("❌ No se encontró la variable de entorno PRONT_IA aqui relacionas la intruducion de los datos que pasaras a la ia");
            is_no_valibales= true;
            String::new()
        });

        if !is_no_valibales {
            match get_recomender_ia(&all_output, &api_key_ia, &pront_ia, &api_url) {
                Ok(response) => match parse_response(&response) {
                    Ok(parsed_response) => {
                        println!(
                            "Respuesta de la IA: {}",
                            parsed_response.choices[0].message.content.to_string()
                        );
                        all_output.push_str(&format!(
                            "\nRespuesta de la IA:\n{}\n",
                            parsed_response.choices[0].message.content.to_string()
                        ));
                    }
                    Err(e) => {
                        eprintln!("❌ Error al parsear la respuesta de la IA: {}", e);
                        all_output.push_str(&format!(
                            "\nError al parsear la respuesta de la IA: {}\n",
                            e
                        ));
                    }
                },
                Err(e) => {
                    eprintln!("❌ Error al consultar la IA: {}", e);
                }
            };
        } else {
            print!("❌ Error al crear consultar para la IA:");
            all_output.push_str(&format!("\n Respuesta de la IA: configuracion incompleta"));
        }
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
