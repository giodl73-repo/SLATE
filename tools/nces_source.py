"""Re-extract frozen official NCES Table 2 HTML; no live fetch or hand transcription."""
from html.parser import HTMLParser
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[1]
FIELDS = ['operating_schools', 'operating_districts', 'student_membership', 'teacher_fte', 'pupil_teacher_ratio']
OTHER = {'DoDEA', 'Bureau of Indian Education', 'American Samoa', 'Guam', 'Northern Marianas', 'Puerto Rico', 'U.S. Virgin Islands'}


class Table(HTMLParser):
    def __init__(self):
        super().__init__()
        self.row = None
        self.cell = None
        self.sup = False
        self.rows = []

    def handle_starttag(self, tag, attrs):
        if tag == 'tr': self.row = []
        if tag in ('td', 'th'): self.cell = []
        if tag == 'sup': self.sup = True

    def handle_data(self, data):
        if self.cell is not None and not self.sup: self.cell.append(data)

    def handle_endtag(self, tag):
        if tag == 'sup': self.sup = False
        if tag in ('td', 'th') and self.cell is not None:
            if self.row is not None: self.row.append(' '.join(''.join(self.cell).split()))
            self.cell = None
        if tag == 'tr' and self.row:
            self.rows.append(self.row)
            self.row = None


def extract():
    manifest = json.loads((ROOT/'data/nces/source-custody.json').read_text(encoding='utf-8'))
    releases = []
    for source in manifest['sources']:
        raw = (ROOT/source['path']).read_bytes()
        if hashlib.sha256(raw).hexdigest() != source['sha256']: raise ValueError('Source hash mismatch: '+source['path'])
        # HTML's ISO-8859-1 label is interpreted with the Windows-1252 mapping.
        table = Table(); table.feed(raw.decode('cp1252'))
        rows = []
        for cells in table.rows:
            if len(cells) != 8 or not cells[1] or cells[0] == 'State or jurisdiction': continue
            name = cells[0]
            if not name: continue
            values = [cells[index] for index in (1,2,4,5,7)]
            if not all(value in ('—','‡') or value.replace(',','').replace('.','').isdigit() for value in values):
                raise ValueError('Unexpected source row: '+repr(cells))
            group = 'us_total' if name == 'United States' else 'other_jurisdiction' if name in OTHER else 'state_dc'
            rows.append({'jurisdiction':name, 'group':group, **dict(zip(FIELDS,values))})
        if len(rows) != 59 or len({row['jurisdiction'] for row in rows}) != 59: raise ValueError('Expected 59 distinct published rows')
        if sum(row['group']=='state_dc' for row in rows) != 51: raise ValueError('Expected 50 states plus DC')
        notes = [cells[0] for cells in table.rows if len(cells)==1 and cells[0]]
        if not any('Final Version 1a' in note for note in notes): raise ValueError('Source version changed')
        releases.append({'school_year':source['school_year'],'source_id':source['source_id'],'url':source['url'],'sha256':source['sha256'],'notes':notes,'rows':rows})
    if [release['school_year'] for release in releases] != ['2023-24','2024-25']: raise ValueError('Unexpected year pair')
    if {row['jurisdiction'] for row in releases[0]['rows']} != {row['jurisdiction'] for row in releases[1]['rows']}: raise ValueError('Jurisdiction sets differ')
    return {'schema':'slate.nces-table2.v1','captured_at':manifest['captured_at'],'releases':releases}


def verify():
    actual = json.loads((ROOT/'fixtures/nces/summary.json').read_text(encoding='utf-8'))
    if actual != extract(): raise ValueError('Normalized data differ from frozen official originals')


if __name__ == '__main__':
    import argparse
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--write',action='store_true');args=parser.parse_args()
    if args.write:
        path=ROOT/'fixtures/nces/summary.json';path.parent.mkdir(parents=True,exist_ok=True)
        path.write_text(json.dumps(extract(),indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    verify()
    print('Frozen NCES hashes and all 118 normalized source rows verified')
