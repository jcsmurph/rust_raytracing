# Rust RayTracing

Rust Implementation of https://raytracing.github.io/books/RayTracingInOneWeekend.html.

# To Run:
1. Clone the repository to a local directory
2. Run cargo build via command line
3. Run cargo run via command line

# TODOS:
1. Implement testing
2. Print processing message in Gui
3. Show image on "Generate Image" function rather than persistent

# Known Issues
1. If image size it too large, an error message that "'Unknown' is not responding" will appear.

Short term fix:
The image is still being created and the message will disappear once completed.

Long term fix:
Add processing state to context(?)


2. Build will not compile if output.ppm does not exist in src/

Short term fix:
Add dummy output.ppm file

Long term fix:
Only access image on "Generate Image" rather than persistent

3. Image appears in Side Panel with Gui button/sliders

Short term fix:
None

Long term fix:
Reformat Gui for image to show in right window
