# termpix
Draw images in an ANSI terminal! Requires that your terminal can show ANSI colours, and uses a font that can show the 'bottom half block' character (▄).

Usage:
`termpix <file> [--width <width>] [--height <height>] [--max-width <max-width>] [--max-height <max-height>] [--true-color|--true-colour]`

![buzz](https://cloud.githubusercontent.com/assets/4640028/13073384/9d46b2e2-d4f2-11e5-9218-09f1a05bf296.png)

`file` can be any image file readable by the [rust 'image' library](https://github.com/PistonDevelopers/image). 
I've seen it fail with some unusual jpeg files ("Marker SOF2 is not supported.")

By default, it will fill as much of the terminal as possible, while keping the aspect ratio of the input image. 
Use --width or --height to override this, specifying the number of terminal rows or columns to fit to (Or both
to specify an absolute size). Alternatively, use --max-width and/or --max-height to fit to the terminal up to a maximum.

![buzz smaller](https://cloud.githubusercontent.com/assets/4640028/13073404/b60d1410-d4f2-11e5-85c1-ccb6dc967eae.png)

If your terminal supports it, you can use the full 24-bit colour palette with the `--true-colour` flag:
![lena looks good](https://cloud.githubusercontent.com/assets/4640028/13419797/fa51cb88-dfd4-11e5-87c3-f8620cd67557.png)

In low-colour mode, high-contrast, colourful images tend to work better than flatter images. Skin tones and shades of brown are 
particularly poorly represented in the ANSI colour pallette.
![lena looks bad](https://cloud.githubusercontent.com/assets/4640028/13073360/705a85b0-d4f2-11e5-917a-fdb91e5e45b9.png)

# Installing

* Install Rust & Cargo: https://www.rust-lang.org/downloads.html
* `cargo install --git https://github.com/fimkap/termpix` (see `cargo install` options for e.g. install location customisation)
Use https://github.com/hopey-dishwasher/termpix for the original project.

# License
Apache 2.0 license


