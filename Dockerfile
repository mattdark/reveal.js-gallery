FROM rustlang/rust:nightly

WORKDIR /home
RUN git clone https://github.com/mattdark/reveal.js-gallery.git && cd reveal.js-gallery && sh installer.sh && cargo build && cp target/debug/reveal-gallery ../.

EXPOSE 8000

CMD ["/home/reveal-gallery"]
