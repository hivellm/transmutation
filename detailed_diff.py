from difflib import SequenceMatcher, unified_diff

# Read files
with open('data/output_precision_mode.md', 'r', encoding='utf-8') as f:
    precision_lines = f.readlines()

with open('data/output_docling.md', 'r', encoding='utf-8') as f:
    docling_lines = f.readlines()

# Find major differences
print("=" * 80)
print("MAJOR DIFFERENCES (first 100 lines)")
print("=" * 80)

diff = list(unified_diff(docling_lines[:100], precision_lines[:100], 
                         fromfile='docling', tofile='precision', lineterm=''))

issues = []
for i, line in enumerate(diff[:200]):  # Show first 200 diff lines
    if line.startswith('---') or line.startswith('+++') or line.startswith('@@'):
        print(f"\n{line}")
    elif line.startswith('-'):  # Missing in our output
        print(f"MISSING: {line[1:80]}")
        issues.append(('missing', line[1:].strip()))
    elif line.startswith('+'):  # Extra in our output
        print(f"EXTRA:   {line[1:80]}")
        issues.append(('extra', line[1:].strip()))

print("\n" + "=" * 80)
print("KEY ISSUES TO FIX:")
print("=" * 80)

# Analyze patterns
missing_breaks = sum(1 for t, content in issues if t == 'missing' and content == '')
extra_breaks = sum(1 for t, content in issues if t == 'extra' and content == '')
missing_headers = sum(1 for t, content in issues if t == 'missing' and content.startswith('##'))
wrong_joins = sum(1 for t, content in issues if t == 'extra' and 'Abstract' in content or 'Introduction' in content)

print(f"1. Missing blank lines: {missing_breaks}")
print(f"2. Extra blank lines: {extra_breaks}")
print(f"3. Missing headers (##): {missing_headers}")
print(f"4. Wrong text joins: {wrong_joins}")

# Calculate similarity by sections
print("\n" + "=" * 80)
print("SIMILARITY BY SECTION:")
print("=" * 80)

def find_section(lines, start_pattern):
    for i, line in enumerate(lines):
        if start_pattern in line:
            return i
    return 0

# Abstract section
abs_start_d = find_section(docling_lines, '## Abstract')
abs_start_p = find_section(precision_lines, '## Abstract')
intro_start_d = find_section(docling_lines, '## 1 Introduction')
intro_start_p = find_section(precision_lines, '## 1 Introduction')

if abs_start_d and abs_start_p and intro_start_d and intro_start_p:
    abstract_d = ''.join(docling_lines[abs_start_d:intro_start_d])
    abstract_p = ''.join(precision_lines[abs_start_p:intro_start_p])
    abs_sim = SequenceMatcher(None, abstract_d, abstract_p).ratio() * 100
    print(f"Abstract section: {abs_sim:.1f}%")

# Overall
total_sim = SequenceMatcher(None, ''.join(docling_lines), ''.join(precision_lines)).ratio() * 100
print(f"Overall: {total_sim:.1f}%")

