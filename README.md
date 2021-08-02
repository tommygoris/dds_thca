
# How to build and compile

Ensure Cargo and your favorite Rust toolchain is installed properly.

1. Clone the repository locally
2. Follow the instructions in https://github.com/Rust-SDL2/rust-sdl2 to build and compile Rust-SDL2 projects.
  - Depending on which Rust toolchain you have installed, follow the instructions in the Readme in the link above.
  - For example, will need to download the necessary SDL libraries and place them in the correct Rust lib folder.
  - The libraries required are listed below. The instructions listed in the link above will only go over the main SDL2 lib/dlls but all the libs below are required
    to compile and build this SDL2 rust project.
      1. https://www.libsdl.org/download-2.0.php (main SDL2 lib/dll mentioned in the link provided) 
      2. https://www.libsdl.org/projects/SDL_image/ (SDL2 image lib/dll)
      3. https://www.libsdl.org/projects/SDL_ttf/ (SDL2 ttf for fonts lib/dll)     
3. Open a command prompt in the folder location of the cloned repository.
4. In the command prompt window, run the following comand: Cargo run --release
5. The project will now compile and build. If any errors are observed, ensure Cargo and Rust is installed properly. Also, ensure the libs in step 2 were installed correctly.
