FROM python:3.8-slim-buster

WORKDIR /app

# Copy in files
COPY notifications notifications
COPY lib lib
COPY config config
COPY main.py main.py
COPY requirements requirements

RUN pip3 install -r requirements

CMD [ "python", "main.py"]
