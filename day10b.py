input = """71
183
111
89
92
142
25
101
52
86
18
22
70
2
135
163
34
143
153
35
144
24
23
94
100
102
17
57
76
182
134
38
7
103
66
31
11
121
77
113
128
82
99
148
137
41
32
48
131
60
127
138
73
28
10
84
180
63
125
53
176
165
114
145
152
72
107
167
59
164
78
126
118
136
83
79
58
14
106
69
51
39
157
42
177
173
93
141
3
33
13
19
45
154
95
170
54
181
6
151
1
112
96
115
85
108
166
160
40
122
12"""

input = [int(i.strip()) for i in input.split()]


all_adapters = sorted(input)
highest_adapter = max(all_adapters)
phone_power_port = highest_adapter + 3

all_adapters = [0] + all_adapters + [phone_power_port]

resolutions = {}

for adapter in all_adapters:
    next_adapters = [adapter + n for n in [1, 2, 3] if adapter + n in all_adapters]
    resolutions[adapter] = next_adapters


resolutions[phone_power_port] = 1
print(resolutions)

def find_num_paths_from(n):
  resolution = resolutions[n]
  if isinstance(resolution, int):
      return resolution

  r = sum(find_num_paths_from(next) for next in resolution)
  resolutions[n] = r
  return r


print(find_num_paths_from(0))
