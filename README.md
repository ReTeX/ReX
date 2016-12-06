# ReX
Typesetting mathematics.

# Progress

![Integral equation](samples/integral.png)

Extendable glyphs (integral, sqrt): `\Vert f \Vert_2 = \sqrt{\int f^2(x)\,\mathrm{d}x}`

![l2 norm](samples/norm.png)

Scripts: `x^{x^{x^x_x}_{x^x_x}}_{x^{x^x_x}_{x^x_x}}`

Todo: Add horizontal kerning to scripts (test `P_n`).

![scripts](samples/scripts.png)

Accents: '\hat A\grave A\bar A\tilde A\hat x \grave x\bar x\tilde x\hat y\grave y\bar y\tilde y'

Todo: Compensate for overshot from accents (see kerning)?

![accents](samples/accents.png)

![quadratic equation](samples/quadratic_accent.png)

`\left` and `\right`: `\sigma = \left(\int f^2(x)\,\mathrm{d}x\right)^{1/2}`

![leftright](samples/leftright.png)

![holder](samples/holder.png)

Fractions: `f^{(n)}(z) = \frac{n!}{2\pi i} \oint \frac{f(\xi)}{(\xi - z)^{n+1}}\,\mathrm{d}\xi`

TODO: There seems to be a few inconsistencies with some fonts and the standard...

![cauchy](samples/cauchy.png)

`\frac{1}{\left(\sqrt{\phi\sqrt5} - \phi\right) e^{\frac{2}{5}\pi}} = 1 + \frac{e^{-2\pi}}{1 + \frac{e^{-4\pi}}{1 + \frac{e^{-6\pi}}{1 + \frac{e^{-8\pi}}{1 + \unicodecdots}}}}`

![nested fractions](samples/nested_fractions.png)

Atom commands [`\mathop`, `\mathrel`, `\mathord`]: `\mathop{\mathrm{lim\,sup}}\limits_{x\rightarrow\infty}\ \mathop{\mathrm{sin}}(x)\mathrel{\mathop{=}\limits^?}1`

![atom commands](samples/atoms.png)