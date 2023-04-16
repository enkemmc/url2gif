# URL2gif
URL2gif is a command-line tool for converting URLs to animated GIFs.

## Installation
To install URL2gif, you can follow these steps:

* Download the latest release from the URL2gif repository.
* Copy the binary to a folder on your path
    * ``` sudo mv url2gif /usr/bin/url2gif && chmod +x /usr/bin/url2gif ```

# Requirements
You must have [chromedriver](https://chromedriver.chromium.org/downloads) installed and running on port 9515.  Make sure you install a version that is compatible with your version of Chrome.

# Usage
To use URL2gif, you can run the url2gif command from your terminal, followed by a URL and some optional arguments.

```
url2gif https://www.earthcam.com/cams/newyork/timessquare/ --headless --frames=60 --output=ny_gif.gif
```

The above will create a 60 frame gif.
