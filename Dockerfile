FROM rustlang/rust:nightly

RUN git clone https://github.com/mattdark/reveal.js-gallery.git
RUN cd reveal.js-gallery
RUN cargo build

EXPOSE 8000

CMD ["./reveal-gallery"]
