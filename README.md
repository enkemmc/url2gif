# URL2gif
URL2gif is a command-line tool for converting URLs to animated GIFs.

## Installation
To install My App, you can follow these steps:

* Download the latest release from the URL2gif repository.
* Extract the downloaded archive to a directory of your choice.
* Run the install.sh script included in the extracted archive, which will copy the My App executable to /usr/local/bin and set up tab completion for the url2gif command.


# Requirements
You must have [chromedriver](https://chromedriver.chromium.org/downloads) installed and running on port 9515 (9515 is the default port for chromedriver).  Make sure you install a version that is compatible with your version of Chrome.

# Usage
To use URL2gif, you can run the url2gif command from your terminal, followed by a URL and some optional arguments.

```
url2gif https://www.earthcam.com/cams/newyork/timessquare/ --headless --frames=60
```

The above will create a 60 frame gif.
