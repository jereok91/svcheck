# 🛰️ Server Version Checker

**Server Version Checker** es una herramienta de línea de comandos escrita en Rust que realiza solicitudes HTTP `GET` a uno o varios dominios/IPs y muestra la versión del servidor web (cabecera `Server`). Es ideal para análisis rápidos, auditorías de seguridad o inventarios de infraestructura.

---

## 🚀 Características

- ✅ Acepta múltiples dominios/IPs como entrada.
- ✅ Extrae y muestra la cabecera `Server`.
- ✅ Modo detallado con `--all` para ver todos los encabezados HTTP.
- ✅ Guarda los resultados en un archivo con `--save archivo.txt`.
- ✅ Ayuda rápida con `--help`.

---

## 📦 Requisitos

- [Rust](https://www.rust-lang.org/tools/install) (estable)

---

## ⚙️ Instalación

```bash
git clone https://github.com/tu-usuario/server-version-checker.git
cd server-version-checker
cargo build --release
```

## 🧪 Uso

Consultar uno o varios dominios y/o ips

```bash
cargo run -- google.com github.com
```
Mostrar todos los encabezados
```bash
cargo run -- example.com --all
```
Guardar resultados en un archivo
```bash
cargo run -- openai.com --save resultados.txt
```
Ver ayuda
```bash
cargo run -- --help
```

## 📄 Licencia
Este proyecto está bajo la licencia GPL-3.0 license Consulta el archivo LICENSE para más información.

## 🤝 Contribuciones
¡Las contribuciones son bienvenidas! Puedes enviar issues, sugerencias o pull requests.

