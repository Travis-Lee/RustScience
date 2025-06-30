# RustScience

The Rust community focuses on learning the language, discussing technical challenges, and creating innovative software projects.

The following content focuses on macOS.

1) Ensure Rust is installed. If not, open a terminal and install it.
   - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 

2) Once the installation is complete, either restart your terminal or run 'source ~/.cargo/env' to apply the environment variables.

3) Create a new directory for your project as follows:
   - mkdir demo
   - cd demo
   - cargo init --vcs none

4) Navigate to the project root directory, then compile and run the project.  
   - cargo build --release 
   After building in release mode, the optimized binary will be located in the target/release folder.   

5) Once the project has been successfully compiled, you can run the program.
   "./target/release/matrix_multiplication" or "cargo run --release"

