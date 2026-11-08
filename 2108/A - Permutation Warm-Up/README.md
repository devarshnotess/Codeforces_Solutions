<h2><a href="https://codeforces.com/contest/2108/problem/A" target="_blank" rel="noopener noreferrer">2108A — Permutation Warm-Up</a></h2>

| | |
|---|---|
| **Difficulty** | 800 |
| **Language** | Rust 2021 |
| **Verdict** | ✅ Accepted |
| **Problem Link** | [Codeforces 2108A](https://codeforces.com/contest/2108/problem/A) |

## Topics
`combinatorics` `greedy` `math`

---

## Problem Statement

<div class="header"><div class="title">A. Permutation Warm-Up</div><div class="time-limit"><div class="property-title">time limit per test</div>1 second</div><div class="memory-limit"><div class="property-title">memory limit per test</div>256 megabytes</div><div class="input-file input-standard"><div class="property-title">input</div>standard input</div><div class="output-file output-standard"><div class="property-title">output</div>standard output</div></div><div><p> </p><p>For a permutation $$$p$$$ of length $$$n$$$$$$^{\text{∗}}$$$, we define the function:</p><p>$$$$$$ f(p) = \sum_{i=1}^{n} \lvert p_i - i \rvert $$$$$$</p><p>You are given a number $$$n$$$. You need to compute how many <span class="tex-font-style-bf">distinct</span> values the function $$$f(p)$$$ can take when considering <span class="tex-font-style-bf">all possible</span> permutations of the numbers from $$$1$$$ to $$$n$$$.</p><div class="statement-footnote"><p>$$$^{\text{∗}}$$$A permutation of length $$$n$$$ is an array consisting of $$$n$$$ distinct integers from $$$1$$$ to $$$n$$$ in arbitrary order. For example, $$$[2,3,1,5,4]$$$ is a permutation, but $$$[1,2,2]$$$ is not a permutation ($$$2$$$ appears twice in the array), and $$$[1,3,4]$$$ is also not a permutation ($$$n=3$$$ but there is $$$4$$$ in the array). </p></div></div><div class="input-specification"><div class="section-title">Input</div><p>Each test contains multiple test cases. The first line contains the number of test cases $$$t$$$ ($$$1 \le t \le 100$$$). The description of the test cases follows. </p><p>The first line of each test case contains an integer $$$n$$$ ($$$1 \leq n \leq 500$$$) — the number of numbers in the permutations.</p></div><div class="output-specification"><div class="section-title">Output</div><p>For each test case, output a single integer — the number of distinct values of the function $$$f(p)$$$ for the given length of permutations.</p></div><div class="sample-tests"><div class="section-title">Example</div><div class="sample-test"><div class="input"><div class="title">Input<div title="Copy" data-clipboard-target="#id002254467116433736" id="id0025698364929442574" class="input-output-copier">Copy</div></div><pre id="id002254467116433736"><div class="test-example-line test-example-line-even test-example-line-0">5</div><div class="test-example-line test-example-line-odd test-example-line-1">2</div><div class="test-example-line test-example-line-even test-example-line-2">3</div><div class="test-example-line test-example-line-odd test-example-line-3">8</div><div class="test-example-line test-example-line-even test-example-line-4">15</div><div class="test-example-line test-example-line-odd test-example-line-5">43</div></pre></div><div class="output"><div class="title">Output<div title="Copy" data-clipboard-target="#id0093290404347482" id="id005732407869146687" class="input-output-copier">Copy</div></div><pre id="id0093290404347482">2
3
17
57
463
</pre></div></div></div><div class="note"><div class="section-title">Note</div><p>Consider the first two examples of the input.</p><p>For $$$n = 2$$$, there are only $$$2$$$ permutations — $$$[1, 2]$$$ and $$$[2, 1]$$$. $$$f([1, 2]) = \lvert 1 - 1 \rvert + \lvert 2 - 2 \rvert = 0$$$, $$$f([2, 1]) = \lvert 2 - 1 \rvert + \lvert 1 - 2 \rvert = 1 + 1 = 2$$$. Thus, the function takes $$$2$$$ distinct values.</p><p>For $$$n=3$$$, there are already $$$6$$$ permutations: $$$[1, 2, 3]$$$, $$$[1, 3, 2]$$$, $$$[2, 1, 3]$$$, $$$[2, 3, 1]$$$, $$$[3, 1, 2]$$$, $$$[3, 2, 1]$$$, the function values of which will be $$$0, 2, 2, 4, 4$$$, and $$$4$$$ respectively, meaning there are a total of $$$3$$$ values.</p></div>