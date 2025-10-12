#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import sys
sys.stdout.reconfigure(encoding='utf-8')

# Test author lines
with open('data/output_docling.md', 'r', encoding='utf-8') as f:
    docling_lines = [l.rstrip() for l in f.readlines()]

with open('data/output_precision_mode.md', 'r', encoding='utf-8') as f:
    precision_lines = [l.rstrip() for l in f.readlines()]

print("DOCLING AUTHOR LINES (lines 5-22):")
for i in range(4, min(22, len(docling_lines))):
    print(f"{i+1:3}: '{docling_lines[i]}'")

print("\nPRECISION AUTHOR LINES (lines 5-30):")
for i in range(4, min(30, len(precision_lines))):
    print(f"{i+1:3}: '{precision_lines[i]}'")

# Count differences in author region
author_diffs = 0
for i in range(4, 22):
    if i < len(docling_lines) and i < len(precision_lines):
        if docling_lines[i] != precision_lines[i]:
            author_diffs += 1

print(f"\nDifferences in author region: {author_diffs} / 18 lines")

