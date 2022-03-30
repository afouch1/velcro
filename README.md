# Velcro

It *stitches* folders together!

Velcro was built out of a necessity to automate the creation
of large folder hierarchies and as such allows the user to reference
a configuration file (written in yaml) to create large complex
folder structures. 

# Install

To install, either build from source or `cargo install --git https://github.com/afouch1/velcro.git`. 

To use simply navigate to the location where you wish to create the folders
and `velcro /path/to/config.yml`. 

# Config File

The configuration file is a Yaml file that utilizes a very small subset of Yaml
with the following rules:

 - The top level document must be a hash set where each item is an array.
 - Only strings and arrays of strings are allowed except in the top level document
 - The top level document must contain an item called `folders` that represents the folder hierarchy.
 - All other top level items are "named groups" to reuse given folder structures

An example configuration:

```yaml
folders: 
    - Movies:
        - Iron Man:
            - movie
        - "Avengers: Endgame": # Note the quotation marks since our movie contains a colon
            - movie
    - Music:
        - genres
    - Images

movie:
    - screenshots
    - movie
    - reviews

genres:
    - Rock
    - Jazz
    - Extreme Death Metal
```

This will create a folder structure as the following:

```
.
├── Images
├── Movies
│   ├── Avengers: Endgame
│   │   ├── reviews
│   │   ├── screenshots
│   │   └── video
│   └── Iron Man
│       ├── reviews
│       ├── screenshots
│       └── video
└── Music
    ├── Extreme Death Metal
    ├── Jazz
    └── Rock
```