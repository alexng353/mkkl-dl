# rust-mangakakalot

Author: [@alexng353](https://github.com/alexng353)

A very simple regex + reqwests parser and tricker for mangakakalot-affiliated sites, written in rust as my first "real" project.

## Todo

- find out how many bugs there are and fix em all

## Done

- add support for chapmanganato.com

### Usage

```bash
# unix
./rust-mangakakalot <URL>

# windows
rust-mangakakalot.exe <URL>
```

### disclaimer

Only been tested once on each site (mangakakalot, chapmanganato)

- mangakakalot
  - Devilchi (118 chapters)
  - Solo Leveling (180 chapters)
- chapmanganato
  - Attack on Titan (53 chapters)

I MADE IT SLOW ON PURPOSE.

The default delay is 500ms between images and 3s between chapters so you don't get instantly banned from mangakakalot.

If you put a .env in the same folder as the executable, it *should* automatically read and parse your settings:

below are the default settings
```bash
# .env
IMG_DELAY=500 # value in millis
OUTPUT_DIR="./output" # it can also take an absolute path
CHAPTER_DELAY=3000 # value also in millis
```

# License

```
MIT License

Copyright (c) 2022 alexng353

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```