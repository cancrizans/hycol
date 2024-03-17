---
name: harmony
title: Color Harmony
---


## Complementarity and Skin Frame

Complementary colors is one of the most inconsistent and contradictory topics in color theory. This is inclusive both of the meaning, purpose and effect of complementary colors, and specifically of which colors should be complementary with which.

I argue that this is in fact a manifestation of the notion of complementarity being relative; this relativity fortuitously happens to be in fact mathematically identical to special relativity in physics, since the Lorentz group is the isometry group of the hyperbolic plane. We speak of complementarity only with respect to a center, or whitepoint, which in turn defines a **reference frame**. Complementarity in one frame is distinct from that in another.

First of all, complementarity involves exclusively the chroma component, and it is independent from the luma.

Two chromaticities are complementary in a given frame if the (hyperbolic) straight segment between them passes through the center. Equivalently, we can define **hue** as the angle from the center, assuming the thermal axis from the center to the ideal warm point as 0°, and then complementary colours have hues 180° apart. Hue is frame dependent.

The D65 whitepoint, the "true" grey, defines a frame which is related the physiology of human vision. Complementaries in this frame are opponents of the opponent process theory. Afterimages of a certain color are in its opponent complementary, for example in this well-known illusion:

![](/assets/opponent_illusion.gif)

Staring at the cross for enough time makes a green dot appear in the moving vacancy - this is because green is the opponent complementary to magenta, *because* they are collinear with the grey of the background.

However, for art the opponent pairs are less useful. There is a notion of complementarity, which we will call **harmonic complementarity**, in which complementary colors are those that clash and are perceived to be in a messy and repulsive relationship. Harmonic complementaries are unclear in their intent, displeasing, and have a characteristic sickly shimmer on their boundary. This impression is worse at higher saturation, but it is in fact always present.

The prototypical clashing pair is red and green: <i class="ctile-13"></i> <i class="ctile-6"></i>. Note that this is different from the D65 frame, since the opponent complementary of red is cyan, and for green is, as seen, magenta.

I have performed an experiment and acquired data to precisely determine which pairs of colors are harmonic complementaries. If we draw the segments between them, they *roughly* appear to be passing through a single point:

![](/assets/complementarity_data.png)

This harmonic center, which I've fixed at $$w=0.25 + 0.1i$$ is a decidedly orange-ish chromaticity which happens to be, roughly, that of human skin (of various ethnicities). It looks like this at various lumas

![](/assets/skincenter.png)

The significance of a match with skin tones is probably a rabbit hole best left for spare afternoons. Still, we can term this the skin center, defining a **skin frame**. For some reason, harmony is relative to the skin frame. After transforming ("boosting") to the skin frame, the definition of hues and their spacing is completely different from D65. These new, harmonic hues shrink the space dedicated to cool colors and expand the accuracy on warm hues, due to the inherently warm bias of the whitepoint. 

In the skin frame, D65 itself is not neutral anymore, and it is perceived to have a **blue** hue, with mild saturation. This means that, as such, it really is harmonic complementary to orange, and this is true at all luma. We can therefore conclude, somewhat humorously but with reasonable motivation, that **white is complementary to orange**, and they will indeed clash if the alignment is correct.



## 14-tone system

The division of the color wheel in the skin frame into a certain number of equal spokes is completely arbitrary, and arguably unnecessary since the geometric picture is available; nevertheless, having a discrete structure will greatly improve practical use, even if there is a small loss of generality. So far, the best choice I've found is to divide hues into **14 equal steps**.

For the skin frame hues in the 14-tone system I've chosen the following naming convention, using lowercase latin letters to avoid confusion with pre-existing systems:

| | | |
|-|-|-|
|<i class="ctile-0">v</i>| Vermillion | Warm, orange-ish red|
|<i class="ctile-1">c</i>| Carrot | Reddish orange|
|<i class="ctile-2">o</i>| Orange | Orange fruit, cold orange |
|<i class="ctile-3">h</i>| Honey | Warm, orange-ish yellow|
|<i class="ctile-4">y</i>| Yellow | Lemon, cold yellow|
|<i class="ctile-5">x</i>| Chartreuse | Pear, yellow-green|
|<i class="ctile-6">g</i>| Green | Slightly cold green |
|<i class="ctile-7">t</i>| Teal | Bluish green |
|<i class="ctile-8">n</i>| Noon | Noon sky, greenish blue|
|<i class="ctile-9">b</i>| Blue | Late afternoon sky |
|<i class="ctile-10">l</i>| Lavender | Bluish violet|
|<i class="ctile-11">m</i>| Magenta | Cold pinks |
|<i class="ctile-12">p</i>| Pink | Bordeaux, warm pinks, violet-ish red |
|<i class="ctile-13">r</i>| Red | Blood red |

The diagram below depicts the extents of the sRGB gamut in the skin frame with marked hue spokes, and the max-saturation colors of each hue. (For this guide we will use more subdued representative tones since the wild fluctuations in luma and saturation on the gamut boundary can be confusing). Also marked, with greek letters, the RGB primaries and secundaries, displaying misalignment with the new categories (e.g., primary red is decidedly warm).

![](/assets/skinframe_gamut.png)

Observe that D65 greys, including full black and full white, are exactly halfway between n and b, meaning their complementary axis is as far as possible from o and c. This is convenient for digital art as it allows the use of full black and white while minimizing the risk of clashing with intermediate tones, if o and c are preferred.

### Categorical Warp

*wip*

## 14-Tone Intervals

Harmonic coloring is based on two principles:

1. Complementary tones are maximally dissonant and should always be avoided.
2. Contrast should be employed optimally to establish importance of elements.

These are forces in opposition: clearly complementary colors have the greatest contrast, yet the best choice for a dichromatic piece is an interval which is as large as possible without being dissonant. Since complementaries are separated by 7 steps, this optimal choice is a 6-step interval we call a **resonant**. Every tone has two resonants going clockwise or counter-clockwise. This is the 14 tones connected by resonance; since 6 is even (and so is 14), resonances have the same parity, thus there is an even cycle of resonants **voygnlp** and an odd cycle **chxtbmr**, analogous to the circle of fifths in music.

![](/assets/star.png)

Working only with resonants is quite boring and static; some variation in relationship between the hues can expand the emotional range of a piece. While the contrast is lesser, an interval of 5 steps is also quite pleasant, and it could be called a **semi-resonant**. Here is all the possible intervals, sorted by increasing dissonance, with my interpretation:

|N | Interval | Quality | Left | Right|
|---|---|---|---|
|0 | Unison | Complex (see below) | <i class="ctile-13"></i> <i class="ctile-13"></i>|
|6 | Resonant | Mutual emphasis: maximum contrast without complementarity |   <i class="ctile-13"></i> <i class="ctile-5"></i> |   <i class="ctile-13"></i> <i class="ctile-7"></i> |
|5 | Semi-resonant | Similar to resonant, but with lesser contrast |   <i class="ctile-13"></i> <i class="ctile-4"></i> |   <i class="ctile-13"></i> <i class="ctile-8"></i> |
|3 | Adjoint | Adjoint tones work together towards the same goal |   <i class="ctile-13"></i> <i class="ctile-2"></i> |   <i class="ctile-13"></i> <i class="ctile-10"></i> |
|2 | Major Analogous | Slightly dissonant: complementary of semi-resonant |    <i class="ctile-13"></i> <i class="ctile-1"></i> |   <i class="ctile-13"></i> <i class="ctile-11"></i> |
|4 | Mediant | Odd and disorienting |   <i class="ctile-13"></i> <i class="ctile-3"></i> |   <i class="ctile-13"></i> <i class="ctile-9"></i> |
|1 | Minor Analogous | Dissonant: complementary of resonant |   <i class="ctile-13"></i> <i class="ctile-0"></i> |   <i class="ctile-13"></i> <i class="ctile-12"></i> |
|7 | Complementary | Highly displeasing | <i class="ctile-13"></i> <i class="ctile-6"></i> |

![](/assets/combinatoric_modes.png)

### Axial complementarity

*wip*

## Triads

The simplest non-trivial palette structure is the triangular field between three vertices, one of which is marked as the **key**, and the other two for the **base**. The key is the chromatic focal point of the piece and the purpose of the triad is to emphasize the key by establishing a context in which it is intensified by contrast.

The triad should be thought of as an arrow pointing to the key, it is inherently a directed and dynamical structure. The motion is from base to key, but also background to foreground, general to specific, and it is the physical guided motion of the eye as it absorbs the piece.

A triad can certainly not have any complementaries among its vertices. In addition, it should envelop the harmonic center, so the base vertices should be on opposite sides of the complementary to the key. Finally, since the key should be emphasized, the interval of the base must be smaller than those from the base to the key. This only leaves four possibilities:

![](/assets/triads.png)

The key is positioned on the right, and I have further marked tones that have been excluded due to complementarity. 

The **ultra-narrow** triad is functional but has undesirable properties: the base is dissonant and the range is small. This palette is difficult to extend as further fields quickly drown out the main one. In addition, the symmetry with respect to the key axis makes it rather static.

The **narrow** triads are similar, but not identical, to Fletcher's. They are asymmetric and come in two chiral forms. The intervals are varied and harmonious: a resonant, a semi-resonant, and an adjoint. These palettes exhalt the key efficiently while also incorporating a great degree of playful secondary motion. They provide the range for rather surprising and dynamic artwork, and have the greater potential to be extended. Generally narrow triads are the most promising choice for main fields.

**Wide** triads are unweildy. The primality of the key is extremely weak (in fact, in the 12 tone system these are equilateral triangles), the base interval is a mediant which is generally unpleasant or uncertain in character, and finally there is symmetry. The range is vast, but this is potentially at the detriment of the piece.

### Tonnetz

Given what we know about harmony so far, the abstract color wheel with analogous tones adjacent is not a particularly convenient representation. We can lift a page from music theory again and construct a two-dimensional lattice (tonnetz) where each edge represents a particular consonant interval:

![](/assets/lattice.png)

A portion of a Neo-Riemannian style lattice for the 14-tone system. The lattice is periodic. Dotted lines represent adjoints, solid line are resonants, and dashed lines are semi-resonants. Every triangle is a narrow triad, and each tone is the key for the triads directly above and below it.

For example, y is the bottom vertex of the narrow triad y/b/p, with y as the key, p as the base resonant, and b as the semi-resonant. However, it shares an edge with the narrow triad p/y/t, with now p as the key, y as the base resonant, and t as the semi-resonant. Joined together, the two triads assemble in a four-tone object including y,b,p and t, and with the new interval b-t which is a major analogous.

The tonnetz also places at maximum distance colors which are complementaries or mediants, providing a further rationalization of why they're perceived as displeasing -- they are "distant" in terms of the number of resonant steps that must be taken to bring one to the other. In this picture (which is to be understood as more poetic than literal), r and g are offending to the eye because the eye is invited to imagine, for example, that the relationship of g to r is that g is resonant to p, which is semi-resonant to t, which is resonant to r, but also the inverted path r -> x -> v -> g. The complexity and ambiguity of the relationship is too much for immediate comprehension, as is the case, for example for r and t.

### Value

*wip*

## Polytriadic Structure

*wip*