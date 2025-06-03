use std::env;

use crate::utilities::{
    config::Config,
    ia_integration::{get_recomender_ia, parse_response},
};

pub fn cunsulta_ia_run(all_ouput_heades: &String) -> String {
    let mut all_output = all_ouput_heades.clone();
    // Separador visual
    let separador = "🤖".repeat(24);
    let separator_line = format!("🛰️🛰️🛰️{}🛰️🛰️🛰️", separador);
    println!("{}", separator_line);
    all_output.push_str(&separator_line);
    println!("Consulta a la IA. Esto puede tardar un momento...");

    // Cargar configuración
    let config = Config::load()
        .map_err(|e| {
            eprintln!(
                "🚧🚧🚧 Error cargando configuración: {} \n
                📄 puedes configurar  en el config.toml \n",
                e
            );

            e
        })
        .ok();

    // Obtener credenciales de IA
    let (api_key_ia, api_url, pront_ia) = match config {
        Some(cfg) => (cfg.ia.api_key, cfg.ia.api_url, cfg.ia.pront_ia),
        None => (
            get_env_var_or_warn("API_KEY", "OPENROUTER_API_KEY", "https://openrouter.ai/deepseek/deepseek-r1:free/api"),
            get_env_var_or_warn("API_URL", "API_URL", "https://openrouter.ai/deepseek/deepseek-r1:free/api"),
            get_env_var_or_warn("PRONT_IA", "PRONT_IA", "Introducción de datos para la IA,pede usar el ejemplo que se deja en el archivo de configuración "),
        ),
    };

    // Verificar si tenemos todas las credenciales necesarias
    if api_key_ia.is_empty() || api_url.is_empty() || pront_ia.is_empty() {
        println!("❌ Error: Configuración incompleta para consultar la IA");
        all_output.push_str("\nRespuesta de la IA: configuración incompleta")
    }

    // Consultar a la IA
    match get_recomender_ia(&all_output, &api_key_ia, &pront_ia, &api_url) {
        Ok(response) => match parse_response(&response) {
            Ok(parsed_response) => {
                let ia_response = parsed_response.choices[0].message.content.to_string();
                println!("Respuesta de la IA: {}", ia_response);
                all_output.push_str(&format!("\nRespuesta de la IA:\n{}\n", ia_response));
            }
            Err(e) => handle_ia_error("parsear", e, &mut all_output),
        },
        Err(e) => handle_ia_error("consultar", e, &mut all_output),
    };

    all_output
}

fn get_env_var_or_warn(var_name: &str, description: &str, help_url: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| {
        eprintln!(
            "❌ No se encontró la variable de entorno {} ({}) - más info: {}",
            var_name, description, help_url
        );
        String::new()
    })
}

fn handle_ia_error(operation: &str, error: impl std::fmt::Display, all_output: &mut String) {
    eprintln!("❌ Error al {} la IA: {}", operation, error);
    all_output.push_str(&format!("\nError al {} la IA: {}\n", operation, error));
}
