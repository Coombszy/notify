FROM python:3.8-slim-buster

WORKDIR /app

# Copy in files
COPY notifications notifications
COPY lib lib
COPY config config
COPY main.py main.py

CMD [ "python", "main.py"]
