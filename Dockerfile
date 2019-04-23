FROM rustlang/rust:nightly

WORKDIR /home
RUN git clone https://github.com/mattdark/reveal.js-gallery.git && cd reveal.js-gallery && sh installer && cargo build

EXPOSE 8000

CMD ["/home/reveal.js-gallery/reveal-gallery"]
