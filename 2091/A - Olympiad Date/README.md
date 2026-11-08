<h2><a href="https://codeforces.com/contest/2091/problem/A" target="_blank" rel="noopener noreferrer">2091A — Olympiad Date</a></h2>

| | |
|---|---|
| **Difficulty** | 800 |
| **Language** | Rust 2021 |
| **Verdict** | ✅ Accepted |
| **Problem Link** | [Codeforces 2091A](https://codeforces.com/contest/2091/problem/A) |

## Topics
`greedy` `strings`

---

## Problem Statement

<div class="header"><div class="title">A. Olympiad Date</div><div class="time-limit"><div class="property-title">time limit per test</div>1 second</div><div class="memory-limit"><div class="property-title">memory limit per test</div>256 megabytes</div><div class="input-file input-standard"><div class="property-title">input</div>standard input</div><div class="output-file output-standard"><div class="property-title">output</div>standard output</div></div><div><p>The final of the first Olympiad by IT Campus "NEIMARK" is scheduled for March 1, 2025. A nameless intern was tasked with forming the date of the Olympiad using digits — 01.03.2025.</p><p>To accomplish this, the intern took a large bag of digits and began drawing them one by one. In total, he drew $$$n$$$ digits — the digit $$$a_i$$$ was drawn in the $$$i$$$-th turn.</p><p>You suspect that the intern did extra work. Determine at which step the intern could have first assembled the digits to form the date of the Olympiad (the separating dots can be ignored), or report that it is impossible to form this date from the drawn digits. Note that leading zeros <span class="tex-font-style-bf">must be displayed</span>.</p></div><div class="input-specification"><div class="section-title">Input</div><p>Each test contains multiple test cases. The first line contains the number of test cases $$$t$$$ ($$$1 \leq t \leq 10^4$$$). The description of the test cases follows.</p><p>The first line of each test case contains a single integer $$$n$$$ ($$$1 \le n \le 20$$$).</p><p>The second line of each test case contains $$$n$$$ integers $$$a_i$$$ ($$$0 \leq a_i \leq 9$$$) — the numbers that the intern pulled out in chronological order.</p></div><div class="output-specification"><div class="section-title">Output</div><p>For each test case, output the minimum number of digits that the intern could pull out. If all the digits cannot be used to make a date, output the number $$$0$$$.</p></div><div class="sample-tests"><div class="section-title">Example</div><div class="sample-test"><div class="input"><div class="title">Input<div title="Copy" data-clipboard-target="#id0011409443276232278" id="id0013947639841252957" class="input-output-copier">Copy</div></div><pre id="id0011409443276232278"><div class="test-example-line test-example-line-even test-example-line-0">4</div><div class="test-example-line test-example-line-odd test-example-line-1">10</div><div class="test-example-line test-example-line-odd test-example-line-1">2 0 1 2 3 2 5 0 0 1</div><div class="test-example-line test-example-line-even test-example-line-2">8</div><div class="test-example-line test-example-line-even test-example-line-2">2 0 1 2 3 2 5 0</div><div class="test-example-line test-example-line-odd test-example-line-3">8</div><div class="test-example-line test-example-line-odd test-example-line-3">2 0 1 0 3 2 5 0</div><div class="test-example-line test-example-line-even test-example-line-4">16</div><div class="test-example-line test-example-line-even test-example-line-4">2 3 1 2 3 0 1 9 2 1 0 3 5 4 0 3</div></pre></div><div class="output"><div class="title">Output<div title="Copy" data-clipboard-target="#id004890186274096784" id="id004848197963682368" class="input-output-copier">Copy</div></div><pre id="id004890186274096784">9
0
8
15
</pre></div></div></div>