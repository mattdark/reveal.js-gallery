# UIKit repository
uikit_repo=uikit/uikit
uikit_file=uikit.zip

# reveal.js repository
revealjs_repo=hakimel/reveal.js

# Directory for CSS & JS files
dir=static

get_uikit() {
    # Solution found on https://gist.github.com/steinwaywhw/a4cd19cda655b8249d908261a62687f8
    curl -s https://api.github.com/repos/$1/releases/latest \
    | grep "browser_download_url.*zip" \
    | cut -d '"' -f 4 \
    | wget -qi - -O $uikit_file
    unzip -d $dir/uikit $uikit_file
    rm $uikit_file
}

get_revealjs() {
    git clone https://github.com/$revealjs_repo.git $dir/reveal.js
}

get_revealjs $revealjs_repo
get_uikit $uikit_repo
