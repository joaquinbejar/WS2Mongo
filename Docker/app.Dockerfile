# Etapa de compilación
FROM rust:1.78.0-alpine3.19 as builder

# Instala las dependencias necesarias para compilar openssl y otras dependencias nativas
RUN apk update \
    && apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig build-base

# Crea un nuevo proyecto vacío para construir las dependencias
WORKDIR /usr/src/app
RUN USER=root cargo new --bin ws2mongo
WORKDIR /usr/src/app/ws2mongo

# Copia tus archivos de dependencias y compila solo las dependencias
# Esto previene la recompilación de dependencias si solo cambian los archivos fuente
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Ahora copia tu código fuente y compila de nuevo, esta vez será rápido solo recompilando tu código
COPY src ./src
RUN rm ./target/release/deps/ws2mongo*
RUN cargo build --release

# Etapa de ejecución
FROM alpine:3.19
ARG APP=/usr/src/app

# Copia el binario compilado desde la etapa de compilación
COPY --from=builder /usr/src/app/ws2mongo/target/release/ws2mongo $APP/ws2mongo

# Configura el directorio de trabajo y el binario como ejecutable
WORKDIR $APP
RUN chmod +x $APP/ws2mongo

# Configura el contenedor para correr como un ejecutable
ENTRYPOINT ["./ws2mongo"]
