# Introducción

Esta aplicación es solo una prueba de concepto, con el fin de evaluar la confiabilidad de los paquetes disponibles en Rust
para el uso del broker de mensajería Kafka.

En este branch adicional de conectarse a kafka, almacenara los datos en una base de datos cockroach y se configurara el docker compose para enviar los logs a
grafana Loki.

Luego de múltiples pruebas se termino eligiendo el paquete kafka sobre rdkafka, debiodo a que rdkafka generaba entrega exitosa
incluso con el host inaccesible, sin embargo, debo reconocer que la velocidad de rdkafka era mejor.

Por si acaso, en el archivo main.rs queda el código de rdkafka.

# Logs con Grafana Loki

En este caso usaremos el paquete de rust log4rs para guardar el log en un archvo y a promtail para enviar estos archivosa Grafana Loki.

Para log4rs se configura desde el archivo main dentro del método setup_log.

Para promtail lo debemos configurar como un nuevo servicio y el archivo de configuración esta en .devops/docker/develop/promtail_config.yml.
La carpeta en la que esta el log de la aplicación se monta como volumen en promtail para que los archivos se comuniquen de forma correcta.

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

