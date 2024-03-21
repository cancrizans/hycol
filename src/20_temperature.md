# Temperature and Reference Frames


## Horocyclic Temperature: Rationale

I propose a notion of colour temperature alternative to the traditional one based on the Planckian locus and more aligned with the subjective impression of "warmth". Temperature of individual colours, not merely lights, is artistically relevant. Warmer colours are perceived to be closer, moving outside the screen towards the viewer, and more abstractly have a quality of "intimacy"; conversely, cooler colours recede.

<!-- As already said, the $w = t \in \mathbb{R}$ axis of neutrals has a natural and intuitive notion of warmth, which is signed distance alongside it from a reference white $N_0$, and thus $T_{w_0} = 2 \th^{-1}(t)$. This temperature is in completely different units and interpretation than physical temperature of a black body, spanning on the neutral axis from $T = -1.07$ (noon sky) through $T = 0.0$ at true white $N_0$ into $T=1.62$ (glowing orangeish red). Note that warmer and cooler colours than these extremes exist off-axis. -->

The first assumption that I make is that temperature only depends on chroma, not luma (if both are properly constructed). This means that the isotherms, the sets of colors of equal temperature, are curves in the hyperbolic chroma plane. What kind of curves? It would be preferable for them to be a more symmetric and simple family of curves, or at least be approximately described as such.

Let's begin by identifying the "warmest hue", which must be properly defined to hold at any given saturation. We observe that such warmest hue is very clearly perceived to be an orangeish-red, or **vermillion**. Blood red and carrot on either side, at equal saturation, are sharply seen to be colder. Consider the hyperbolic line from D65 white to vermillion, extended backwards as well into sky blue. We'll call this the **thermal axis**, and HYCOL has been defined so that this is conveniently just the horizontal line of $w \in \mathbb{R}$. It would be reasonable to define the temperature for colors on the thermal axis as the signed distance from white, and in fact we'll see in a bit that this is sound. If we do, then translations along the thermal axis are just simply shifts in temperature.

Then isotherms must meet the thermal axis at right angle, and their temperature is that of the point of intersection. If they are sufficiently symmetric, so curves of constant curvature, then they are a pencil of generalized circles orthogonal to the thermal axis. 

The first observation to make is that the isotherms do indeed curve, with the concavity oriented towards warmth. This is manifested in the fact that the temperature variation with hue is much larger for warm hues, i.e. vermillion is clearly warmer than neighbouring tones, while cold hues tend to have a very subdued variation, with greens, teals, blues and even purples having roughly comparable temperature. The second piece of evidence is the expectation that classical (Planck-locus based) isotherms in the CIEXYZ model are expected to intersect somewhere in the magentas -- hyperbolic lines perpendicular to the thermal axis could not possibly converge, so there must be curvature.

If isotherm curvature is present towards warmth, then depending on its value we have three distinct geometrical cases:

* If curvature < 1, the isotherms are hypercycles (aka equidistant curves) and they do eventually diverge.
* If curvature = 1, matching precisely the $\mathbb{H}^2$ curvature, the isotherms are horocycles, which are infinite curves that all meet at one point at infinity on the thermal axis, the **ideal warm point**.
* If curvature > 1, then the isotherms are circles with a common center. This center is a point on the thermal axis warmer than vermillion and way out of the physical gamut.

I find that the limiting horocyclical case is as good a match with observations than the other two possibilities, and is more parsimonious as it eliminates the parameter of the isotherm curvature. Let's therefore assume the horocyclical model for temperature.

## Horocyclic Temperature: Review

The ideal warm point $w_W=1$ is a special point on the boundary of $\mathbb{H}^2$ at infinity. Horocyclic temperature is in fact just distance from this point, properly renormalized. In particular, imagine starting from a non-ideal warm center $w_W$, then define the temperature of a color $w_U$ as the distance from the center minus the temperature of D65 white:

$$T_U = d(w_U, w_W) - d(0,w_W)$$

Then sending $w_W \rightarrow 1$ the temperature converges in the limit. As argued, the lines of constant temperature, the isotherms, are horocycles, which can be seen as circles of infinite radius centered *and* converging onto on the point $w_W$, which is infinitely far away. In the Poincaré disk model, the horocycles are represented as circles tangent to the boundary at $w_W$.

Here are the isotherms at different luminances, depicted as horizontal strips. The horizontal metric is accurate.

![](assets/isotherm_strips.png)

Consider now curves which are orthogonal to the isotherms, which are the curves of maximum temperature variation. These are in fact straight lines, specifically all the lines from $w_W$. Since these generalize the original thermal axis through D65, we can call all of these **thermal axes**, and hypothesize a sort of "relativity" holds which maps any thermal axis to any other. Within any thermal axis, the temperature difference between two colors is precisely their distance (which is along the axis, since it's a line).

Being orthogonal everywhere, the isotherms and thermal axes suggest a definition of a different set of conformal coordinates. This in fact the half-plane model of the hyperbolic plane, detailed in the appendix. If we define

$$ z = -i\frac{w+1}{w-1} $$

then $z$ with $\Im z > 0$ defines a coordinate on the chroma plane, the **thermal chart**. The imaginary part of $z$ is related to the temperature, while the real part expresses a kind of green-violet axis:

$$z = \Re{z} + i e^{-T}$$

The chart is not particularly useful for visualization purposes or color theory, as it distorts distances even more severely than the disk model, but it does provide the following convenient formula for the temperature of any chroma:

$$T = - \log \Im \big(-i\frac{w+1}{w-1}\big) = - \log \frac{1-|w|^2}{|w-1|^2}$$

## Reference Frames

$\mathbb{H}^2$ was chosen as a chroma plane because of its elevated degree of symmetry. The temperature system breaks part of it; specifically the isometries that remain are those who keep the ideal warm point fixed, which is a two-dimensional subgroup. We will call these "boosts" for lack of a better term.

Consider a chroma $c$ which I want to use as a new "center", as a whitepoint, in alternative to D65 white which is $0$. There is precisely one boost which takes $c \mapsto 0$. (Note: this would not have been true if the warm point was not ideal, so the ability to change whitepoint and maintain a notion of temperature is another advantage of the horocyclical model.) This means that every whitepoint defines uniquely a new **reference frame** and vice versa.

To be more explicit - when a new whitepoint is presented, it's easy enough to recenter our coordinates on it so that colors are now described as relative to the new context. However, rotation around the whitepoint is now arbitrary, meaning relative hue is defined but absolute hue is meaningless (a disaster - we can't tell red from blue!) and requiring the new frame to be parallel to the original one is impossible due to the curvature of chroma space. Instead, the frame is aligned so that hue zero is the direction from the whitepoint to the ideal warm pole, which is to say the thermal axis passing through it. That direction is always the warmest hue, by definition the vermillion relative to that whitepoint.

Observe that the pencil of isotherm is invariant under boosts, so they will look the same in all reference frame - horocycles through the ideal warm point, which is always $w_W=1$, but the temperature they refer to will be shifted by the temperature of the whitepoint itself.