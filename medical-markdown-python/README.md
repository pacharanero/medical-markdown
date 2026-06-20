> **Legacy / historical reference.** This Python package is the original experimental implementation of Medical Markdown. It has been superseded by the canonical Rust crate at the repository root, and its behaviour is now captured by the language-agnostic conformance suite in `../tests/conformance/`. It is kept only for historical reference and is slated for removal once nothing depends on it. New work should target the Rust crate; do not add features here.

Tests for medical-markdown-python experimental implementation

This is currently implemented as a python-markdown extension, and the 
specification is found in medical-markdown.

## Installation

1. create a virtualenv and activate it
2. git clone git@github.com:open-health-hub/medical-markdown-spec.git
3. pip install -r requirements.txt
4. ```python med/medit.py < test_data/simple.txt``` to see what it doesi

## Notes

The markdown extension found in med/medmarkdown.py, as well as returning
replacement markdown lines, also stores a structured representation of the 
medical codes in a dict called structured. These codes ```XX/Notes``` can be 
interspersed with normal markdown, although currently markdown cannot appear 
in the notes themselves

See med/medit.py for use
