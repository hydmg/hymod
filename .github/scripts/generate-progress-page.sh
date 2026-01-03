#!/bin/bash
set -e

OUTPUT_DIR="./gh-pages-deploy"
OUTPUT_FILE="$OUTPUT_DIR/index.html"

echo "Generating 'Are We Mod Yet?' page..."

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Run nextest and capture test list (filter out warnings and non-test lines)
echo "Collecting test information..."
# Capture only STDOUT, dropping stderr (compiler warnings)
RAW_TEST_LIST=$(cargo nextest list --all-features --workspace --color never 2>/dev/null)
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ Error: 'cargo nextest list' failed with exit code $EXIT_CODE"
    # Re-run to show error since we silenced it above
    cargo nextest list --all-features --workspace --color never
    exit $EXIT_CODE
fi

TEST_LIST=$(echo "$RAW_TEST_LIST" | grep -E '^[[:space:]]*[a-zA-Z_][a-zA-Z0-9_-]*::' || true)
TEST_COUNT=$(echo "$TEST_LIST" | grep -c "::" || echo "0")
echo "ℹ️  Discovered $TEST_COUNT tests from cargo nextest list."

# Read test output from file if provided
if [ -n "$TEST_OUTPUT_FILE" ]; then
    if [ -f "$TEST_OUTPUT_FILE" ]; then
        echo "ℹ️  Reading test output from $TEST_OUTPUT_FILE..."
        TEST_OUTPUT=$(cat "$TEST_OUTPUT_FILE")
        OUTPUT_LINES=$(echo "$TEST_OUTPUT" | wc -l)
        echo "ℹ️  Loaded $OUTPUT_LINES lines of test output."
    else
        echo "⚠️  Warning: Test output file '$TEST_OUTPUT_FILE' provided but not found!"
    fi
fi

# Count total tests  
if [ -z "$TEST_LIST" ]; then
    echo "❌ Error: No tests found in output!"
    echo "Debug: Raw list output snippet (first 5 lines):"
    echo "$RAW_TEST_LIST" | head -n 5
    exit 1
else
    TOTAL_TESTS=$(echo "$TEST_LIST" | grep -c '::' || echo "0")
    TOTAL_TESTS=$(echo "$TOTAL_TESTS" | tr -d '\n' | tr -d ' ')
    echo "ℹ️  Parsed $TOTAL_TESTS valid test entries."
fi

# Count passing tests
PASSED_TESTS=$(echo "$TEST_OUTPUT" | grep -c 'PASS.* \[' || echo "0")
PASSED_TESTS=$(echo "$PASSED_TESTS" | tr -d '\n' | tr -d ' ')

# Ensure numbers are valid
TOTAL_TESTS=${TOTAL_TESTS:-0}
PASSED_TESTS=${PASSED_TESTS:-0}

# Calculate percentage
if [ "$TOTAL_TESTS" -gt 0 ] 2>/dev/null; then
    PERCENTAGE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
else
    PERCENTAGE=0
fi

echo "Tests: $PASSED_TESTS/$TOTAL_TESTS passing ($PERCENTAGE%)"

# Build tests JSON array
TESTS_JSON="["
FIRST=true

while IFS= read -r line; do
    if [ -z "$line" ]; then
        continue
    fi
    
    # Extract binary ID and full test name
    # nextest list output format: <binary-id> <test-name>
    BINARY_ID=$(echo "$line" | awk '{print $1}')
    FULL_TEST_NAME=$(echo "$line" | awk '{print $2}')
    
    if [ -z "$BINARY_ID" ]; then
        continue
    fi
    
    # Fallback if no second part (though nextest usually provides both)
    if [ -z "$FULL_TEST_NAME" ]; then
        FULL_TEST_NAME="$BINARY_ID"
    fi
    
    # Extract crate name (part before first :: in binary ID)
    CRATE_NAME=$(echo "$BINARY_ID" | cut -d':' -f1)
    
    # Extract display name (everything after last ::, remove test_ prefix, convert underscores to spaces)
    DISPLAY_NAME=$(echo "$FULL_TEST_NAME" | awk -F'::' '{print $NF}' | sed 's/^test_//g' | sed 's/_/ /g')
    
    # Check if test passed (grep matches the fully qualified test name)
    if echo "$TEST_OUTPUT" | grep -q "PASS.*$FULL_TEST_NAME"; then
        STATUS="passed"
    else
        STATUS="failed"
    fi
    
    # Add to JSON
    if [ "$FIRST" = true ]; then
        FIRST=false
    else
        TESTS_JSON="$TESTS_JSON,"
    fi
    
    # Escape quotes in names
    DISPLAY_NAME=$(echo "$DISPLAY_NAME" | sed 's/"/\\"/g')
    CRATE_NAME=$(echo "$CRATE_NAME" | sed 's/"/\\"/g')
    
    TESTS_JSON="$TESTS_JSON{\"name\":\"$DISPLAY_NAME\",\"crate\":\"$CRATE_NAME\",\"status\":\"$STATUS\"}"
    
done <<< "$TEST_LIST"

TESTS_JSON="$TESTS_JSON]"

# Generate HTML
cat > "$OUTPUT_FILE" << 'HTMLEOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <link rel="shortcut icon" type="image/x-icon" href="favicon.ico">
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Are We Mod Yet? - Hymod Development Progress</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
            min-height: 100vh;
            color: #fff;
            padding: 2rem;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
        }
        
        .logo {
            position: fixed;
            top: 2rem;
            right: 2rem;
            width: 60px;
            height: 60px;
            opacity: 0.8;
            transition: opacity 0.3s;
        }
        
        .logo:hover {
            opacity: 1;
        }
        
        .logo img {
            width: 100%;
            height: 100%;
            object-fit: contain;
        }
        
        header {
            text-align: center;
            margin-bottom: 3rem;
        }
        
        h1 {
            font-size: 3.5rem;
            font-weight: 700;
            margin-bottom: 0.5rem;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }
        
        .repo-link {
            color: #a8b2d1;
            text-decoration: none;
            font-size: 1.1rem;
            transition: color 0.3s;
        }
        
        .repo-link:hover {
            color: #667eea;
        }
        
        .progress-section {
            margin-bottom: 3rem;
        }
        
        .stats {
            display: flex;
            justify-content: center;
            gap: 3rem;
            margin-bottom: 2rem;
            flex-wrap: wrap;
        }
        
        .stat {
            text-align: center;
        }
        
        .stat-value {
            font-size: 3rem;
            font-weight: 700;
            color: #667eea;
        }
        
        .stat-label {
            font-size: 0.9rem;
            color: #8892b0;
            text-transform: uppercase;
            letter-spacing: 1px;
        }
        
        .progress-bar-container {
            background: rgba(255, 255, 255, 0.05);
            border-radius: 50px;
            height: 40px;
            overflow: hidden;
            position: relative;
            box-shadow: inset 0 2px 10px rgba(0, 0, 0, 0.3);
        }
        
        .progress-bar {
            height: 100%;
            background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
            border-radius: 50px;
            transition: width 1s ease-out;
            display: flex;
            align-items: center;
            justify-content: flex-end;
            padding-right: 1.5rem;
            font-weight: 600;
            font-size: 1.1rem;
        }
        
        .grid-section {
            margin-top: 3rem;
        }
        
        .grid-header {
            font-size: 1.5rem;
            margin-bottom: 1.5rem;
            color: #ccd6f6;
            font-weight: 600;
        }
        
        .test-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
            gap: 1rem;
            margin-bottom: 3rem;
        }
        
        .test-square {
            aspect-ratio: 1;
            background: rgba(255, 255, 255, 0.05);
            border-radius: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 0.75rem;
            padding: 0.75rem;
            text-align: center;
            cursor: pointer;
            transition: all 0.3s;
            position: relative;
            border: 2px solid transparent;
            word-break: break-word;
        }
        
        .test-square.passed {
            background: linear-gradient(135deg, #10b981 0%, #059669 100%);
            border-color: #34d399;
        }
        
        .test-square.failed {
            background: rgba(255, 255, 255, 0.08);
            border-color: rgba(255, 255, 255, 0.1);
        }
        
        .test-square:hover {
            transform: translateY(-4px);
            box-shadow: 0 8px 20px rgba(0, 0, 0, 0.4);
        }
        
        .test-square.passed:hover {
            box-shadow: 0 8px 20px rgba(16, 185, 129, 0.4);
        }
        
        .tooltip {
            position: absolute;
            bottom: 110%;
            left: 50%;
            transform: translateX(-50%);
            background: rgba(0, 0, 0, 0.95);
            padding: 0.75rem 1rem;
            border-radius: 8px;
            font-size: 0.85rem;
            white-space: nowrap;
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.3s;
            z-index: 10;
            border: 1px solid rgba(255, 255, 255, 0.1);
        }
        
        .test-square:hover .tooltip {
            opacity: 1;
        }
        
        .crate-name {
            color: #667eea;
            font-weight: 600;
        }
        
        footer {
            text-align: center;
            margin-top: 4rem;
            padding-top: 2rem;
            border-top: 1px solid rgba(255, 255, 255, 0.1);
            color: #8892b0;
            font-size: 0.9rem;
        }
        
        footer a {
            color: #667eea;
            text-decoration: none;
        }
        
        footer a:hover {
            text-decoration: underline;
        }
        /* Coverage Table Styles */
        .coverage-container {
            margin-top: 4rem;
            margin-bottom: 4rem;
        }
        .coverage-table {
            width: 100%;
            border-collapse: collapse;
            background: rgba(255, 255, 255, 0.05);
            border-radius: 12px;
            overflow: hidden;
        }
        .coverage-table th, .coverage-table td {
            padding: 1rem;
            text-align: left;
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
        }
        .coverage-table th {
            font-weight: 600;
            color: #ccd6f6;
            background: rgba(255, 255, 255, 0.05);
        }
        .coverage-table tr:hover {
            background: rgba(255, 255, 255, 0.02);
        }
        .coverage-table tr:last-child td {
            border-bottom: none;
        }
        .text-right { text-align: right !important; }
        .coverage-high { color: #10b981; font-weight: 600; }
        .coverage-medium { color: #f59e0b; font-weight: 600; }
        .coverage-low { color: #ef4444; font-weight: 600; }
        .file-cell { font-family: 'SF Mono', 'Fira Code', 'Roboto Mono', monospace; color: #a8b2d1; font-size: 0.9rem; }
    </style>
</head>
<body>
    <a href="https://hydmg.com" class="logo" target="_blank" rel="noopener">
        <img src="logo.png" alt="HYDMG Logo">
    </a>
    
    <div class="container">
        <header>
            <h1>Are We Mod Yet?</h1>
            <a href="https://github.com/hydmg/hymod" class="repo-link" target="_blank" rel="noopener">hymod</a>
        </header>
        
        <div class="progress-section">
            <div class="stats">
                <div class="stat">
                    <div class="stat-value" id="percentage">PERCENTAGE_PLACEHOLDER%</div>
                    <div class="stat-label">Complete</div>
                </div>
                <div class="stat">
                    <div class="stat-value" id="passed">PASSED_PLACEHOLDER</div>
                    <div class="stat-label">Passing</div>
                </div>
                <div class="stat">
                    <div class="stat-value" id="total">TOTAL_PLACEHOLDER</div>
                    <div class="stat-label">Total Tests</div>
                </div>
            </div>
            
            <div class="progress-bar-container">
                <div class="progress-bar" id="progressBar" style="width: PERCENTAGE_PLACEHOLDER%">
                    <span>PERCENTAGE_PLACEHOLDER%</span>
                </div>
            </div>
        </div>
        
        <div class="grid-section">
            <div class="grid-header">Test Implementation Status</div>
            <div id="testGrid" class="test-grid">
                <!-- Tests will be inserted here -->
            </div>
        </div>

        <!-- COVERAGE_PLACEHOLDER -->
        
        <footer>
            Automatically generated from test results
        </footer>
    </div>
    
    <script>
        const tests = TESTS_JSON_PLACEHOLDER;
        const grid = document.getElementById('testGrid');
        
        tests.forEach(test => {
            const square = document.createElement('div');
            square.className = `test-square ${test.status}`;
            square.innerHTML = `
                <span>${test.name}</span>
                <div class="tooltip">
                    <div class="crate-name">${test.crate}</div>
                    <div>${test.status === 'passed' ? '✓ Passing' : '○ Not Implemented'}</div>
                </div>
            `;
            grid.appendChild(square);
        });
    </script>
</body>
</html>
HTMLEOF

# Generate Coverage HTML
echo "Generating coverage table..."
python3 .github/scripts/generate-coverage-table.py "${COVERAGE_JSON:-coverage.json}" > coverage_chunk.html

# Replace placeholders in HTML
sed -i.bak "s/PERCENTAGE_PLACEHOLDER/$PERCENTAGE/g" "$OUTPUT_FILE"
sed -i.bak "s/PASSED_PLACEHOLDER/$PASSED_TESTS/g" "$OUTPUT_FILE"
sed -i.bak "s/TOTAL_PLACEHOLDER/$TOTAL_TESTS/g" "$OUTPUT_FILE"
sed -i.bak "s|TESTS_JSON_PLACEHOLDER|$TESTS_JSON|g" "$OUTPUT_FILE"

# Inject Coverage HTML using awk for safe multiline insertion
# We use a temporary file to construct the final output
awk '
    /<!-- COVERAGE_PLACEHOLDER -->/ {
        system("cat coverage_chunk.html")
        next
    }
    { print }
' "$OUTPUT_FILE" > "$OUTPUT_FILE.tmp" && mv "$OUTPUT_FILE.tmp" "$OUTPUT_FILE"

rm -f coverage_chunk.html

# Clean up backup file
rm -f "$OUTPUT_FILE.bak"

echo "✅ Progress page generated at: $OUTPUT_FILE"
echo "Stats: $PASSED_TESTS/$TOTAL_TESTS tests passing ($PERCENTAGE%)"
