from difflib import SequenceMatcher

# Read files
with open('data/output_fast_mode.md', 'r', encoding='utf-8') as f:
    fast = f.read()

with open('data/output_precision_mode.md', 'r', encoding='utf-8') as f:
    precision = f.read()

with open('data/output_docling.md', 'r', encoding='utf-8') as f:
    docling = f.read()

# Calculate similarities
fast_sim = SequenceMatcher(None, fast, docling).ratio() * 100
precision_sim = SequenceMatcher(None, precision, docling).ratio() * 100
improvement = precision_sim - fast_sim

# Print results
print("========== SIMILARITY COMPARISON ==========")
print(f"Fast Mode:      {fast_sim:5.1f}% (Rust heuristics)")
print(f"Precision Mode: {precision_sim:5.1f}% (Docling-style)")
print(f"Improvement:    {improvement:+5.1f}%")
print(f"Target:         95.0%")
status = "✓ PASSED" if precision_sim >= 95 else "✗ NEEDS WORK"
print(f"Status:         {status}")
print("=" * 43)

