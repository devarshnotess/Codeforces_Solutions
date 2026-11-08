<h2><a href="https://codeforces.com/contest/2033/problem/A" target="_blank" rel="noopener noreferrer">2033A — Sakurako and Kosuke</a></h2>

| | |
|---|---|
| **Difficulty** | 800 |
| **Language** | C++20 (GCC 13-64) |
| **Verdict** | ✅ Accepted |
| **Problem Link** | [Codeforces 2033A](https://codeforces.com/contest/2033/problem/A) |

## Topics
`constructive algorithms` `implementation` `math`

---

## Problem Statement

<div class="header"><div class="title">A. Sakurako and Kosuke</div><div class="time-limit"><div class="property-title">time limit per test</div>1 second</div><div class="memory-limit"><div class="property-title">memory limit per test</div>256 megabytes</div><div class="input-file input-standard"><div class="property-title">input</div>standard input</div><div class="output-file output-standard"><div class="property-title">output</div>standard output</div></div><div><p>Sakurako and Kosuke decided to play some games with a dot on a coordinate line. The dot is currently located in position $$$x=0$$$. They will be taking turns, and <span class="tex-font-style-bf">Sakurako will be the one to start</span>. </p><p>On the $$$i$$$-th move, the current player will move the dot in some direction by $$$2\cdot i-1$$$ units. Sakurako will always be moving the dot in the negative direction, whereas Kosuke will always move it in the positive direction. </p><p>In other words, the following will happen:</p><ol> <li> Sakurako will change the position of the dot by $$$-1$$$, $$$x = -1$$$ now </li><li> Kosuke will change the position of the dot by $$$3$$$, $$$x = 2$$$ now </li><li> Sakurako will change the position of the dot by $$$-5$$$, $$$x = -3$$$ now </li><li> $$$\cdots$$$ </li></ol><p>They will keep on playing while the absolute value of the coordinate of the dot does not exceed $$$n$$$. More formally, the game continues while $$$-n\le x\le n$$$. It can be proven that the game will always end.</p><p>Your task is to determine who will be the one who makes the last turn.</p></div><div class="input-specification"><div class="section-title">Input</div><p>The first line contains one integer $$$t$$$ ($$$1\le t\le 100$$$) — the number of games that Sakurako and Kosuke played.</p><p>Each game is described by one number $$$n$$$ ($$$1 \le n\le 100$$$) — the number that defines the condition when the game ends.</p></div><div class="output-specification"><div class="section-title">Output</div><p>For each of the $$$t$$$ games, output a line with the result of that game. If Sakurako makes the last turn, output "<span class="tex-font-style-tt">Sakurako</span>" (without quotes); else output "<span class="tex-font-style-tt">Kosuke</span>".</p></div><div class="sample-tests"><div class="section-title">Example</div><div class="sample-test"><div class="input"><div class="title">Input<div title="Copy" data-clipboard-target="#id007660970102573771" id="id004839420070366741" class="input-output-copier">Copy</div></div><pre id="id007660970102573771"><div class="test-example-line test-example-line-even test-example-line-0">4</div><div class="test-example-line test-example-line-odd test-example-line-1">1</div><div class="test-example-line test-example-line-even test-example-line-2">6</div><div class="test-example-line test-example-line-odd test-example-line-3">3</div><div class="test-example-line test-example-line-even test-example-line-4">98</div></pre></div><div class="output"><div class="title">Output<div title="Copy" data-clipboard-target="#id002092325062343945" id="id005131225677957573" class="input-output-copier">Copy</div></div><pre id="id002092325062343945">Kosuke
Sakurako
Kosuke
Sakurako
</pre></div></div></div>