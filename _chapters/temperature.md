---
title: Temperature and Reference Frames
---

### Horocyclic Temperature

I propose a notion of colour temperature alternative to the traditional one based on the Planckian locus and more aligned with the subjective impression of "warmth". Temperature of individual colours, not merely lights, is artistically relevant. Warmer colours are perceived to be closer, moving outside the screen towards the viewer, and more abstractly have a quality of "intimacy"; conversely, cooler colours recede.

<!-- As already said, the $w = t \in \mathbb{R}$ axis of neutrals has a natural and intuitive notion of warmth, which is signed distance alongside it from a reference white $N_0$, and thus $T_{w_0} = 2 \th^{-1}(t)$. This temperature is in completely different units and interpretation than physical temperature of a black body, spanning on the neutral axis from $T = -1.07$ (noon sky) through $T = 0.0$ at true white $N_0$ into $T=1.62$ (glowing orangeish red). Note that warmer and cooler colours than these extremes exist off-axis. -->

The first assumption that I make is that temperature only depends on chroma, not luma (if both are properly constructed). This means that the isotherms, the sets of colors of equal temperature, are curves in the hyperbolic chroma plane. What kind of curves? It would be preferable for them to be a more symmetric and simple family of curves, or at least be approximately described as such.

Let's begin by identifying the "warmest hue", which must be properly defined to hold at any given saturation. We observe that such warmest hue is very clearly perceived to be an orangeish-red, or **vermillion**. Blood red and carrot on either side, at equal saturation, are sharply seen to be colder. Consider the hyperbolic line from D65 white to vermillion, extended backwards as well into sky blue. We'll call this the **thermal axis**, and HYCOL has been defined so that this is conveniently just the horizontal line of $$w \in \mathbb{R}$$. It would be reasonable to define the temperature for colors on the thermal axis as the signed distance from white, and in fact we'll see in a bit that this is sound. If we do, then translations along the thermal axis are just simply shifts in temperature.

Then isotherms must meet the thermal axis at right angle, and their temperature is that of the point of intersection. If they are sufficiently symmetric, so curves of constant curvature, then they are a pencil of generalized circles orthogonal to the thermal axis; namely, they could be any of these:

* Straight lines
* Hypercycles

*wip*


![](/assets/isotherm_strips.png)