#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import sys
sys.stdout.reconfigure(encoding='utf-8')

from difflib import SequenceMatcher

# Read files
with open('data/output_docling.md', 'r', encoding='utf-8') as f:
    docling = f.read()

with open('data/output_precision_mode.md', 'r', encoding='utf-8') as f:
    precision = f.read()

# Calculate overall similarity
sim = SequenceMatcher(None, precision, docling).ratio() * 100
print(f"Overall Similarity: {sim:.1f}%")
print("=" * 80)

# Split into lines for detailed analysis
docling_lines = docling.split('\n')
precision_lines = precision.split('\n')

print(f"\nDocling lines: {len(docling_lines)}")
print(f"Precision lines: {len(precision_lines)}")
print(f"Difference: {abs(len(docling_lines) - len(precision_lines))} lines")

# Find first 10 major differences
print("\n" + "=" * 80)
print("FIRST 10 MAJOR DIFFERENCES:")
print("=" * 80)

from difflib import unified_diff
diff = list(unified_diff(docling_lines[:100], precision_lines[:100], lineterm='', n=1))

diff_count = 0
for i, line in enumerate(diff):
    if line.startswith('---') or line.startswith('+++'):
        continue
    if line.startswith('@@'):
        print(f"\n{line}")
        diff_count += 1
        if diff_count > 10:
            break
    elif line.startswith('-'):
        # Line in docling but not in precision
        print(f"  EXPECTED: {line[1:]}")
    elif line.startswith('+'):
        # Line in precision but not in docling
        print(f"  GOT:      {line[1:]}")

# Character-level analysis of first 500 chars
print("\n" + "=" * 80)
print("CHARACTER-LEVEL COMPARISON (first 500 chars):")
print("=" * 80)
print("\nDOCLING:")
print(docling[:500])
print("\nPRECISION:")
print(precision[:500])

# Find sections with low similarity
print("\n" + "=" * 80)
print("SECTION SIMILARITY ANALYSIS:")
print("=" * 80)

# Split by headings
import re
docling_sections = re.split(r'\n## ', docling)
precision_sections = re.split(r'\n## ', precision)

for i, (d_sec, p_sec) in enumerate(zip(docling_sections[:5], precision_sections[:5])):
    if len(d_sec) < 10:
        continue
    sec_sim = SequenceMatcher(None, d_sec, p_sec).ratio() * 100
    title = d_sec.split('\n')[0][:50]
    print(f"Section {i} ({title}...): {sec_sim:.1f}%")

