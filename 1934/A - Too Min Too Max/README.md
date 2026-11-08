<h2><a href="https://codeforces.com/contest/1934/problem/A" target="_blank" rel="noopener noreferrer">1934A — Too Min Too Max</a></h2>

| | |
|---|---|
| **Difficulty** | 800 |
| **Language** | C++20 (GCC 13-64) |
| **Verdict** | ✅ Accepted |
| **Problem Link** | [Codeforces 1934A](https://codeforces.com/contest/1934/problem/A) |

## Topics
`greedy` `math`

---

## Problem Statement

<div class="header"><div class="title">A. Too Min Too Max</div><div class="time-limit"><div class="property-title">time limit per test</div>1 second</div><div class="memory-limit"><div class="property-title">memory limit per test</div>256 megabytes</div><div class="input-file input-standard"><div class="property-title">input</div>standard input</div><div class="output-file output-standard"><div class="property-title">output</div>standard output</div></div><div><p>Given an array $$$a$$$ of $$$n$$$ elements, find the maximum value of the expression:</p><p>$$$$$$|a_i - a_j| + |a_j - a_k| + |a_k - a_l| + |a_l - a_i|$$$$$$</p><p>where $$$i$$$, $$$j$$$, $$$k$$$, and $$$l$$$ are four <span class="tex-font-style-bf">distinct</span> indices of the array $$$a$$$, with $$$1 \le i, j, k, l \le n$$$.</p><p>Here $$$|x|$$$ denotes the absolute value of $$$x$$$.</p></div><div class="input-specification"><div class="section-title">Input</div><p>The first line contains one integer $$$t$$$ ($$$1 \le t \le 500$$$) — the number of test cases. The description of the test cases follows.</p><p>The first line of each test case contains a single integer $$$n$$$ ($$$4 \le n \le 100$$$) — the length of the given array.</p><p>The second line of each test case contains $$$n$$$ integers $$$a_1, a_2, \ldots, a_n$$$ ($$$-10^6 \le a_i \le 10^6$$$).</p></div><div class="output-specification"><div class="section-title">Output</div><p>For each test case, print a single integer — the maximum value.</p></div><div class="sample-tests"><div class="section-title">Example</div><div class="sample-test"><div class="input"><div class="title">Input<div title="Copy" data-clipboard-target="#id006350392983347201" id="id006852270444120323" class="input-output-copier">Copy</div></div><pre data-content="5
4
1 1 1 1
5
1 1 2 2 3
8
5 1 3 2 -3 -1 10 3
4
3 3 1 1
4
1 2 2 -1
" id="id006350392983347201"><div class="test-example-line test-example-line-even test-example-line-0">5</div><div class="test-example-line test-example-line-odd test-example-line-1">4</div><div class="test-example-line test-example-line-odd test-example-line-1">1 1 1 1</div><div class="test-example-line test-example-line-even test-example-line-2">5</div><div class="test-example-line test-example-line-even test-example-line-2">1 1 2 2 3</div><div class="test-example-line test-example-line-odd test-example-line-3">8</div><div class="test-example-line test-example-line-odd test-example-line-3">5 1 3 2 -3 -1 10 3</div><div class="test-example-line test-example-line-even test-example-line-4">4</div><div class="test-example-line test-example-line-even test-example-line-4">3 3 1 1</div><div class="test-example-line test-example-line-odd test-example-line-5">4</div><div class="test-example-line test-example-line-odd test-example-line-5">1 2 2 -1</div></pre></div><div class="output"><div class="title">Output<div title="Copy" data-clipboard-target="#id002884625473722636" id="id006775245670254039" class="input-output-copier">Copy</div></div><pre id="id002884625473722636">0
6
38
8
8
</pre></div></div></div><div class="note"><div class="section-title">Note</div><p>In the first test case, for any selection of $$$i$$$, $$$j$$$, $$$k$$$, $$$l$$$, the answer will be $$$0$$$. For example, $$$|a_1 - a_2| + |a_2 - a_3| + |a_3 - a_4| + |a_4 - a_1| = |1 - 1| + |1 - 1| + |1 - 1| + |1 - 1| = 0 + 0 + 0 + 0 = 0$$$.</p><p>In the second test case, for $$$i = 1$$$, $$$j = 3$$$, $$$k = 2$$$, and $$$l = 5$$$, the answer will be $$$6$$$. $$$|a_1 - a_3| + |a_3 - a_2| + |a_2 - a_5| + |a_5 - a_1| = |1 - 2| + |2 - 1| + |1 - 3| + |3 - 1| = 1 + 1 + 2 + 2 = 6$$$.</p></div>