
import markdown
from med.medmarkdown import MedExtension

def test_simple():
    html = markdown.markdown("""
Line one
!pmh this patient has lots of history, not sure what else to write.
Line two
    """, extensions=[MedExtension()])
    print html
