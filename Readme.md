# Introducción

Esta aplicación es solo una prueba de concepto, con el fin de evaluar la confiabilidad de los paquetes disponibles en Rust 
para el uso del broker de mensajería Kafka.

Luego de múltiples pruebas se termino eligiendo el paquete kafka sobre rdkafka, debiodo a que rdkafka generaba entrega exitosa 
incluso con el host inaccesible, sin embargo, debo reconocer que la velocidad de rdkafka era mejor.

Por si acaso, en el archivo main.rs queda el código de rdkafka. 

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

