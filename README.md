# StarQuad

Another WIP renderer for the Gaia dataset.

Prior art: my [star-stream](https://github.com/lancelet/star-stream) project in Haskell.

Some rough goals for this project:

  - Use Rust not Haskell this time (it's a learning exercise - I prefer Haskell in general but want more experience with Rust).

  - Build an equal area quadtree to index star positions, using the HEALPix top-level regions as initial squares. This has to be able to page to and from disk using a memory-resident cache of some kind.

  - Allow rendering of arbitrary sky regions (`star-stream` did this very poorly) by using queries on the HEALPix quad-tree.

  - Render using arbitrary point-spread functions and better sampling approaches than `star-stream`.

  - Render to high dynamic range images (eg. OpenEXR, Radiance HDR).

  - Create giant prints of the night sky on an Epson P906 printer. I have ordered one of these printers!
