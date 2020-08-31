FROM rust:1.45-stretch as builder

ADD . ./

RUN apt-get update && \
    apt-get install -y libssl-dev && \
    cargo build --verbose --release && \
    cargo install

FROM debian:stretch
COPY --from=builder /usr/local/cargo/bin/rdf_worker /usr/bin

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates

ENV AMQP_QUEUE job_rdf
ENV AMQP_COMPLETED_QUEUE job_rdf_completed
ENV AMQP_ERROR_QUEUE job_rdf_error
CMD rdf_worker
