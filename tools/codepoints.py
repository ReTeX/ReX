#!/bin/python

from fontTools.ttLib import TTFont
import sys

if len(sys.argv) != 2:
    print("usage: codepoints.py font.otf")
    sys.exit(1)
    
font  = TTFont(sys.argv[1])
cmaps = font['cmap'].tables

results = {}
for cmap in cmaps:
    print(cmap.cmap)
    for code, name in cmap.cmap.items():
        if "size" in name: print(name)
        results[name] = results.get(name, [])
        if code not in results[name]:
            results[name].append(code)

print("Symbols with multiple code points")
for name in results:
    if "slash" in name:
        print(name, results[name])
