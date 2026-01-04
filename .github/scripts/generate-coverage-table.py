import json
import sys
import os

def generate_coverage_table(json_path):
    if not os.path.exists(json_path):
        return "<p>No coverage data found.</p>"

    try:
        with open(json_path, 'r') as f:
            data = json.load(f)
    except Exception as e:
        return f"<p>Error loading coverage data: {e}</p>"

    # llvm-cov json structure: data[0].files
    if not data.get('data'):
        return "<p>Invalid coverage data format.</p>"

    files = data['data'][0].get('files', [])
    
    # Sort files by filename
    files.sort(key=lambda x: x['filename'])

    html = """
    <div class="coverage-container">
        <div class="grid-header">Code Coverage</div>
        <table class="coverage-table">
            <thead>
                <tr>
                    <th>File</th>
                    <th class="text-right">Lines</th>
                    <th class="text-right">Functions</th>
                    <th class="text-right">Instantiations</th>
                </tr>
            </thead>
            <tbody>
    """

    for file in files:
        filename = file['filename']
        # Skip if filename contains "test" or is in a "tests" directory to render only source code coverage if desired
        # For now, we list everything as users often want to see test coverage too, or broadly what was compiled.
        # But usually we filter out standard library or registry files. llvm-cov usually handles project-local filtering.
        
        lines = file['summary']['lines']
        functions = file['summary']['functions']
        instantiations = file['summary']['instantiations']

        line_pct = lines['percent']
        func_pct = functions['percent']
        inst_pct = instantiations['percent']
        
        # Color coding for line coverage
        color_class = "coverage-low"
        if line_pct >= 90:
            color_class = "coverage-high"
        elif line_pct >= 75:
            color_class = "coverage-medium"

        html += f"""
                <tr>
                    <td class="file-cell">{filename}</td>
                    <td class="text-right {color_class}">{line_pct:.1f}%</td>
                    <td class="text-right">{func_pct:.1f}%</td>
                    <td class="text-right">{inst_pct:.1f}%</td>
                </tr>
        """

    html += """
            </tbody>
        </table>
    </div>
    """
    
    return html

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: generate-coverage-table.py <coverage.json>")
        sys.exit(1)
        
    json_path = sys.argv[1]
    print(generate_coverage_table(json_path))
