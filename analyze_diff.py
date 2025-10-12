import difflib

# Read files
with open('data/output_docling.md', 'r', encoding='utf-8') as f:
    docling_lines = f.readlines()

with open('data/output_fast_mode.md', 'r', encoding='utf-8') as f:
    fast_lines = f.readlines()

print("=" * 80)
print("FIRST 30 LINES COMPARISON")
print("=" * 80)

print("\nDOCLING (reference - first 30 lines):")
print("-" * 80)
for i, line in enumerate(docling_lines[:30], 1):
    print(f"{i:3}: {line.rstrip()}")

print("\n" + "=" * 80)
print("FAST MODE (ours - first 30 lines):")
print("-" * 80)
for i, line in enumerate(fast_lines[:30], 1):
    print(f"{i:3}: {line.rstrip()}")

print("\n" + "=" * 80)
print("DIFF ANALYSIS (what's missing/wrong in our output)")
print("=" * 80)

# Calculate difference
differ = difflib.Differ()
diff = list(differ.compare(docling_lines[:50], fast_lines[:50]))

print("\nKey differences (first 50 lines):")
for i, line in enumerate(diff[:100]):  # Show first 100 diff lines
    if line.startswith('- '):  # Line in docling but not in ours
        print(f"MISSING: {line[2:].rstrip()}")
    elif line.startswith('+ '):  # Line in ours but not in docling
        print(f"EXTRA:   {line[2:].rstrip()}")

