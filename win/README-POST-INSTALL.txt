===========================================
        INSTALACIÓN Y CONFIGURACIÓN
             DE svcheck (Windows)
===========================================

Gracias por instalar svcheck 🚀

Este documento te guiará para dejar configurado correctamente
el archivo de configuración necesario para el funcionamiento
de la herramienta.

-------------------------------------------
📍 PASO 1: UBICA EL ARCHIVO config.toml
-------------------------------------------

Después de instalar o descomprimir svcheck, asegúrate de que el archivo:
    config.toml

esté en el mismo directorio donde se encuentra:
    svcheck.exe

Ejemplo:
    C:\Program Files\svcheck\
        ├── svcheck.exe
        └── config.toml

-------------------------------------------
🛠 PASO 2: EDITA TUS CONFIGURACIONES
-------------------------------------------

Abre el archivo `config.toml` con tu editor de texto favorito 
(Bloc de Notas o Visual Studio Code, por ejemplo).

El archivo contiene parámetros como:

    #[ia]
    #api_key = "TU_API_KEY"
    #api_url = "URL_API"
    #pront_ia = "Asistente"

✅ Descomentar y editar los valores según tus necesidades
✅ Reemplaza `"TU_API_KEY"` con tu clave real.
✅ Reemplaza `"URL_API"` con la URL de tu API.
✅ Reemplaza `"Asistente"` con el nombre de tu asistente.

-------------------------------------------
✅ PASO 3: EJECUTA svcheck
-------------------------------------------

Desde una terminal (CMD o PowerShell), puedes ejecutar el binario así:

    svcheck midominio.com 127.0.0.1

El programa cargará automáticamente el archivo de configuración si
está ubicado en el mismo directorio que `svcheck.exe`.

-------------------------------------------
📢 NOTA IMPORTANTE
-------------------------------------------

- El archivo de configuración solo es cargado automáticamente en Windows.
- Asegúrate de no mover el archivo `config.toml` a otro lugar.
- Para entornos de red restringidos, asegúrate de que svcheck tenga permisos de acceso a Internet.

¡Gracias por usar svcheck!

