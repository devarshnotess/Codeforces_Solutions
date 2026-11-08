<h2><a href="https://codeforces.com/contest/2093/problem/A" target="_blank" rel="noopener noreferrer">2093A — Ideal Generator</a></h2>

| | |
|---|---|
| **Difficulty** | 800 |
| **Language** | Rust 2021 |
| **Verdict** | ✅ Accepted |
| **Problem Link** | [Codeforces 2093A](https://codeforces.com/contest/2093/problem/A) |

## Topics
`math`

---

## Problem Statement

<div class="header"><div class="title">A. Ideal Generator</div><div class="time-limit"><div class="property-title">time limit per test</div>1 second</div><div class="memory-limit"><div class="property-title">memory limit per test</div>256 megabytes</div><div class="input-file input-standard"><div class="property-title">input</div>standard input</div><div class="output-file output-standard"><div class="property-title">output</div>standard output</div></div><div><p>We call an array $$$a$$$, consisting of $$$k$$$ positive integers, palindromic if $$$[a_1, a_2, \dots, a_k] = [a_k, a_{k-1}, \dots, a_1]$$$. For example, the arrays $$$[1, 2, 1]$$$ and $$$[5, 1, 1, 5]$$$ are palindromic, while the arrays $$$[1, 2, 3]$$$ and $$$[21, 12]$$$ are not.</p><p>We call a number $$$k$$$ an ideal generator if any integer $$$n$$$ ($$$n \ge k$$$) can be represented as the sum of the elements of a palindromic array of length exactly $$$k$$$. Each element of the array must be greater than $$$0$$$.</p><p>For example, the number $$$1$$$ is an ideal generator because any natural number $$$n$$$ can be generated using the array $$$[n]$$$. However, the number $$$2$$$ is not an ideal generator — there is no palindromic array of length $$$2$$$ that sums to $$$3$$$.</p><p>Determine whether the given number $$$k$$$ is an ideal generator.</p></div><div class="input-specification"><div class="section-title">Input</div><p>The first line of the input contains one integer $$$t$$$ ($$$1 \le t \le 1000$$$) — the number of test cases.</p><p>The first and only line of each test case contains one integer $$$k$$$ ($$$1 \le k \le 1000$$$).</p></div><div class="output-specification"><div class="section-title">Output</div><p>For each number $$$k$$$, you need to output the word <span class="tex-font-style-tt">"YES"</span> if it is an ideal generator, or <span class="tex-font-style-tt">"NO"</span> otherwise.</p><p>You may output <span class="tex-font-style-tt">"Yes"</span> and <span class="tex-font-style-tt">"No"</span> in any case (for example, the strings <span class="tex-font-style-tt">"yES"</span>, <span class="tex-font-style-tt">"yes"</span>, and <span class="tex-font-style-tt">"Yes"</span> will be recognized as a positive answer).</p></div><div class="sample-tests"><div class="section-title">Example</div><div class="sample-test"><div class="input"><div class="title">Input<div title="Copy" data-clipboard-target="#id00891078638901241" id="id008867829501440435" class="input-output-copier">Copy</div></div><pre id="id00891078638901241"><div class="test-example-line test-example-line-even test-example-line-0">5</div><div class="test-example-line test-example-line-odd test-example-line-1">1</div><div class="test-example-line test-example-line-even test-example-line-2">2</div><div class="test-example-line test-example-line-odd test-example-line-3">3</div><div class="test-example-line test-example-line-even test-example-line-4">73</div><div class="test-example-line test-example-line-odd test-example-line-5">1000</div></pre></div><div class="output"><div class="title">Output<div title="Copy" data-clipboard-target="#id003936436616168856" id="id0018168910657452186" class="input-output-copier">Copy</div></div><pre id="id003936436616168856">YES
NO
YES
YES
NO
</pre></div></div></div>