<h2><a href="https://codeforces.com/contest/1101/problem/C" target="_blank" rel="noopener noreferrer">1101C — Division and Union</a></h2>

| | |
|---|---|
| **Difficulty** | 1500 |
| **Language** | C++20 (GCC 13-64) |
| **Verdict** | ✅ Accepted |
| **Problem Link** | [Codeforces 1101C](https://codeforces.com/contest/1101/problem/C) |

## Topics
`sortings`

---

## Problem Statement

<div class="header"><div class="title">C. Division and Union</div><div class="time-limit"><div class="property-title">time limit per test</div>2 seconds</div><div class="memory-limit"><div class="property-title">memory limit per test</div>256 megabytes</div><div class="input-file input-standard"><div class="property-title">input</div>standard input</div><div class="output-file output-standard"><div class="property-title">output</div>standard output</div></div><div><p>There are $$$n$$$ segments $$$[l_i, r_i]$$$ for $$$1 \le i \le n$$$. You should divide all segments into two <span class="tex-font-style-it">non-empty</span> groups in such way that there is no pair of segments from different groups which have at least one common point, or say that it's impossible to do it. Each segment should belong to exactly one group.</p><p>To optimize testing process you will be given multitest.</p></div><div class="input-specification"><div class="section-title">Input</div><p>The first line contains one integer $$$T$$$ ($$$1 \le T \le 50000$$$) — the number of queries. Each query contains description of the set of segments. Queries are independent.</p><p>First line of each query contains single integer $$$n$$$ ($$$2 \le n \le 10^5$$$) — number of segments. It is guaranteed that $$$\sum{n}$$$ over all queries does not exceed $$$10^5$$$.</p><p>The next $$$n$$$ lines contains two integers $$$l_i$$$, $$$r_i$$$ per line ($$$1 \le l_i \le r_i \le 2 \cdot 10^5$$$) — the $$$i$$$-th segment.</p></div><div class="output-specification"><div class="section-title">Output</div><p>For each query print $$$n$$$ integers $$$t_1, t_2, \dots, t_n$$$ ($$$t_i \in \{1, 2\}$$$) — for each segment (in the same order as in the input) $$$t_i$$$ equals $$$1$$$ if the $$$i$$$-th segment will belongs to the first group and $$$2$$$ otherwise.</p><p>If there are multiple answers, you can print any of them. If there is no answer, print $$$-1$$$.</p></div><div class="sample-tests"><div class="section-title">Example</div><div class="sample-test"><div class="input"><div class="title">Input<div title="Copy" data-clipboard-target="#id0030002899608308475" id="id003127944429280285" class="input-output-copier">Copy</div></div><pre id="id0030002899608308475">3
2
5 5
2 3
3
3 5
2 3
2 3
3
3 3
4 4
5 5
</pre></div><div class="output"><div class="title">Output<div title="Copy" data-clipboard-target="#id003761296728191573" id="id0009605895214288152" class="input-output-copier">Copy</div></div><pre id="id003761296728191573">2 1 
-1
1 1 2 
</pre></div></div></div><div class="note"><div class="section-title">Note</div><p>In the first query the first and the second segments should be in different groups, but exact numbers don't matter.</p><p>In the second query the third segment intersects with the first and the second segments, so they should be in the same group, but then the other group becomes empty, so answer is $$$-1$$$.</p><p>In the third query we can distribute segments in any way that makes groups non-empty, so any answer of $$$6$$$ possible is correct.</p></div>