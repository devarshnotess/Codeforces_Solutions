<h2><a href="https://codeforces.com/contest/2074/problem/B" target="_blank" rel="noopener noreferrer">2074B — The Third Side</a></h2>

| | |
|---|---|
| **Difficulty** | 800 |
| **Language** | Rust 2021 |
| **Verdict** | ✅ Accepted |
| **Problem Link** | [Codeforces 2074B](https://codeforces.com/contest/2074/problem/B) |

## Topics
`geometry` `greedy` `math`

---

## Problem Statement

<div class="header"><div class="title">B. The Third Side</div><div class="time-limit"><div class="property-title">time limit per test</div>2 seconds</div><div class="memory-limit"><div class="property-title">memory limit per test</div>256 megabytes</div><div class="input-file input-standard"><div class="property-title">input</div>standard input</div><div class="output-file output-standard"><div class="property-title">output</div>standard output</div></div><div><p>The pink soldiers have given you a sequence $$$a$$$ consisting of $$$n$$$ positive integers.</p><p>You must repeatedly perform the following operation <span class="tex-font-style-bf">until there is only $$$1$$$ element left</span>.</p><ul> <li> Choose two <span class="tex-font-style-bf">distinct</span> indices $$$i$$$ and $$$j$$$. </li><li> Then, choose a positive integer value $$$x$$$ such that there exists a <span class="tex-font-style-bf">non-degenerate triangle</span>$$$^{\text{∗}}$$$ with side lengths $$$a_i$$$, $$$a_j$$$, and $$$x$$$. </li><li> Finally, remove two elements $$$a_i$$$ and $$$a_j$$$, and append $$$x$$$ to the end of $$$a$$$. </li></ul><p>Please find the maximum possible value of the only last element in the sequence $$$a$$$.</p><div class="statement-footnote"><p>$$$^{\text{∗}}$$$A triangle with side lengths $$$a$$$, $$$b$$$, $$$c$$$ is non-degenerate when $$$a+b  \gt  c$$$, $$$a+c  \gt  b$$$, $$$b+c  \gt  a$$$.</p></div></div><div class="input-specification"><div class="section-title">Input</div><p>Each test contains multiple test cases. The first line contains the number of test cases $$$t$$$ ($$$1 \le t \le 10^4$$$). The description of the test cases follows. </p><p>The first line of each test case contains a single integer $$$n$$$ ($$$1 \le n \le 2 \cdot 10^5$$$).</p><p>The second line of each test case contains $$$n$$$ integers $$$a_1,a_2,\ldots,a_n$$$ ($$$1 \le a_i \le 1000$$$) — the elements of the sequence $$$a$$$.</p><p>It is guaranteed that the sum of $$$n$$$ over all test cases does not exceed $$$2 \cdot 10^5$$$.</p></div><div class="output-specification"><div class="section-title">Output</div><p>For each test case, output the maximum possible value of the only last element on a separate line.</p></div><div class="sample-tests"><div class="section-title">Example</div><div class="sample-test"><div class="input"><div class="title">Input<div title="Copy" data-clipboard-target="#id0005301065598900778" id="id007273409109309961" class="input-output-copier">Copy</div></div><pre data-content="4
1
10
3
998 244 353
5
1 2 3 4 5
9
9 9 8 2 4 4 3 5 3
" id="id0005301065598900778"><div class="test-example-line test-example-line-even test-example-line-0">4</div><div class="test-example-line test-example-line-odd test-example-line-1">1</div><div class="test-example-line test-example-line-odd test-example-line-1">10</div><div class="test-example-line test-example-line-even test-example-line-2">3</div><div class="test-example-line test-example-line-even test-example-line-2">998 244 353</div><div class="test-example-line test-example-line-odd test-example-line-3">5</div><div class="test-example-line test-example-line-odd test-example-line-3">1 2 3 4 5</div><div class="test-example-line test-example-line-even test-example-line-4">9</div><div class="test-example-line test-example-line-even test-example-line-4">9 9 8 2 4 4 3 5 3</div></pre></div><div class="output"><div class="title">Output<div title="Copy" data-clipboard-target="#id005050204763200518" id="id00997394279368003" class="input-output-copier">Copy</div></div><pre id="id005050204763200518">10
1593
11
39
</pre></div></div></div><div class="note"><div class="section-title">Note</div><p>On the first test case, there is already only one element. The value of the only last element is $$$10$$$.</p><p>On the second test case, $$$a$$$ is initially $$$[998,244,353]$$$. The following series of operations is valid:</p><ol> <li> Erase $$$a_2=244$$$ and $$$a_3=353$$$, and append $$$596$$$ to the end of $$$a$$$. $$$a$$$ is now $$$[998,596]$$$. </li><li> Erase $$$a_1=998$$$ and $$$a_2=596$$$, and append $$$1593$$$ to the end of $$$a$$$. $$$a$$$ is now $$$[1593]$$$. </li></ol><p>It can be shown that the only last element cannot be greater than $$$1593$$$. Therefore, the answer is $$$1593$$$.</p></div>