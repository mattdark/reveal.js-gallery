# reveal.js Gallery

[reveal.js](https://revealjs.com/) is a framework for creating presentations using HTML. It's an alternative to Microsoft Powerpoint or LibreOffice Impress but using a text editor and any browser. reveal.js Gallery exists as a way to better organize and access all of your presentations from one place. Created using [Rust](https://rust-lang.org), [Rocket](https://rocket.rs/) & [UIKit](https://getuikit.com/).

![reveal.js Gallery index](https://github.com/mattdark/reveal.js-gallery/raw/screenshots/screenshot/reveal.js-gallery.png "reveal.js Gallery")

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

1. Install Rust, on GNU/Linux or Mac:

```
curl https://sh.rustup.rs -sSf | sh
```

for any other operating system go to [rustup.rs](https://rustup.rs/)

This project was build using Rocket which is a web framework written in Rust and it uses the nightly version of the language.

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

## Built With

* [Rust](http://rust-lang.org/) - The programming language
* [Rocket](https://rocket.rs/) - The web framework
* [reveal.js](https://revealjs.com/) - The presentation framework
* [UIKit](https://rometools.github.io/rome/) - The frontend framework

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Authors

* **Mario García** - [mariog.xyz](https://mariog.xyz/)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

