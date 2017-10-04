Demo of how to interface Java frontend with a C (native) backend. I have created a bash script file called "run" which one can select (on Linux ofc) to simply build all components (native and Java) and execute the project for ease. To manually do it, one can always look into the script file and take hints from there.

Interfacing involves callbacks passed from Java to C (for C to callback into Java for giving it the results) which is more troublesome than simple parameter passing from Java to C.
