# Test Precision Mode vs Fast Mode

$PDF = "data\1706.03762v7.pdf"
$OUTPUT_FAST = "data\output_fast_mode.md"
$OUTPUT_PRECISION = "data\output_precision_mode.md"
$OUTPUT_DOCLING = "data\output_docling.md"

Write-Host "=== Testing Transmutation Modes ===" -ForegroundColor Cyan
Write-Host ""

# Test Fast Mode (default)
Write-Host "[1/2] Running FAST mode (pure Rust heuristics)..." -ForegroundColor Green
Measure-Command {
    & ".\target\release\transmutation.exe" convert $PDF -o $OUTPUT_FAST -f markdown
} | Select-Object -ExpandProperty TotalSeconds | ForEach-Object {
    Write-Host "  Completed in: $_ seconds" -ForegroundColor Gray
}

# Test Precision Mode
Write-Host "[2/2] Running PRECISION mode (Docling-style analysis)..." -ForegroundColor Yellow
Measure-Command {
    & ".\target\release\transmutation.exe" convert $PDF -o $OUTPUT_PRECISION -f markdown --precision
} | Select-Object -ExpandProperty TotalSeconds | ForEach-Object {
    Write-Host "  Completed in: $_ seconds" -ForegroundColor Gray
}

Write-Host ""
Write-Host "=== Results ===" -ForegroundColor Cyan

# Calculate file sizes
$size_fast = (Get-Item $OUTPUT_FAST).Length
$size_precision = (Get-Item $OUTPUT_PRECISION).Length
$size_docling = (Get-Item $OUTPUT_DOCLING).Length

Write-Host "Fast Mode:      $size_fast bytes" -ForegroundColor Green
Write-Host "Precision Mode: $size_precision bytes" -ForegroundColor Yellow
Write-Host "Docling:        $size_docling bytes" -ForegroundColor Blue

Write-Host ""
Write-Host "=== Similarity Comparison ===" -ForegroundColor Cyan

# Compare with Docling using Python
$pythonCode = @"
from difflib import SequenceMatcher

def similarity(a, b):
    return SequenceMatcher(None, a, b).ratio() * 100

# Read files
with open('$OUTPUT_FAST', 'r', encoding='utf-8') as f:
    fast = f.read()
with open('$OUTPUT_PRECISION', 'r', encoding='utf-8') as f:
    precision = f.read()
with open('$OUTPUT_DOCLING', 'r', encoding='utf-8') as f:
    docling = f.read()

# Calculate similarities
fast_sim = similarity(fast, docling)
precision_sim = similarity(precision, docling)

print(f'Fast Mode vs Docling:      {fast_sim:.1f}%')
print(f'Precision Mode vs Docling: {precision_sim:.1f}%')
print(f'Improvement: +{(precision_sim - fast_sim):.1f}%')
"@

python -c $pythonCode

Write-Host ""
Write-Host "Files saved:" -ForegroundColor Cyan
Write-Host "  Fast mode:      $OUTPUT_FAST" -ForegroundColor Gray
Write-Host "  Precision mode: $OUTPUT_PRECISION" -ForegroundColor Gray

