from difflib import SequenceMatcher

# Read files
with open('data/output_precision_mode.md', 'r', encoding='utf-8') as f:
    precision = f.read()

with open('data/output_docling.md', 'r', encoding='utf-8') as f:
    docling = f.read()

# Calculate similarity
sim = SequenceMatcher(None, precision, docling).ratio() * 100

print("=" * 50)
print(f"Precision Mode Similarity: {sim:.1f}%")
print(f"Target:                    95.0%")
print(f"Status:                    {'PASS ✓' if sim >= 95 else 'NEEDS WORK ✗'}")
print("=" * 50)

if sim < 95:
    print(f"\nStill need: {95 - sim:.1f}% improvement")

