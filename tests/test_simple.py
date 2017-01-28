
import markdown
from med.medmarkdown import MedExtension

def test_simple():
    m = MedExtension()
    markdown.markdown("""
        OBS/Observation
    """, extensions=[m])

    s = m.get_structured_data()
    assert s['OBS']['notes'] == 'Observation', s

def test_less_simple():
    contents = open('test_data/less_simple.txt', 'r').read()
    m = MedExtension()
    html = markdown.markdown(contents, extensions=[m])
    s = m.get_structured_data()
    assert s['PC']['notes'] == 'Mobility issues across multiple lines until we find a newline', s
    print html

