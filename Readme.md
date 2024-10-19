# Dirguard

Dirguard is a Rust application designed to monitor file system events within a specified directory and take action based on a predefined configuration. It uses the `hotwatch` crate for monitoring file system changes and `log` crate for logging events.

## Features
- Monitors file creation and modification events in a specified directory.
- Deletes files that match predefined names specified in a configuration file.
- Logs events and actions to a log file.

## Requirements
- Rust programming language (version 1.50 or above recommended).
- `hotwatch` crate for file system monitoring.
- `log` and `env_logger` crates for logging.

## Installation
1. Clone the repository:
     sh
    git clone https://github.com/nyaaaww/Dirguard.git
    cd Dirguard
     

2. Build the application:
     sh
    cargo build --release
     

3. Run the application:
     sh
    ./dg
     

## Usage
Run the application with the path to the directory you want to monitor as the argument:
 sh
./target/release/dg /path/to/watch
 

## Configuration
The application reads a configuration file named `config.txt` in the root directory. Add the names of the files you want to be deleted when created or modified, one per line. Lines starting with `#` are treated as comments and are ignored.

##Example 
This is an example for `config.txt`.
List of files to be deleted when created or modified:
  ``` txt
     secret.txt  
     confidential.pdf  
 ```

## Logging
Events and actions taken by the application are logged to a file named `hotwatch.log` in the root directory.

## Contributing
Contributions to the project are welcome. Please ensure that you follow the existing code style and add appropriate tests for any new features.

## License
This project is licensed under the GPL License - see the LICENSE file for details.

## Contact
For any questions or suggestions, please open an issue on the GitHub repository.

---

Please replace `https://github.com/nyaaaww/Dirguard.git` with the actual URL of your repository and adjust any other details as necessary.