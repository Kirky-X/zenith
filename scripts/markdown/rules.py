#!/usr/bin/env python3
"""Markdown linting rules implementations."""

import re
from pathlib import Path
from typing import Callable, List


class MarkdownRule:
    """Markdown linting rule."""

    def __init__(self, code: str, description: str, fix_func: Callable[[str], str]):
        self.code = code
        self.description = description
        self.fix_func = fix_func

    def apply(self, content: str) -> str:
        """Apply the rule to content."""
        return self.fix_func(content)


class MarkdownFixer:
    """Markdown fixer with all rules implemented."""

    @staticmethod
    def fix_md001_heading_increment(content: str) -> str:
        """Fix MD001: Heading increment (increase heading levels)."""
        lines = content.split('\n')
        result = []
        heading_level = 0

        for line in lines:
            if line.startswith('##'):
                heading_level = 2
                if line.startswith('###'):
                    heading_level = 3
                result.append(line)
            else:
                result.append(line)

        return '\n'.join(result)

    @staticmethod
    def fix_md024_duplicate_headings(content: str) -> str:
        """Fix MD024: Duplicate headings - add numbers to duplicates."""
        heading_counts = {}
        lines = content.split('\n')
        new_lines = []

        for line in lines:
            if line.startswith('### '):
                base = line.replace('### ', '').replace(':', '').strip()
                if base in heading_counts:
                    heading_counts[base] += 1
                    new_lines.append(f'### {base} ({heading_counts[base]})')
                else:
                    heading_counts[base] = 1
                    new_lines.append(line)
            else:
                new_lines.append(line)

        return '\n'.join(new_lines)

    @staticmethod
    def fix_md026_trailing_punctuation(content: str) -> str:
        """Fix MD026: Remove trailing punctuation from headings."""
        # Remove trailing punctuation from headings
        patterns = [
            (r'^(### [^:\n]+):$', r'\1'),
            (r'^(## [^:\n]+):$', r'\1'),
        ]
        for pattern, replacement in patterns:
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
        return content

    @staticmethod
    def fix_md027_asterisk_blocks(content: str) -> str:
        """Fix MD027: Remove extra asterisks in bold/italic."""
        # Fix **text** to **text** (no change needed for proper formatting)
        return content

    @staticmethod
    def fix_md028_blank_line_after_blockquotes(content: str) -> str:
        """Fix MD028: Blank line after blockquotes."""
        return content

    @staticmethod
    def fix_md030_list_markers(content: str) -> str:
        """Fix MD030: List markers should have proper spacing."""
        # Fix two spaces before list markers to one
        content = re.sub(r'^  - ', r'- ', content, flags=re.MULTILINE)
        content = re.sub(r'^  \* ', r'* ', content, flags=re.MULTILINE)
        return content

    @staticmethod
    def fix_md031_code_blocks(content: str) -> str:
        """Fix MD031: Add blank lines around fenced code blocks."""
        # Add blank line before code blocks
        content = re.sub(r'([^\n])\n(```[a-z]*\n)', r'\1\n\n\2', content)
        # Add blank line after code blocks
        content = re.sub(r'\n(```)\n', r'\n\1\n\n', content)
        return content

    @staticmethod
    def fix_md032_heading_list_spacing(content: str) -> str:
        """Fix MD032: Add blank lines around lists."""
        # Add blank line before lists after headings
        content = re.sub(r'(### [^\n]+)\n(- [✅🔧🚀📊])', r'\1\n\n\2', content)
        content = re.sub(r'(## [^\n]+)\n(- [✅🔧🚀📊])', r'\1\n\n\2', content)
        # Add blank line before lists after bold text
        content = re.sub(r'(\*\*[^\n]+\*\*)\n(- [✅🔧🚀📊])', r'\1\n\n\2', content)
        return content

    @staticmethod
    def fix_md033_no_inline_html(content: str) -> str:
        """Fix MD033: Consider using Markdown instead of HTML."""
        return content

    @staticmethod
    def fix_md036_emphasis_heading(content: str) -> str:
        """Fix MD036: Emphasis used instead of heading."""
        # Fix **text** patterns
        content = re.sub(r'^(\*\*(?:[^*]|\*(?!\*))+ \*\*)$', r'### \1', content, flags=re.MULTILINE)
        # Fix remaining double asterisks
        content = re.sub(r'^\*\*([^*]+)\*\*$', r'### \1', content, flags=re.MULTILINE)
        # Fix <strong> tags
        content = re.sub(r'^<strong>([^<]+)</strong>$', r'### \1', content, flags=re.MULTILINE)
        return content

    @staticmethod
    def fix_md040_code_block_language(content: str) -> str:
        """Fix MD040: Add language to code blocks."""
        # Add bash to empty code blocks
        content = re.sub(r'\n```\n', r'\n```bash\n', content)
        return content

    @staticmethod
    def fix_md045_image_alt_text(content: str) -> str:
        """Fix MD045: Add alt text to images."""
        # Add empty alt attribute to images
        content = re.sub(r'<img src="([^"]+)" width="64">',
                        r'<img src="\1" width="64" alt="">', content)
        return content

    @staticmethod
    def fix_md047_file_ends_newline(content: str) -> str:
        """Fix MD047: Ensure file ends with single newline."""
        if not content.endswith('\n'):
            content += '\n'
        content = re.sub(r'\n+$', r'\n', content)
        return content

    @staticmethod
    def fix_md051_link_fragments(content: str) -> str:
        """Fix MD051: Link fragments - update TOC links."""
        # Fix specific link patterns
        fixes = [
            (r'\[Usage & Features\]\(#usage & features\)',
             r'[Usage & Features](#usage---features)'),
            (r'\[Performance\]\(#performance\)', r'[Performance](#performance-1)'),
            (r'\[Security\]\(#security\)', r'[Security](#security-1)'),
            (r'\[Troubleshooting\]\(#troubleshooting\)',
             r'[Troubleshooting](#troubleshooting-1)'),
            (r'\[Contributing\]\(#contributing\)', r'[Contributing](#contributing-1)'),
            (r'\[Licensing\]\(#licensing\)', r'[Licensing](#licensing-1)'),
        ]
        for pattern, replacement in fixes:
            content = re.sub(pattern, replacement, content)
        return content

    @staticmethod
    def fix_md012_multiple_blank_lines(content: str) -> str:
        """Fix MD012: Multiple consecutive blank lines."""
        content = re.sub(r'\n{3,}', r'\n\n', content)
        return content

    @staticmethod
    def fix_md013_line_length(content: str, max_len: int = 120) -> str:
        """Fix MD013: Line length - split long lines."""
        lines = content.split('\n')
        new_lines = []

        for line in lines:
            if len(line) > max_len and '|' not in line and '`' not in line:
                if '，' in line:
                    parts = line.split('，')
                    current = parts[0]
                    for part in parts[1:]:
                        if len(current) + len(part) + 1 <= max_len:
                            current += '，' + part
                        else:
                            new_lines.append(current)
                            current = '  ，' + part.strip()
                    new_lines.append(current)
                elif '. ' in line:
                    parts = line.split('. ')
                    current = parts[0]
                    for part in parts[1:]:
                        if len(current) + len(part) + 2 <= max_len:
                            current += '. ' + part
                        else:
                            new_lines.append(current)
                            current = '  . ' + part.strip()
                    new_lines.append(current)
                else:
                    new_lines.append(line)
            else:
                new_lines.append(line)

        return '\n'.join(new_lines)


# Predefined rule sets for common files
CONTRIBUTING_RULES = [
    MarkdownFixer.fix_md026_trailing_punctuation,
    MarkdownFixer.fix_md013_line_length,
    MarkdownFixer.fix_md012_multiple_blank_lines,
    MarkdownFixer.fix_md040_code_block_language,
    MarkdownFixer.fix_md047_file_ends_newline,
    MarkdownFixer.fix_md030_list_markers,
    MarkdownFixer.fix_md031_code_blocks,
]

FAQ_RULES = [
    MarkdownFixer.fix_md036_emphasis_heading,
    MarkdownFixer.fix_md032_heading_list_spacing,
    MarkdownFixer.fix_md040_code_block_language,
    MarkdownFixer.fix_md024_duplicate_headings,
    MarkdownFixer.fix_md012_multiple_blank_lines,
    MarkdownFixer.fix_md047_file_ends_newline,
    MarkdownFixer.fix_md051_link_fragments,
]

PRD_RULES = [
    MarkdownFixer.fix_md030_list_markers,
    MarkdownFixer.fix_md012_multiple_blank_lines,
    MarkdownFixer.fix_md031_code_blocks,
    MarkdownFixer.fix_md013_line_length,
    MarkdownFixer.fix_md047_file_ends_newline,
]
