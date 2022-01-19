FROM python:3.8-slim-buster

WORKDIR /app

# Copy in files
COPY notifications /app/notifications
COPY lib /app/lib
COPY config /app/config
COPY main.py /app/main.py

CMD [ "python", "main.py"]