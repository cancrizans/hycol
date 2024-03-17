# Appendix

## Models of the Hyperbolic plane



### Poincaré Disk

Parametrized by complex $w$ in the unit disk $|w|<1$. This is a conformal model. The conformal boundary $|w|=1$ is composed of all ideal points.

The representation of geometric objects are as such:
- Lines: generalized circular arcs which meet the boundary at right angles. 
- Circles: Euclidean circles contained entirely within the disk. 
- Horocycles: Euclidean circles tangent to the boundary.
- Hypercycles: generalized circular arcs that meet the boundary at non-right angles.

Angles are accurately represented.

The metric is

$ds^2 = \frac{4|dw|^2}{(1-|w|^2)^2}$

Isometries are given by Moebius transformations in $PSU(1,1)$, so of the form

$$\begin{bmatrix} u & v \\ \overline{v} & \overline{u} \end{bmatrix}$$

where $|u|^2-|v|^2 = 1$. This is actually a double cover, so it's necessary to quotient by the centre $\{-1,1\}$. It can equivalently be described as the unit split-quaternion $q = u + v j$ where $i^2 = -1$, $j^2=1$, $ij = -ji$, and the split-quaternion norm is $N(q) = |u|^2-|v|^2$.


### Poincaré Half-Plane

Parametrized by complex $z = x+iy$ with $y = \Im{z} > 0$. This is a conformal model. The conformal boundary is given by the real axis $\Im{z} = 0$ compactified with the one point $z=\infty$.

The representation of geometric objects are as such:
- Lines: generalized circular arcs which meet the boundary at right angles. (Including vertical lines which are understood to meet $z=\infty$ at 90°)
- Circles: Euclidean circles contained entirely within the disk. 
- Horocycles: Euclidean circles tangent to the boundary. (Including horizontal lines)
- Hypercycles: generalized circular arcs that meet the boundary at non-right angles. (Including any lines not horizontal nor vertical)

The metric is

$$ds^2 = \frac{|dz|^2}{(\Im z)^2} = \frac{dx^2+dy^2}{y^2}$$

Isometries are given by real Moebius transformations in $PSL(2,\mathbb{R})$, so of the form

$$\begin{bmatrix} a & b \\ c & d \end{bmatrix}$$

where $a,b,c,d$ are real and $ad-bc=1$, again with quotient of the overall sign.

### Conversions

#### Equidistand Azimuthal and Poincaré Disk

$$\rho = 2 \tanh^{-1}|w|$$

$$w = \tanh(\rho/2) e^{i\theta}$$

#### Poincaré Disk and Half-Plane

Also known as the Riemann Sphere's Cayley transform.

This is assuming the convention of $w = 1 \rightarrow z = \infty$, $w = -1 \rightarrow z = 0$.

$$ z = -i \frac{w+1}{w-1} $$

$$w = \frac{z-i}{z+i}$$