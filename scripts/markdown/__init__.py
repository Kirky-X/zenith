#!/usr/bin/env python3
"""Unified Markdown linting fix tool for Zenith project."""

import argparse
import sys
from pathlib import Path

from rules import (
    CONTRIBUTING_RULES,
    FAQ_RULES,
    PRD_RULES,
    MarkdownFixer,
)

# Rule mappings for different documents
DOCUMENT_RULES = {
    'CONTRIBUTING.md': CONTRIBUTING_RULES,
    'FAQ.md': FAQ_RULES,
    'prd.md': PRD_RULES,
    'USER_GUIDE.md': [
        MarkdownFixer.fix_md012_multiple_blank_lines,
        MarkdownFixer.fix_md047_file_ends_newline,
    ],
    'API_REFERENCE.md': [
        MarkdownFixer.fix_md012_multiple_blank_lines,
        MarkdownFixer.fix_md047_file_ends_newline,
    ],
}


def apply_rules(content: str, rules: list) -> str:
    """Apply a list of rules to content."""
    for rule in rules:
        content = rule(content)
    return content


def fix_markdown_file(file_path: Path, rules: list = None) -> bool:
    """Fix a single markdown file."""
    if not file_path.exists():
        print(f"Error: File not found: {file_path}")
        return False

    content = file_path.read_text(encoding='utf-8')
    original_content = content

    if rules is None:
        # Auto-detect rules based on filename
        rules = DOCUMENT_RULES.get(file_path.name, [
            MarkdownFixer.fix_md012_multiple_blank_lines,
            MarkdownFixer.fix_md047_file_ends_newline,
        ])

    content = apply_rules(content, rules)

    if content != original_content:
        file_path.write_text(content, encoding='utf-8')
        print(f"✓ Fixed: {file_path}")
        return True
    else:
        print(f"○ No changes: {file_path}")
        return False


def main():
    parser = argparse.ArgumentParser(
        description='Unified Markdown fix tool for Zenith project',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python fix_markdown.py docs/CONTRIBUTING.md
  python fix_markdown.py docs/FAQ.md docs/prd.md
  python fix_markdown.py --all
  python fix_markdown.py --check docs/FAQ.md
        """,
    )

    parser.add_argument(
        'files',
        nargs='*',
        help='Markdown files to fix',
    )
    parser.add_argument(
        '--all',
        action='store_true',
        help='Fix all known markdown files in docs/',
    )
    parser.add_argument(
        '--check',
        action='store_true',
        help='Check files for issues without modifying',
    )
    parser.add_argument(
        '--rules',
        choices=['CONTRIBUTING', 'FAQ', 'PRD'],
        help='Specify rule set to use',
    )

    args = parser.parse_args()

    if args.check:
        print("Check mode: Not yet implemented")
        sys.exit(0)

    if args.all:
        docs_dir = Path(__file__).parent.parent.parent / 'docs'
        files_to_fix = [
            docs_dir / name
            for name in DOCUMENT_RULES.keys()
            if (docs_dir / name).exists()
        ]
    elif args.files:
        files_to_fix = [Path(f) for f in args.files]
    else:
        parser.print_help()
        sys.exit(1)

    # Determine rule set
    if args.rules:
        rules_map = {
            'CONTRIBUTING': DOCUMENT_RULES.get('CONTRIBUTING.md', []),
            'FAQ': DOCUMENT_RULES.get('FAQ.md', []),
            'PRD': DOCUMENT_RULES.get('prd.md', []),
        }
        global_rules = rules_map[args.rules]
    else:
        global_rules = None

    fixed_count = 0
    for file_path in files_to_fix:
        rules = global_rules
        if rules is None:
            rules = DOCUMENT_RULES.get(file_path.name, [
                MarkdownFixer.fix_md012_multiple_blank_lines,
                MarkdownFixer.fix_md047_file_ends_newline,
            ])

        if fix_markdown_file(file_path, rules):
            fixed_count += 1

    print(f"\nCompleted: {fixed_count} file(s) fixed")


if __name__ == '__main__':
    main()
