
import markdown
from med.medmarkdown import MedExtension

def test_simple():
    m = MedExtension()
    html = markdown.markdown("""
        OBS/Observation
    """, extensions=[m])

    s = m.get_structured_data()
    assert s['OBS']['notes'] == 'Observation', s
