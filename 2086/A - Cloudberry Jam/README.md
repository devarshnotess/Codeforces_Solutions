<h2><a href="https://codeforces.com/contest/2086/problem/A" target="_blank" rel="noopener noreferrer">2086A — Cloudberry Jam</a></h2>

| | |
|---|---|
| **Difficulty** | 800 |
| **Language** | Rust 2021 |
| **Verdict** | ✅ Accepted |
| **Problem Link** | [Codeforces 2086A](https://codeforces.com/contest/2086/problem/A) |

## Topics
`math`

---

## Problem Statement

<div class="header"><div class="title">A. Cloudberry Jam</div><div class="time-limit"><div class="property-title">time limit per test</div>2 seconds</div><div class="memory-limit"><div class="property-title">memory limit per test</div>512 megabytes</div><div class="input-file input-standard"><div class="property-title">input</div>standard input</div><div class="output-file output-standard"><div class="property-title">output</div>standard output</div></div><div><p>The most valuable berry of the Karelian forests is cloudberry. To make jam from cloudberries, you take equal amounts of berries and sugar and cook them. Thus, if you have $$$2$$$ kg of berries, you need $$$2$$$ kg of sugar. However, from $$$2$$$ kg of berries and $$$2$$$ kg of sugar, you will not get $$$4$$$ kg of jam, as one might expect, but only $$$3$$$ kg, since some of the jam evaporates during cooking. Specifically, during standard cooking, exactly a quarter (or $$$25\%$$$) of the jam evaporates.</p><p>How many kilograms of cloudberries are needed to prepare $$$n$$$ $$$3$$$-kilogram jars of jam?</p><center> <img class="tex-graphics" src="https://espresso.codeforces.com/95b8b620bbececd08402497d017d61c8cca50efe.png" style="max-width: 100.0%;max-height: 100.0%;">   </center></div><div class="input-specification"><div class="section-title">Input</div><p>Each test consists of several test cases. The first line contains a single integer $$$t$$$ ($$$1 \le t \le 10^{4}$$$) — the number of test cases. The following lines describe the test cases.</p><p>Each test case contains a single integer $$$n$$$ ($$$1 \le n \le 10^8$$$) — the number of jars of jam that need to be prepared.</p></div><div class="output-specification"><div class="section-title">Output</div><p>For each test case, output a single integer — the amount of berries needed for the jam in kilograms.</p></div><div class="sample-tests"><div class="section-title">Example</div><div class="sample-test"><div class="input"><div class="title">Input<div title="Copy" data-clipboard-target="#id0004433437041325805" id="id007943563116529165" class="input-output-copier">Copy</div></div><pre id="id0004433437041325805"><div class="test-example-line test-example-line-even test-example-line-0">2</div><div class="test-example-line test-example-line-odd test-example-line-1">1</div><div class="test-example-line test-example-line-even test-example-line-2">3</div></pre></div><div class="output"><div class="title">Output<div title="Copy" data-clipboard-target="#id006542950484816946" id="id0008666094667513702" class="input-output-copier">Copy</div></div><pre id="id006542950484816946">2
6
</pre></div></div></div><div class="note"><div class="section-title">Note</div><p>For the test case $$$1$$$, explanations are given in the text of the statement — to prepare $$$1$$$ jar of jam, you need $$$2$$$ kilograms of cloudberries.</p><p>Consider the test case $$$2$$$: if we take $$$6$$$ kilograms of berries and $$$6$$$ kilograms of sugar, we get $$$\frac{(6 + 6) \cdot 3}{4} = 9$$$ kilograms of jam; which gives $$$\frac{9}{3} = 3$$$ jars of jam.</p></div>