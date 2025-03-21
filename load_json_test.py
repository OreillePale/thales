import json
import pandas as pd
import matplotlib.pyplot as plt

def unpack(jjson):
    samples = jjson['samples']
    records = []
    for m in jjson['marks']:
        d = {}
        d['samples'] = samples
        for k in m:
            if k != "input":
                d[k] = m[k]
            else:
                for k2 in m[k]:
                    d[f'input.{k2}'] = m[k][k2]
        records.append(d)

    return pd.DataFrame.from_records(records)

with open('test.json') as f:
    jjson = json.load(f)

df = unpack(jjson)

plt.figure()
plt.loglog()
plt.grid(ls='--')
for name in df['name'].unique():
    sdf = df[df['name'] == name]

    plt.errorbar(sdf['input.n'], sdf['t_mean'], sdf['t_std'], fmt='o-', capsize=3, label=name)
plt.legend()
plt.xlabel('Samples')
plt.ylabel('Average time [s]')
plt.tight_layout()
plt.savefig('fib.png',dpi=300)

plt.show()

