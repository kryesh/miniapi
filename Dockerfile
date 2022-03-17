#Builder container
FROM docker.io/rust AS builder
ARG OUTFILE=miniapi

#CPU to optimise for
ARG TARGET_CPU=generic

#Comment out if you want top use stable channel
#ARG CHANNEL=nightly

WORKDIR /build

COPY . .
RUN ./dockerscript

#Scratch container to be published
FROM scratch
ENV MIMALLOC_LARGE_OS_PAGES=1
EXPOSE 8080

COPY --from=builder /build/miniapi /miniapi

CMD ["/miniapi"]
