# MQS

Math Question Solver (work in progress)

### Example Input

```lua
#std -- import std

-- theorem
!unfold := x^2 + ..0 x -> (x + ..0 / 2)^2 - (..0 / 2)^2

-- variables
$old := x^1 + 4 x + y^2 + 6 y = 0
$new := (x + 2)^2 + (y + 3)^2 = 13

-- question to solve (?0)
?:= old <=> new
```

### Example Markdown Output

(output from mqs with `--markdown` option)

**question to solve: *?0*** \
&emsp;**theory:** $\operatorname{old} \iff \operatorname{new}$ \
&emsp;**meaning:** $\operatorname{old}$ implies $\operatorname{new}$ \
&emsp;**approach:** \
&emsp;&emsp;1: substitute `$old` \
&emsp;&emsp;&emsp;$\operatorname{old} \longrightarrow x^2 + 4 x + y^2 + 6 y = 0$ \
&emsp;&emsp;2: substitute `$new` \
&emsp;&emsp;&emsp;$\operatorname{new} \longrightarrow (x + 2)^2 + (y + 3)^2 = 10$ \
&emsp;&emsp;3: rewrite using `!unfold` \
&emsp;&emsp;&emsp;$x^2 + 4 x \longrightarrow (x + \frac{4}{2})^2 - (\frac{4}{2})^2$ \
&emsp;&emsp;4: rewrite using `!unfold` \
&emsp;&emsp;&emsp;$y^2 + 6 y \longrightarrow (y + \frac{6}{2})^2 - (\frac{6}{2})^2$ \
&emsp;&emsp;5: simplify (2x) \
&emsp;&emsp;&emsp;$\frac{4}{2} \longrightarrow 2$ \
&emsp;&emsp;6: simplify \
&emsp;&emsp;&emsp;$2^2 \longrightarrow 4$ \
&emsp;&emsp;7: simplify (2x) \
&emsp;&emsp;&emsp;$\frac{6}{2} \longrightarrow 3$ \
&emsp;&emsp;8: simplify \
&emsp;&emsp;&emsp;$3^2 \longrightarrow 9$ \
&emsp;&emsp;9: rewrite using `std::!move_sub` \
&emsp;&emsp;&emsp;$\space..\space - 9 = \space..\space \longrightarrow \space..\space = \space..\space + 9$ \
&emsp;&emsp;10: simplify \
&emsp;&emsp;&emsp;$0 + 9 \longrightarrow 9$ \
&emsp;&emsp;11: rewrite using `std::!swap_add` \
&emsp;&emsp;&emsp;$(\space..\space - 4) + \space..\space \longrightarrow \space..\space + (\space..\space - 4)$ \
&emsp;&emsp;12: rewrite using std::!move_sub \
&emsp;&emsp;&emsp;$\space..\space - 4 = \space..\space \longrightarrow \space..\space = \space..\space + 4$ \
&emsp;&emsp;13: simplify \
&emsp;&emsp;&emsp;$9 + 4 \longrightarrow 13$ \
&emsp;&emsp;both sides are equal! (using `std::!sides_equal`) \
&emsp;**answer:** correct (true) \
&emsp;**steps tried:** 10023 \
 \
$\frac{1}{1}$ answers are true

### Example Markdown Output At Step

(output from mqs with `--markdown` and `--at 0:9` options)

**question to solve: *?0*** \
&emsp;**theory:** $\operatorname{old} \iff \operatorname{new}$ \
&emsp;**meaning:** $\operatorname{old}$ implies $\operatorname{new}$ \
&emsp;**step 9:** \
&emsp;&emsp;state before step: \
&emsp;&emsp;&emsp;$(x + 2)^2 - 4 + (y + 3)^2 - 9 = 0 \implies (x + 2)^2 + (y + 3)^2 = 13$ \
&emsp;&emsp;step: rewrite using `std::!move_sub` \
&emsp;&emsp;&emsp;$\space..\space - 9 = \space..\space \longrightarrow \space..\space = \space..\space + 9$ \
&emsp;&emsp;state after step: \
&emsp;&emsp;&emsp;$(x + 2)^2 - 4 + (y + 3)^2 = 0 + 9 \implies (x + 2)^2 + (y + 3)^2 = 13$
