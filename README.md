# Python S3D Loader and Blender Importer

This repository contains two projects.  

One is a Python package to open EverQuest .s3d files and read their contents.

The other is a Blender addon which uses the Python package to import EverQuest .s3d files into Blender.

# Building the Python package

## Install Rust toolchain

This project uses Rust, so you must have that installed in order to build this package.  https://www.rust-lang.org/

## Create a virtualenv

I recommend using the Python version for whatever Blender version you are using.  At the time of this writing, for Blender 4.3, that version is Python 3.11.

`virtualenv --python 3.11 .venv`
`source .venv/bin/activate`

## Install dependencies

`pip install -e .`

Or, if you want to run tests too,

`pip install -e .[dev]`

## Running tests

I recommend that you run these tests before attempting to use this library, to ensure everything is working.

To run tests, you must supply the following files, and put them in the `./python/tests/fixtures` folder:

```
rivervale.s3d
rivervale_chr.s3d
rivervale_obj.s3d
```

Then you can run pytest.

`pytest`


## Python Example Usage

For now, I would recommend looking at the tests and the Blender addon to see how this package is used in Python.
The API is subject to change.

## Installing the Blender addon

The Blender addon requires the Python package above.  So first, install the package into the blender addon folder.

`pip install . --target ./blender/addons/eqloader`

Then add the `./blender` folder to the 'Script paths' section in Blender preferences.

Then find the addon in the 'Addons' section of preferences and enable it.

## Using the Blender addon

You should see an 'Everquest .s3d' option in the File->Import menu.  There are various options to customize the import process.

## PEQ Door Loader

The 'doors' and some other objects in various zones in Everquest are not defined in the .s3d files, but instead on the server.  I have created a different project which creates a usable `sqlite` database with required zone door information from the PEQ project.