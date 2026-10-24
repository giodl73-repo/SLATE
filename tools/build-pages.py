"""Build the bounded synthetic graph task and native canonical references."""
import json
import shutil
import subprocess
from pathlib import Path
from nces_source import verify

ROOT=Path(__file__).resolve().parents[1]

def output(*args):
    return subprocess.check_output(args,cwd=ROOT,text=True,encoding='utf-8')

def main():
    verify()
    cache=ROOT/'.cache';cache.mkdir(exist_ok=True)
    refs=[]
    for mode in ['reset','outage','changed']:
        path=f'fixtures/browser-{mode}.json'
        refs.append({'input':(ROOT/path).read_text(),'canonical':output('cargo','run','--locked','--quiet','-p','slate-web','--bin','slate-browser','--',path).strip()})
    (cache/'native-results.json').write_text(json.dumps(refs),encoding='utf-8')
    source_refs=[]
    for name in ['us','california','tennessee','guam']:
        path=f'fixtures/nces-query-{name}.json'
        source_refs.append({'input':(ROOT/path).read_text(encoding='utf-8'),'canonical':output('cargo','run','--locked','--quiet','-p','slate-web','--bin','slate-browser','--',path).strip()})
    (cache/'nces-native-results.json').write_text(json.dumps(source_refs),encoding='utf-8')
    subprocess.run(['cargo','build','--locked','--release','--target','wasm32-unknown-unknown','-p','slate-web','--lib'],cwd=ROOT,check=True)
    metadata=json.loads(output('cargo','metadata','--no-deps','--format-version','1'))
    dest=ROOT/'dist/SLATE';(dest/'pkg').mkdir(parents=True,exist_ok=True)
    shutil.copy2(Path(metadata['target_directory'])/'wasm32-unknown-unknown/release/slate_web.wasm',dest/'pkg/slate_web.wasm')
    for file in (ROOT/'web').iterdir():
        if file.is_file():shutil.copy2(file,dest/file.name)
    for mode,target in [('reset','example.json'),('outage','outage.json')]:shutil.copy2(ROOT/f'fixtures/browser-{mode}.json',dest/target)
    for name in ['LICENSE','THIRD-PARTY-LICENSES.txt']:shutil.copy2(ROOT/name,dest/name)
    (dest/'sources').mkdir(exist_ok=True)
    custody=json.loads((ROOT/'data/nces/source-custody.json').read_text(encoding='utf-8'))
    for source in custody['sources']:shutil.copy2(ROOT/source['path'],dest/source['published'])
    shutil.copy2(ROOT/'data/nces/source-custody.json',dest/'source-custody.json')
    shutil.copy2(ROOT/'fixtures/nces/summary.json',dest/'nces-summary.json')
    shutil.copy2(ROOT/'data/nces/README.md',dest/'SOURCE-NOTICE.txt')
    provenance={'commit':output('git','rev-parse','HEAD').strip(),'dirty':bool(output('git','status','--porcelain').strip()),'rust':output('rustc','--version').strip(),'source':'https://github.com/giodl73-repo/SLATE','tasks':['frozen NCES state aggregates','synthetic local Cedar graph ratings'],'source_capture':custody['captured_at']}
    (dest/'provenance.json').write_text(json.dumps(provenance,indent=2),encoding='utf-8');(dest/'.nojekyll').touch()
    size=sum(path.stat().st_size for path in dest.rglob('*') if path.is_file())
    if size>=5_000_000:raise ValueError('Artifact exceeds 5 MB')
    print(f'SLATE Pages: {size:,} bytes')

if __name__=='__main__':main()
