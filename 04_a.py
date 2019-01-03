import re
import datetime
import numpy as np
from collections import defaultdict

log_pattern = re.compile(r'\[(.+)\] (.+)')
shift_pattern = re.compile(r'Guard #(\d+) begins shift')

records = []
count = 0
for line in open('04_input.txt'):
    stamp, message = log_pattern.match(line).groups()
    stamp = datetime.datetime.strptime(stamp, '%Y-%m-%d %H:%M')
    if 'Guard' in message:
        msg = shift_pattern.match(message).group(1)
        count += 1
    elif 'falls' in message:
        msg = 1
    else:
        msg = 0
    records.append((stamp, msg))

records.sort(key=lambda x: x[0])

data = defaultdict(list)
guard = None
start = None
end = None

for stamp, msg in records:
    if isinstance(msg, str):
        guard = int(msg)
        continue
    if msg == 1:
        start = stamp.minute
    if msg == 0:
        end = stamp.minute
    if start is not None and end is not None:
        minutes = data[format(stamp, '%m-%d'), guard] or [0] * 60
        minutes[start:end] = [1] * (end - start)
        data[format(stamp, '%m-%d'), guard] = minutes
        start = end = None


guards = defaultdict(int)
for (_, id), minutes in data.items():
    guards[id] += sum(minutes)


guard = max(guards.items(), key=lambda x: x[1])[0]

minutes = defaultdict(int)
for (_, id), values in data.items():
    if id != guard:
        continue
    for i, m in enumerate(values):
        minutes[i] += m

minute = max(minutes.items(), key=lambda x: x[1])[0]

print('Result:', guard * minute)