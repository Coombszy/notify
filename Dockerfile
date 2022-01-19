FROM python:3.8-slim-buster

WORKDIR /app

# Copy in files
COPY notifications /app/notifications
COPY lib /app/lib
COPY main.py /app/main.py
RUN pip3 install -r requirements.txt

COPY . .

CMD [ "python3", "/app/main.py"]