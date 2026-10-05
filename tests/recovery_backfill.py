"""Real PostgreSQL regression: resumable, idempotent and fail-closed recovery.

Requires a disposable empty schema matching the deployed explorer and
TEST_RECOVERY_DATABASE_URL, ZEBRA_STATE_PATH, optional INDEXER_TEST_BINARY.
"""
import os,pathlib,subprocess
import tempfile,urllib.parse
root=pathlib.Path(tempfile.mkdtemp(prefix='crosslink-recovery-test-'))
env=dict(os.environ)
url=env['TEST_RECOVERY_DATABASE_URL']
assert 'test' in urllib.parse.urlparse(url).path.lower(), 'Use an isolated disposable test database'
env.update(DATABASE_URL=url,NETWORK='crosslink',ZEBRA_RPC_COOKIE_FILE='/dev/null')
assert env.get('ZEBRA_STATE_PATH'), 'ZEBRA_STATE_PATH must point at verified public node state'
binary=env.get('INDEXER_TEST_BINARY','./target/release/cipherscan-indexer-crosslink')
def sql(query):
 p=subprocess.run(['psql',url,'-v','ON_ERROR_STOP=1','-Atc',query],env=env,capture_output=True,text=True)
 assert p.returncode==0,p.stderr
 return p.stdout.strip()
def backfill(name,start,end):
 p=subprocess.run([binary,'backfill','--from',str(start),'--to',str(end)],env=env,capture_output=True,text=True)
 (root/(name+'.log')).write_text(p.stdout+p.stderr)
 return p
assert sql('SELECT count(*) FROM blocks')=='0', 'Test database must be empty'
r=backfill('test-initial-backfill',0,20)
assert r.returncode==0,(r.stdout+r.stderr)[-2500:]
fingerprint="SELECT (SELECT count(*) FROM blocks), (SELECT count(*) FROM transactions), (SELECT count(*) FROM transaction_outputs), (SELECT COALESCE(sum(balance),0) FROM addresses);"
before=sql(fingerprint)
assert sql("SELECT value FROM indexer_state WHERE key='last_indexed_height'")=='20'
r=backfill('test-identical-replay',0,20)
assert r.returncode==0,(r.stdout+r.stderr)[-2500:]
assert sql(fingerprint)==before
sql("UPDATE blocks SET hash=repeat('f',64) WHERE height=20")
r=backfill('test-changed-block',20,20)
assert r.returncode!=0 and 'explicit chain repair required' in r.stdout+r.stderr
assert sql(fingerprint)==before
assert sql("SELECT hash FROM blocks WHERE height=20")==('f'*64)
r=backfill('test-changed-parent',21,21)
assert r.returncode!=0 and 'explicit chain repair required' in r.stdout+r.stderr
assert sql(fingerprint)==before
print('PASS: 21-block backfill, live checkpoint handoff, identical replay preserves exact balances, changed hash and parent fail before writes')
