Create a virtual enviroment with:  
`python -m venv .venv`  
Activate it with:  
`source .venv/bin/activate`  
  
If you're on the wrong python version, you could run:  
`export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1`  
This is not adviced but it should work.  
  
The command to build the library is:  
`maturin develop`  