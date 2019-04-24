FROM rustlang/rust:nightly

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /home
RUN git clone https://github.com/mattdark/reveal.js-gallery.git && cd reveal.js-gallery && sh installer.sh && cargo build && cp target/debug/reveal-gallery ../.

EXPOSE 8000

CMD ["/home/reveal-gallery"]
