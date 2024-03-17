# Triangular Blending and Fields


*wip: rewrite for pandoc markdown*

The fundamental object that the theory is based on is the **geodesic gradient**, which is the "optimal" interpolation between two colours (the vertices), in the sense that it involves the least possible perceived total colour variation and that such perceived change occurs at constant speed. This is geometrically the geodesic segment between two points, which is the shortest path and, by necessity, of curvature zero.

When a geodesic gradient stretches between two vertices with different luminance, the luminance changes at the same rate that the chromaticity does, relative to the total. For example, when we are $1/3$ of the way through from $A$ to $B$, or equivalently we are performing a blend of $A$ and $B$ with weights $2/3$ and $1/3$ respectively, the luminance will be $l_\mathrm{blend} = \frac{2}{3} l_A + \frac{1}{3}l_B$, *and* the chromaticity will be at a distance along the path in the same proportion: $d(w_A,w_\mathrm{blend}) = \frac{1}{2} d(w_B,w_\mathrm{blend})$. This follows from the fact that the colour metric is a product metric, and defines uniquely the geodesic gradient between any two colours, or equivalently the weighted blend between two colours $A$,$B$ with weights $\lambda_A,\lambda_B$ such that $\lambda_A+\lambda_B=1$.

For the computational aspect, there is a closed form expression for such an interpolation as the hyperbolic equivalent of the SLERP operation for spherical geometry, sometimes called HLERP. However this is of limited use, since the generalization to a weighted blend of more than two colours, which we do require extensively, has no closed form solution anyway.

We are interested in blending several colours $C_1, C_2,\ldots, C_n$ with weights $\lambda_1,\ldots, \lambda_n$ that sum to one: $\sum_i \lambda_i = 1$. (We will only need $n=3$, where HLERP is barycentric coordinates over a triangle, but no simplification is offered compared to the general case). Of course, we will blend lumas $l_i$ and chromas $w_i$ separately with the same weights. For lumas, we just perform a regular linear interpolation. For hyperbolic chromas, defining a canonical weighted average in an arbitrary space is generally impossible, but there is a consistent way to do so for $\mathbb{H}^2$ points due to its high degree of symmetry. 

We impose the requirement that the n-fold HLERP reduces to the two-point geodesic HLERP when all the weights but two vanish:

$$\operatorname{HLERP}(w_1,\lambda;w_2,1-\lambda;w_3,0;w_4,0,\ldots) = \operatorname{HLERP}(w_1,\lambda;w_2,\lambda-1)$$

for triangles, for examples, this means that blends where one vertex has weight zero reduce to the blend between the remaining two vertices.

The following definition successfully generalizes the Euclidean lerp to both spherical and hyperbolic geometry. The weighted average $\overline{w} = \bigoplus_i \lambda_i w_i$ is the unique point that minimizes the objective function:

$$J(w) = \frac{1}{2}\sum_i \lambda_i \,d(w,w_i)^2$$

This optimization must be performed numerically for $n>2$. We employ a version of gradient descent, though care should be applied to understand the nature of this gradient as a mathematical object. First, let's notate by $M(q)$ the Moebius transformation that sends $0$ to $q$ translating along the geodesic segment from $0$ to $q$, which is explicitly

$$M(q) = \frac{1}{\sqrt{1-|q|^2}}\begin{pmatrix}1 & q \\\overline{q} & 1\end{pmatrix}$$

This matrix embodies the notion of translating in the direction and distance of $q$ "in the simplest way possible", considering the difficulty of being unable to compare directions at different points.

We can then define the positions of the vertices relative to $w$ by performing the inverse transformation:

$$r_i = M(-w) \cdot w_i$$

Of course, $d(r_i,0)=d(w_i,w)$. Then, we can define the "logarithm" of such relative position as a complex number with the same phase and norm equal to the distance:

$$\log[r_i] = \log \left[|r_i|e^{i\phi_i}\right] = d(r_i,0) e^{i\phi_i}$$

The logarithm is a tangent vector which belongs to the tangent space to $\mathbb{H}^2$ at the origin, which is just $\mathbb{C}$, a vector space where weighted averages are well defined. The gradient is recognized to be

$$\nabla J(w) = - \sum_i \lambda_i \log[r_i]$$

To perform a gradient descent iteration, we need to properly generalize the operation of (small) displacement $w \mapsto w -\nabla J(w)$. We understand we really need to transform $w$ by the exponential of the tangent vector $-\nabla J$. We define the "exponential" to invert the "logarithm":

$$\exp[|v|e^{i\phi}] = \tanh\left(\frac{|v|}{2}\right)e^{i\phi}$$

And finally the iteration step is as follows:

$$w \mapsto M(\exp[-\nabla J(w)]) \cdot w$$

We employ the following algorithm with linear convergence, described directly in the Poincaré disk for convenience:

- Set $w$ to a first guess, e.g. $\sum_i \lambda_i w_i$.
- Start loop:
- - Find vertices relative to $w$ as $r_i = M(-w)\cdot w_i$.
- - Compute gradient as $\nabla J(w) = - \sum_i \lambda_i \log[r_i]$
- - Compute $u = \exp[-\nabla J]$. This is the next step position, but relative to current $w$.
- - Translate $w$ by $u$ through $w \mapsto M(u) w$
- - If $|u|<\epsilon$, break and return $w$ as $\overline w$.