FROM rustlang/rust:nightly

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR home
RUN git clone https://github.com/mattdark/reveal.js-gallery.git
ADD /reveal.js-gallery
WORKDIR /reveal.js-gallery
RUN sh installer.sh
RUN cargo build

EXPOSE 8000

CMD ["cargo", "run"]
