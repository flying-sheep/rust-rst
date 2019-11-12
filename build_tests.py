import sys
import json
from ast import literal_eval
from urllib.request import urlopen

out = sys.argv[1]
url_base = 'https://sourceforge.net/p/docutils/code/HEAD/tree/trunk/docutils'

with urlopen(f'{url_base}/test/test_writers/test_html5_polyglot_parts.py?format=raw') as con:
	code = con.read().decode()

code = code[code.find('totest ='):code.find('if __name__')]
exec(code)
with open(out, 'w') as f:
	t = 0
	for k, (opts, tests) in totest.items():
		for rst, result_code in tests:
			result = literal_eval(result_code)['fragment']
			rst, result = (r.replace('"', r'\"') for r in (rst, result))
			f.write(f'''\
#[test]
fn test_{t:02}() {{
	check_renders_to("{rst}", "{result.strip()}");
}}
''')
			t += 1
			
