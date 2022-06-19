FROM rust:latest as build

# Creates empty project, to cache dependancies
# Highlights upstream dependancy errors
RUN USER=root cargo new --bin notify
WORKDIR /notify
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# Build notify
COPY ./src ./src
RUN rm ./target/release/deps/notify*
RUN cargo build --release

# Create image
FROM rust:latest

# Copy binary and sample data/config
COPY --from=build /notify/target/release/notify .
COPY ./data ./data 
COPY ./config ./config

# Set entry to run notify
CMD ["./notify"]
