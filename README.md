# rust-hello
Rust harjoittelua Tietix pajajaksolla.

## coffee.rs
Tekstipohjainen kahvilapeli jossa voi vastata vain KYLLÄ tai EI.

## rasterize.rs
rasteroi Stanford .PLY muotoisia 3D-malleja 256x256 pixelin spriteiksi, piirtämällä
niiden paksuiksi paisutetut verteksit .PNG-kuvaksi. Tukee verteksivärejä.

![Rasteroidut verteksit](https://raw.githubusercontent.com/nikoiivari/rust-hello/main/raster.png)

Komentoriviltä luetaan projektikansio, kuvakulma yläviistosta (45 astetta) ja kuinka monesta kuvakulmasta objekti rasteroidaan (360-astetta jaetaan 8 osaan). Lisäksi offset ensimmäisen kuvan kulmalle on 0 astetta. Lopuksi animaatio framen nimi.

`rasterize VertexPainCube 45 8 0 VertexPainCube`

Komentoriviltä pitäisi myös voida asettaa verteksien paksuus.
Tällä hetkellä psize=0.025, eikä sitä voi asettaa komentoriviltä.

## test.rs
SDL2-rajapinnalla tehty 2D pelinteko kokeilu.
