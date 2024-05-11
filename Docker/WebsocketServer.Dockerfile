# Usa la imagen base oficial de Python
FROM python:3.9-slim

# Establece el directorio de trabajo en /app
WORKDIR /app

# Instala la biblioteca websockets
RUN pip install websockets

# Copia el archivo del servidor websocket en el contenedor
COPY websocket_server.py /app

# Expone el puerto que el servidor websocket usará
EXPOSE 5678

# Comando para iniciar el servidor websocket cuando el contenedor se ejecute
CMD ["python", "websocket_server.py"]
