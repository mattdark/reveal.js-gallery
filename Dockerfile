FROM rustlang/rust:nightly

ADD . /home
RUN git clone https://github.com/mattdark/reveal.js-gallery.git /home/reveal.js-gallery
RUN cd /home/reveal.js-gallery
RUN cargo build

EXPOSE 8000

CMD ["./reveal-gallery"]
