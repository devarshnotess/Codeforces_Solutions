<h2><a href="https://codeforces.com/contest/2062/problem/A" target="_blank" rel="noopener noreferrer">2062A — String</a></h2>

| | |
|---|---|
| **Difficulty** | 800 |
| **Language** | Node.js |
| **Verdict** | ✅ Accepted |
| **Problem Link** | [Codeforces 2062A](https://codeforces.com/contest/2062/problem/A) |

## Topics
`constructive algorithms` `greedy` `math` `strings`

---

## Problem Statement

<div class="header"><div class="title">A. String</div><div class="time-limit"><div class="property-title">time limit per test</div>1 second</div><div class="memory-limit"><div class="property-title">memory limit per test</div>512 megabytes</div><div class="input-file input-standard"><div class="property-title">input</div>standard input</div><div class="output-file output-standard"><div class="property-title">output</div>standard output</div></div><div><p>You are given a string $$$s$$$ of length $$$n$$$ consisting of $$$\mathtt{0}$$$ and/or $$$\mathtt{1}$$$. In one operation, you can select a non-empty subsequence $$$t$$$ from $$$s$$$ such that any two adjacent characters in $$$t$$$ are different. Then, you flip each character of $$$t$$$ ($$$\mathtt{0}$$$ becomes $$$\mathtt{1}$$$ and $$$\mathtt{1}$$$ becomes $$$\mathtt{0}$$$). For example, if $$$s=\mathtt{\underline{0}0\underline{101}}$$$ and $$$t=s_1s_3s_4s_5=\mathtt{0101}$$$, after the operation, $$$s$$$ becomes $$$\mathtt{\underline{1}0\underline{010}}$$$.</p><p>Calculate the minimum number of operations required to change all characters in $$$s$$$ to $$$\mathtt{0}$$$.</p><p>Recall that for a string $$$s = s_1s_2\ldots s_n$$$, any string $$$t=s_{i_1}s_{i_2}\ldots s_{i_k}$$$ ($$$k\ge 1$$$) where $$$1\leq i_1  \lt  i_2  \lt  \ldots  \lt i_k\leq n$$$ is a subsequence of $$$s$$$.</p></div><div class="input-specification"><div class="section-title">Input</div><p>The first line of input contains a single integer $$$t$$$ ($$$1 \leq t \leq 10^4$$$) — the number of input test cases.</p><p>The only line of each test case contains the string $$$s$$$ ($$$1\le |s|\le 50$$$), where $$$|s|$$$ represents the length of $$$s$$$.</p></div><div class="output-specification"><div class="section-title">Output</div><p>For each test case, output the minimum number of operations required to change all characters in $$$s$$$ to $$$\mathtt{0}$$$.</p></div><div class="sample-tests"><div class="section-title">Example</div><div class="sample-test"><div class="input"><div class="title">Input<div title="Copy" data-clipboard-target="#id005232721891374538" id="id0008228398955600857" class="input-output-copier">Copy</div></div><pre id="id005232721891374538"><div class="test-example-line test-example-line-even test-example-line-0">5</div><div class="test-example-line test-example-line-odd test-example-line-1">1</div><div class="test-example-line test-example-line-even test-example-line-2">000</div><div class="test-example-line test-example-line-odd test-example-line-3">1001</div><div class="test-example-line test-example-line-even test-example-line-4">10101</div><div class="test-example-line test-example-line-odd test-example-line-5">01100101011101</div></pre></div><div class="output"><div class="title">Output<div title="Copy" data-clipboard-target="#id009510965192518078" id="id006424917842798946" class="input-output-copier">Copy</div></div><pre id="id009510965192518078">1
0
2
3
8
</pre></div></div></div><div class="note"><div class="section-title">Note</div><p>In the first test case, you can flip $$$s_1$$$. Then $$$s$$$ becomes $$$\mathtt{0}$$$, so the answer is $$$1$$$.</p><p>In the fourth test case, you can perform the following three operations in order:</p><ol><li> Flip $$$s_1s_2s_3s_4s_5$$$. Then $$$s$$$ becomes $$$\mathtt{\underline{01010}}$$$.</li><li> Flip $$$s_2s_3s_4$$$. Then $$$s$$$ becomes $$$\mathtt{0\underline{010}0}$$$.</li><li> Flip $$$s_3$$$. Then $$$s$$$ becomes $$$\mathtt{00\underline{0}00}$$$.</li></ol><p>It can be shown that you can not change all characters in $$$s$$$ to $$$\mathtt{0}$$$ in less than three operations, so the answer is $$$3$$$.</p></div>