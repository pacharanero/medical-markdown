import re

from markdown.preprocessors import Preprocessor
from markdown import Extension

CODES = {
    'PC': 'Presenting Complaint',
    'HPC': 'History of the Presenting Complaint',
    'OE': 'On Examination',
    'RS': 'Respiratory System',
    'CVS': 'Cardiovascular System',
    'IMP': 'Impression',
    'PLAN': 'Plan',
}

class MedProcessor(Preprocessor):

    LINE_REG =  re.compile(r'^(\s+)?(\w+)/(.*)$')

    def __init__(self, *args, **kwargs):
        self.structured = {}


    def run(self, lines):
        last_outer = ''
        outputs = []
        last_outer = None

        for line in lines:
            m = self.LINE_REG.match(line)
            if m:
                (spaces, title, notes) = m.groups()
                if last_outer and spaces:
                    self.structured[last_outer][title] = notes.strip()
                else:
                    self.structured[title] = {'notes': notes.strip()}

                if spaces:
                    prefix = '###'
                else:
                    prefix = '##'

                outputs.append('{} {} \n{}'.format(prefix, CODES.get(title), notes))
                if not spaces:
                    last_outer = title
            else:
                if last_outer and line.strip():
                    self.structured[last_outer]['notes'] += " " + line.strip()
                last_outer = None
                outputs.append(line)

        return outputs


    def pmh(self, line):
        data = line[4:]
        parts = data.split(',')
        self.structured['pmh'] = [p.strip() for p in parts]

        html = '<ul>'
        for p in parts:
            html += '<li>{}</li>'.format(p)
        html += '</ul>'

        return "## Previous Medical History\n{}\n\n".format(html)

class MedExtension(Extension):

    def __init__(self, *args, **kwargs):
        self.processor = MedProcessor()
        super(MedExtension, self).__init__(*args, **kwargs)

    def get_structured_data(self):
        return self.processor.structured

    def extendMarkdown(self, md, md_globals):
        md.registerExtension(self)
        md.preprocessors.add('medblock', self.processor, '_end')
