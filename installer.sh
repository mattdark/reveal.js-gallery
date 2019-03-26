# reveal.js repository
revealjs_repo=hakimel/reveal.js

# Directory for CSS & JS files
dir=static

get_revealjs() {
    git clone https://github.com/$revealjs_repo.git $dir/reveal.js
}

get_revealjs $revealjs_repo
