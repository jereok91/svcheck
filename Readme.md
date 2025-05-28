# 🛰️ Server Version Checker

**Server Version Checker** es una herramienta de línea de comandos escrita en Rust que realiza solicitudes HTTP `GET` a uno o varios dominios/IPs y muestra la versión del servidor web (cabecera `Server`). Es ideal para análisis rápidos, auditorías de seguridad o inventarios de infraestructura.

![image](https://github.com/user-attachments/assets/2502c375-bd51-45c7-8b50-d35a2b290a88)
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

### Instalción en Macos y Linux
- bash
```bash
curl -fsSL https://raw.githubusercontent.com/jereok91/svcheck/refs/heads/main/install/MacosLinux/install.sh | bash
```

- zsh
```bash
curl -fsSL https://raw.githubusercontent.com/jereok91/svcheck/refs/heads/main/install/MacosLinux/install.sh | zsh
```

⚠️  $INSTALL_DIR is not in your PATH.
Add this line to your shell config (e.g., ~/.bashrc, ~/.zshrc)

## 🧪 Uso

Consultar uno o varios dominios y/o ips

```bash
svcheck -- google.com github.com
```
Mostrar todos los encabezados
```bash
svcheck -- example.com --all
```
Guardar resultados en un archivo
```bash
svcheck -- openai.com --save resultados.txt
```
Ver ayuda
```bash
svcheck -- --help
```

## 📄 Licencia
Este proyecto está bajo la licencia GPL-3.0 license Consulta el archivo LICENSE para más información.

## 🤝 Contribuciones
¡Las contribuciones son bienvenidas! Puedes enviar issues, sugerencias o pull requests.

