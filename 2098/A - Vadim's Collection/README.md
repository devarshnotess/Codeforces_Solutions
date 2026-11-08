<h2><a href="https://codeforces.com/contest/2098/problem/A" target="_blank" rel="noopener noreferrer">2098A — Vadim's Collection</a></h2>

| | |
|---|---|
| **Difficulty** | 800 |
| **Language** | Rust 2021 |
| **Verdict** | ✅ Accepted |
| **Problem Link** | [Codeforces 2098A](https://codeforces.com/contest/2098/problem/A) |

## Topics
`brute force` `greedy`

---

## Problem Statement

<div class="header"><div class="title">A. Vadim's Collection</div><div class="time-limit"><div class="property-title">time limit per test</div>1 second</div><div class="memory-limit"><div class="property-title">memory limit per test</div>256 megabytes</div><div class="input-file input-standard"><div class="property-title">input</div>standard input</div><div class="output-file output-standard"><div class="property-title">output</div>standard output</div></div><div><p>We call a phone number a <span class="tex-font-style-it">beautiful</span> if it is a string of $$$10$$$ digits, where the $$$i$$$-th digit from the left is at least $$$10 - i$$$. That is, the first digit must be at least $$$9$$$, the second at least $$$8$$$, $$$\ldots$$$, with the last digit being at least $$$0$$$.</p><p>For example, <span class="tex-font-style-tt">9988776655</span> is a beautiful phone number, while <span class="tex-font-style-tt">9099999999</span> is not, since the second digit, which is $$$0$$$, is less than $$$8$$$.</p><p>Vadim has a <span class="tex-font-style-bf">beautiful</span> phone number. He wants to rearrange its digits in such a way that the result is the <span class="tex-font-style-bf">smallest possible beautiful</span> phone number. Help Vadim solve this problem.</p><p>Please note that the phone numbers are compared as integers.</p></div><div class="input-specification"><div class="section-title">Input</div><p>Each test contains multiple test cases. The first line contains the number of test cases $$$t$$$ ($$$1 \le t \le 10^4$$$). The description of the test cases follows. </p><p>The only line of each test case contains a single string $$$s$$$ of length $$$10$$$, consisting of digits. It is guaranteed that $$$s$$$ is a <span class="tex-font-style-bf">beautiful</span> phone number.</p></div><div class="output-specification"><div class="section-title">Output</div><p>For each test case, output a single string of length $$$10$$$ — the smallest possible beautiful phone number that Vadim can obtain.</p></div><div class="sample-tests"><div class="section-title">Example</div><div class="sample-test"><div class="input"><div class="title">Input<div title="Copy" data-clipboard-target="#id000998731727505826" id="id005922410639393516" class="input-output-copier">Copy</div></div><pre id="id000998731727505826"><div class="test-example-line test-example-line-even test-example-line-0">4</div><div class="test-example-line test-example-line-odd test-example-line-1">9999999999</div><div class="test-example-line test-example-line-even test-example-line-2">9988776655</div><div class="test-example-line test-example-line-odd test-example-line-3">9988776650</div><div class="test-example-line test-example-line-even test-example-line-4">9899999999</div></pre></div><div class="output"><div class="title">Output<div title="Copy" data-clipboard-target="#id006541565176361256" id="id006809915289455329" class="input-output-copier">Copy</div></div><pre id="id006541565176361256">9999999999
9876556789
9876567890
9899999999
</pre></div></div></div><div class="note"><div class="section-title">Note</div><p>In the first test case, for the first phone number <span class="tex-font-style-tt">9999999999</span>, regardless of the rearrangement of digits, the same phone number is obtained.</p><p>In the second test case, for the phone number <span class="tex-font-style-tt">9988776655</span>, it can be proven that <span class="tex-font-style-tt">9876556789</span> is the smallest phone number that can be obtained by rearranging the digits.</p></div>