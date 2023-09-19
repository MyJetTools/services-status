FROM ubuntu:22.04
COPY ./target/release/services-status ./target/release/services-status 
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/services-status"]