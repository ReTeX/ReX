# ReX &nbsp; [![](https://travis-ci.org/cbreeden/ReX.svg?branch=master)](build status) [![](https://tokei.rs/b1/github/cbreeden/rex)](https://github.com/cbreeden/rex)

<p align="center"><img src="rex.png" alt="ReX logo" width="300px"/></p>
<h3 align="center">Typesetting Mathematics</h3>

# Samples

You can try ReX live [here](https://s3bk.github.io/iReX/editor.html)!  Simply type in a formula in the editor and click the display on the top to update the rendering.

Note: ReX rendered all of these examples in SVG, but due to limitations in SVG rendering on GitHub, we need to convert them to PNG.
See the `samples/` folder for the original SVG source.

### The Quadratic Fromula
`x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}`

![Example](samples/The_Quadratic_Fromula.png)

### Double angle formula for Sine
`\sin(\theta + \phi) = \sin(\theta)\cos(\phi) + \sin(\phi)\cos(\theta)`

![Example](samples/Double_angle_formula_for_Sine.png)

### Divergence Theorem
`\int_D (\nabla \cdot F)\,\mathrm{d}V = \int_{\partial D} F \cdot n\,\mathrm{d}S`

![Example](samples/Divergence_Theorem.png)

### Standard Deviation
`\sigma = \sqrt{ \frac{1}{N} \sum_{i=1}^N (x_i - \mu)^2 }`

![Example](samples/Standard_Deviation.png)

### Fourier Inverse
`f(x) = \int_{-\infty}^{\infty} \hat f(\xi) e^{2\pi i \xi x}\,\mathrm{d}\xi`

![Example](samples/Fourier_Inverse.png)

### Cauchy-Schwarz Inequality
`\left\vert \sum_k a_kb_k \right\vert \leq \left(\sum_k a_k^2\right)^{\frac12}\left(\sum_k b_k^2\right)^{\frac12}`

![Example](samples/Cauchy-Schwarz_Inequality.png)

### Exponent
`e = \lim_{n \to \infty} \left(1 + \frac{1}{n}\right)^n`

![Example](samples/Exponent.png)

### Ramanujan's Identity
`\frac{1}{\pi} = \frac{2\sqrt{2}}{9801} \sum_{k=0}^\infty \frac{ (4k)! (1103+26390k) }{ (k!)^4 396^{4k} }`

![Example](samples/Ramanujan's_Identity.png)

### A surprising identity
`\int_{-\infty}^{\infty} \frac{\sin(x)}{x}\,\mathrm{d}x = \int_{-\infty}^{\infty}\frac{\sin^2(x)}{x^2}\,\mathrm{d}x`

![Example](samples/A_surprising_identity.png)

### Another gem from Ramanujan
`\frac{1}{\left(\sqrt{\phi\sqrt5} - \phi\right) e^{\frac{2}{5}\pi}} = 1 + \frac{e^{-2\pi}}{1 + \frac{e^{-4\pi}}{1 + \frac{e^{-6\pi}}{1 + \frac{e^{-8\pi}}{1 + \cdots}}}}`

![Example](samples/Another_gem_from_Ramanujan.png)

### Another gem from Cauchy
`f^{(n)}(z) = \frac{n!}{2\pi i} \oint \frac{f(\xi)}{(\xi - z)^{n+1}}\,\mathrm{d}\xi`

![Example](samples/Another_gem_from_Cauchy.png)

### An unneccesary number of scripts
`x^{x^{x^x_x}_{x^x_x}}_{x^{x^x_x}_{x^x_x}}`

![Example](samples/An_unneccesary_number_of_scripts.png)

### Quartic Function
`\mathop{\overbrace{c_4x^4 + c_3x^3 + c_2x^2 + c_1x + c_0}}\limits^{\gray{\mathrm{Quartic}}}`

![Example](samples/Quartic_Function.png)

### Another fun identity
`3^3 + 4^4 + 3^3 + 5^5 = 3435`

![Example](samples/Another_fun_identity.png)

# Install

First note that ReX is currently in heavy development and is not intended to be used in any way other than for testing and debugging.
That being said, you can install ReX using a Rust compiler.  Instructions are found [here](https://www.rustup.rs/).

You can look at the examples in the `tests/` folder to see ReX in action, or simply run

```
cargo run 'x = \frac{-b \pm \sqrt{b^2 - 4ac}{2a}'
```

for a standalone SVG.  The file will be saved as "test.svg".

# License

ReX is primarily distributed under the terms of both the MIT license and
the Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
