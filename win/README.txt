=========================================
 Server Version Checker - svcheck (v0.1.0)
=========================================

Descripción:
------------
svcheck es una herramienta de línea de comandos que te permite consultar 
la versión del servidor web (cabecera HTTP `Server`) de uno o varios 
dominios o direcciones IP.

Uso:
----
Abre una terminal (cmd o PowerShell) y ejecuta:

    svcheck dominio.com 192.168.1.10

Opciones:
---------
    --all         Muestra todos los encabezados HTTP de la respuesta.
    --save FILE   Guarda la salida en un archivo de texto.
    --help        Muestra la ayuda del comando.

Ejemplos:
---------
    svcheck google.com github.com
    svcheck openai.com --all
    svcheck example.org --save resultado.txt

Requisitos:
-----------
Conexión a internet para consultar los dominios/IPs.
Compatible con Windows 10/11.

Notas:
------
Después de la instalación, asegúrate de cerrar y volver a abrir la terminal
para que el comando `svcheck` funcione correctamente desde cualquier ubicación.



=========================================
 Server Version Checker - svcheck (v0.1.0)
=========================================

Description:
------------
svcheck is a command-line tool that allows you to query the web server version 
(HTTP `Server` header) from one or more domains or IP addresses.

Usage:
------
Open a terminal (cmd or PowerShell) and run:

    svcheck domain.com 192.168.1.10

Options:
--------
    --all         Show all HTTP response headers.
    --save FILE   Save the output to a text file.
    --help        Display help and usage information.

Examples:
---------
    svcheck google.com github.com
    svcheck openai.com --all
    svcheck example.org --save result.txt

Requirements:
-------------
Internet connection to query domains or IPs.
Compatible with Windows 10/11.

Notes:
------
After installation, make sure to close and reopen your terminal so that 
the `svcheck` command works from any location.

