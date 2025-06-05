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

### Instaldor de Windows 
[![Descargar para Windows](https://img.shields.io/badge/Windows-Descargar_Instalador-blue?style=for-the-badge&logo=windows)](https://github.com/jereok91/svcheck/raw/refs/heads/main/win/svcheck-installer.exe)

[:package: Instalador Windows](https://github.com/jereok91/svcheck/raw/refs/heads/main/win/svcheck-installer.exe)

### svcheck (Windows) 🚀

### 📍 PASO 1: UBICA EL ARCHIVO config.toml


Después de instalar o descomprimir svcheck, asegúrate de que el archivo:
    config.toml

esté en el mismo directorio donde se encuentra:
    svcheck.exe

Ejemplo:
    
    C:\Program Files\svcheck\
        ├── svcheck.exe
        └── config.toml


#### 🛠 PASO 2: EDITA TUS CONFIGURACIONES

Abre el archivo `config.toml` con tu editor de texto favorito 
(Bloc de Notas o Visual Studio Code, por ejemplo).

El archivo contiene parámetros como:

    #[ia]
    #api_key = "TU_API_KEY"
    #api_url = "URL_API"
    #pront_ia = "Asistente"

- ✅ Descomentar y editar los valores según tus necesidades
- ✅ Reemplaza `"TU_API_KEY"` con tu clave real.
- ✅ Reemplaza `"URL_API"` con la URL de tu API.
- ✅ Reemplaza `"Asistente"` con el nombre de tu asistente.


#### ✅ PASO 3: EJECUTA svcheck


Desde una terminal (CMD o PowerShell), puedes ejecutar el binario así:

    svcheck midominio.com 127.0.0.1

El programa cargará automáticamente el archivo de configuración si
está ubicado en el mismo directorio que `svcheck.exe`.


#### 📢 NOTA IMPORTANTE
- El archivo de configuración solo es cargado automáticamente en Windows.
- Asegúrate de no mover el archivo `config.toml` a otro lugar.
- Para entornos de red restringidos, asegúrate de que svcheck tenga permisos de acceso a Internet.


# Configuración de OpenRouter para integración con IA

## Pasos para obtener tus credenciales

### 1. Crear una cuenta en OpenRouter
🔹 Visita [https://openrouter.ai](https://openrouter.ai)  
🔹 Regístrate con tu email o cuenta de GitHub  

### 2. Generar tu API Key
1. Ve a [https://openrouter.ai/keys](https://openrouter.ai/keys)
2. Haz clic en "Create a key"
3. Copia tu API Key (guárdala en un lugar seguro)
![image](https://github.com/user-attachments/assets/5b2e393f-4644-4b52-bf13-a1e9ea5ad0b3)
![image](https://github.com/user-attachments/assets/9b141efc-1d92-438f-a994-fd91bba55faa)


### 3. Configurar tu aplicación
Puedes usar la API directamente o mediante archivo de configuracion:

#### configurando el archivo .../svcheck/config.toml
```toml
[ia]
api_key = "KEY_GENERATE"
api_url = "URL_MODELO"
pront_ia = "Asistente de hardening web. Analiza:  1. Headers (Server+otros si existen)  2. Identifica:     - Versiones vulnerables (CVEs conocidos)     - Configs inseguras (headers sensibles)  3. Recomienda acciones prioritarias  Formato salida:  ### 🔍 [URL]  **🛡️ Headers**: [destacar riesgos]  **🔴 Riesgos**: [lista priorizada]  **✅ Recomendaciones**: [acciones concretas]  Ejemplos:  🔗 https://a.com | Status:200 | Server:Apache/2.4.62  🔗 https://b.com | Headers: Server:nginx/1.18.0\nX-Powered-By:PHP/7.2  Input actual:"

```

## 🧪 Uso

Consultar uno o varios dominios y/o ips

```bash
svcheck google.com github.com
```
Mostrar todos los encabezados
```bash
svcheck example.com --all
```
Guardar resultados en un archivo
```bash
svcheck openai.com --save resultados.txt
```
Ver ayuda
```bash
svcheck --help
```

## 📄 Licencia
Este proyecto está bajo la licencia GPL-3.0 license Consulta el archivo LICENSE para más información.

## 🤝 Contribuciones
¡Las contribuciones son bienvenidas! Puedes enviar issues, sugerencias o pull requests.

