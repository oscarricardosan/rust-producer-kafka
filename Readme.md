# Introducción

Esta es una aplicación de prueba que muestra como producir mensajes desde una app Rust a un broker de mensajería 

# Docker

Para levantar la aplicación en entorno de desarrollo ejecutar:

```bash
#Levantar contenedor
docker-compose -f .devops/docker/develop/docker-compose.yml up

#Ingresar a contenedor
docker exec -it kafka-rust-producer bash

#Ejecutar app
docker exec -it kafka-rust-producer cargo run
```

# Compilar

Generar ejecutable:

```bash
docker exec -it kafka-rust-producer cargo build --release
docker exec -it kafka-rust-producer  ./target/  
```


# Despliegue

Generar archivo .deb:

```bash
cargo install cargo-deb --version 1.40.5
cargo deb
```

Configurar entorno

```bash

# Instalar .deb
sudo dpkg -i eclipse_miner_1.0.0_amd64.deb

# Crear carpeta para guardar archivos
sudo mkdir pasarex_files_main

# Cambiar permisos de carpeta
sudo chmod -R 777 pasarex_files_main

# Cambiar de directorio
cd ~/bin/eclipse_miner

# Ejecutar app
eclipse_miner
```

