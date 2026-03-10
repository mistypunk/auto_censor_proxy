FROM python:3.9-slim

# Atualização dos nomes dos pacotes para as distros Debian mais recentes
RUN apt-get update && apt-get install -y \
    libgl1 \
    libglib2.0-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Upgrade no pip ajuda a evitar erros de compilação de pacotes de IA
RUN pip install --no-cache-dir --upgrade pip

RUN pip install --no-cache-dir \
    nudenet \
    fastapi \
    uvicorn \
    python-multipart

COPY app.py .

EXPOSE 8080

CMD ["python", "app.py"]
