How to run:
1. cd into the wanted versions /src/ directory
   1. Version 1 is found at ./version_1/src
   2. Version 2 is found at ./version_2/src
2. run `cargo run ./data` to run the program with the data directory path as the argument

Improvements from version 1 to version 2:
Through the use of threading, a significant performance increase was achieved going from just under 2ms to around 0.5ms. 