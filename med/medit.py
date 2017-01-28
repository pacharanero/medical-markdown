import sys

import markdown
from medmarkdown import MedExtension

extension = MedExtension()

data = sys.stdin.read()
html = markdown.markdown(data, extensions=[extension])
print html
print '-' * 80
print extension.get_structured_data()
