# rust-hello
Rust harjoittelua Tietix pajajaksolla.

## coffee.rs
Tekstipohjainen kahvilapeli jossa voi vastata vain KYLLÄ tai EI.

## rasterize.rs
rasteroi Stanford .PLY muotoisia 3D-malleja 256x256 pixelin spriteiksi, piirtämällä
niiden paksuiksi paisutetut verteksit .PNG-kuvaksi. Tukee verteksivärejä.

![Rasteroidut verteksit](https://raw.githubusercontent.com/nikoiivari/rust-hello/main/raster.png)

Komentoriviltä luetaan projektikansio, kuvakulma yläviistosta (45 astetta) ja kuinka monesta kuvakulmasta objekti rasteroidaan (360-astetta jaetaan 8 osaan). Lisäksi offset ensimmäisen kuvan kulmalle on 0 astetta. Seuraava parametri on skaala (scale 1.0) joka käytännössä pienentää objektin kokoa spritellä mitä suurempi arvo annetaan. Tämän jälkeen annetaan pystysuunnan offset (vertical offset -1.0 .. 1.0). Suuntaava valonlähde vasemmalta oikealle (-Z..Z) ja edestä taakse (-X..X). Valon diffuse ja ambient värit hexadesimaalina. Lopuksi animaatio framen nimi.

`rasterize VertexPainCube 45 8 0 1.0 0.0  45.0 12.5 0xffddddff 0x222222ff VertexPainCube`

TODO: Palette mapping. 32bpp kuva mapataan vähä väriseen palettiin, mahdollisesti ditheröinnillä.

Komentoriviltä pitäisi myös voida asettaa verteksien paksuus.
Tällä hetkellä psize=0.025, eikä sitä voi asettaa komentoriviltä.

## test.rs
SDL2-rajapinnalla tehty 2D pelinteko kokeilu.
