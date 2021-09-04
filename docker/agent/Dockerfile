FROM ubuntu:20.04
COPY ./app /app
# RUN apt update && apt install -y redis-server
EXPOSE 10086
#CMD redis-server & cd /app && ./agent_server
CMD cd /app && ./agent_server
ENV sas_client_id="IUkCQnAo8Un4pTVfWNf1a0LlKbD7neBdVwmMeqLy"
ENV sas_client_secret="Hp8IH6BL7iRkKdbwTvVs17A7pUIkIMhc0TU9sq40cHoxpkPPFqJwe865HG1IZhtXDRekIdWuOp3UmwPBKWq6L0TBYgXQmTFhW5UG7FPfp23Otff4gtsBCAmXzmtYRwh7"
ENV RUST_BACKTRACE=1
