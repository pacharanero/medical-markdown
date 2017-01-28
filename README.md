Tests for a very early, unelaborated idea about a Markdown-like specification 
for quickly writing free text medical notes that can be programmatically 
parsed into structured data, with terminologies where appropriate.

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
