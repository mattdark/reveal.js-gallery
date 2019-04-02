# reveal.js Gallery [![Build Status](https://travis-ci.org/mattdark/reveal.js-gallery.svg?branch=master)](https://travis-ci.org/mattdark/reveal.js-gallery)

[reveal.js](https://revealjs.com/) is a framework for creating presentations using HTML. It's an alternative to Microsoft Powerpoint or LibreOffice Impress but using a text editor and any browser. reveal.js Gallery exists as a way to better organize and access all of your presentations from one place. Created using [Rust](https://rust-lang.org), [Rocket](https://rocket.rs/) & [Material Kit](https://www.creative-tim.com/product/material-kit).

![reveal.js Gallery index](https://github.com/mattdark/reveal.js-gallery/raw/screenshots/screenshot/reveal.js-gallery-0.3.png "reveal.js Gallery")
![reveal.js Gallery slides](https://github.com/mattdark/reveal.js-gallery/raw/screenshots/screenshot/reveal.js-gallery-0.3-slides.png "reveal.js Gallery")

## Features

1. Individual URL. You can access a presentation by writing its URL in the address bar on your favorite browser (e.g. http://localhost:8000/presentation/slide1).

2. Individual theme. Use a different theme for every presentation in your gallery.

3. Speaker notes. Support for speaker notes was added. Press 's' on your keyboard to open speaker notes window.

4. Multiplexing. It's one of the plugins I like the most from reveal.js and it was tested within the gallery app. Instructions will be added soon.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine.

### Prerequisites

1. Install Rust, on GNU/Linux or Mac:

```
curl https://sh.rustup.rs -sSf | sh
```

for any other operating system go to [rustup.rs](https://rustup.rs/)

This project was built using Rocket which is a web framework written in Rust and it uses the nightly version of the language.

* Install Rust nightly:

```
rustup install nightly
```

2. Clone this repository:

```
git clone https://github.com/mattdark/reveal.js-gallery.git
```

3. Copy UIKit and reveal.js to the static directory:

```
sh installer.sh
```

### New Presentation

1. Create a Markdown file with the content of your presentation, assign a name that identifies it and save it in /static/slides. There are two examples in that directory.

* Note: ```---``` is used as horizontal separator and ```----``` as vertical one.

2. Use reveal.js themes or create your own and put it in /static/reveal.js/css/theme.

3. Add the following information in /static/slides.json. There is a slides.json example file:

* **id** - A number assiged to identify each presentation
* **file** - Name of the Markdown file
* **title** - Title of the presentation
* **description** - Description of the presentation
* **style** - Name of the theme file
* **url** - Path for your presentation (i.e. slide1)
* **screenshot** - A screen capture of your presentation

### Running

In debug mode:

```
cargo run
```

In release mode:

```
cargo run --release
```

Now open http://localhost:8000 in your favorite browser (I recommend Firefox).

You can also access your presentation by writing the full path in the address bar (e.g. http://localhost:8000/presentation/slide1).

## Built With

* [Rust](http://rust-lang.org/) - The programming language
* [Rocket](https://rocket.rs/) - The web framework
* [reveal.js](https://revealjs.com/) - The presentation framework
* [Material Kit](https://www.creative-tim.com/product/material-kit/) - The UI kit

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Authors

* **Mario García** - [mariog.xyz](https://mariog.xyz/)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## Acknowledgments
* UI based on [Material Kit](https://www.creative-tim.com/product/material-kit) by [Creative Tim](https://www.creative-tim.com/)

