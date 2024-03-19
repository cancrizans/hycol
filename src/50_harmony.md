# Color Harmony

## Complementarity and Skin Frame

Complementary colors is one of the most inconsistent and contradictory topics in color theory. This is inclusive both of the meaning, purpose and effect of complementary colors, and specifically of which colors should be complementary with which.

I argue that this is in fact a manifestation of the notion of complementarity being relative; this relativity fortuitously happens to be in fact mathematically identical to special relativity in physics, since the Lorentz group is the isometry group of the hyperbolic plane. We speak of complementarity only with respect to a center, or whitepoint, which in turn defines a **reference frame**. Complementarity in one frame is distinct from that in another.

First of all, complementarity involves exclusively the chroma component, and it is independent from the luma.

Two chromaticities are complementary in a given frame if the (hyperbolic) straight segment between them passes through the center. Equivalently, we can define **hue** as the angle from the center, assuming the thermal axis from the center to the ideal warm point as 0°, and then complementary colours have hues 180° apart. Hue is frame dependent.

The D65 whitepoint, the "true" grey, defines a frame which is related the physiology of human vision. Complementaries in this frame are opponents of the opponent process theory. Afterimages of a certain color are in its opponent complementary, for example in this well-known illusion:

![](assets/opponent_illusion.gif)

Staring at the cross for enough time makes a green dot appear in the moving vacancy - this is because green is the opponent complementary to magenta, *because* they are collinear with the grey of the background.

However, for art the opponent pairs are less useful. There is a notion of complementarity, which we will call **harmonic complementarity**, in which complementary colors are those that clash and are perceived to be in a messy and repulsive relationship. Harmonic complementaries are unclear in their intent, displeasing, and have a characteristic sickly shimmer on their boundary. This impression is worse at higher saturation, but it is in fact always present.

The prototypical clashing pair is red and green: <i class="ctile-13"></i> <i class="ctile-6"></i>. Note that this is different from the D65 frame, since the opponent complementary of red is cyan, and for green is, as seen, magenta.

I have performed an experiment and acquired data to precisely determine which pairs of colors are harmonic complementaries. If we draw the segments between them, they *roughly* appear to be passing through a single point:

![](assets/complementarity_data.png)

This harmonic center, which I've fixed at $w=0.25 + 0.1i$ is a decidedly orange-ish chromaticity which happens to be, roughly, that of human skin (of various ethnicities). It looks like this at various lumas

![](assets/skincenter.png)

The significance of a match with skin tones is probably a rabbit hole best left for spare afternoons. Still, we can term this the skin center, defining a **skin frame**. For some reason, harmony is relative to the skin frame. After transforming ("boosting") to the skin frame, the definition of hues and their spacing is completely different from D65. These new, harmonic hues shrink the space dedicated to cool colors and expand the accuracy on warm hues, due to the inherently warm bias of the whitepoint. 

In the skin frame, D65 itself is not neutral anymore, and it is perceived to have a **blue** hue, with mild saturation. This means that, as such, it really is harmonic complementary to orange, and this is true at all luma. We can therefore conclude, somewhat humorously but with reasonable motivation, that **white is complementary to orange**, and they will indeed clash if the alignment is correct.

Here is the geodesic gradient between two (harmonic) complementaries: as expected, the gradient passes through the skin center, and consequently, the pairing is unpleasant in character.

![](assets/geograd_comp.png)

In a way, complementary pairs "cancel out" and blend into nothing, assuming we understand human skin as a default non-statement.

Instead, if we shift one of the colors slightly, the geodesic segment misses the skin center and the pairing improves considerably in appeal, as their geodesic blend acquires a very slight greenish-yellowish tint from skin tone:

![](assets/geograd_reso.png)

We will see shortly that such "near misses" are indeed what we should strategically target to employ colors in an aesthetically pleasing manner.


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

![](assets/skinframe_gamut.png)

Observe that D65 greys, including full black and full white, are exactly halfway between n and b, meaning their complementary axis is as far as possible from o and c. This is convenient for digital art as it allows the use of full black and white while minimizing the risk of clashing with intermediate tones, if o and c are preferred.

### Categorical Warp

![](assets/toneslices.png)

Categorical warp is a subtle but important effect in the skin frame which rests on the mismatch between literal, semantic color categories and hue-saturation as defined in a particular frame. It is important for an artist to be able to see colors in terms of their harmonic, *functional* properties, instead.

Semantic categories, which are usually defined in term of opponent process theory (i.e. blue *against* yellow, etc), break down when the saturation is lowered, as they get "warped" by the shifted reference frame in which we're working. The categorical warp is maximum for the pink-chartreuse axis, so let's imagine as an example that one wants to produce a desaturated yellow-green.

Instict would dictate one attempts to find a shade of greenish and slightly yellowish grey that would be categorized, in common parlance, as "chartreuse, but less intense". This is however incorrect in a functional sense, as the less intense color of the same hue as intense chartreuse is in fact much more yellow-orange. Since our perception adapts extremely well to the skin background, the effect is almost invisible in the diagram above, but actually picking the coloured tiles will reveal the very significant mismatch.

A slightly different, but related form of categorical mismatch occurs with primary blue (β),which is sharply located in the lavender (l) hues in the skin frame, but appears to be distinctly more blue than that, almost blue-violet. This is entirely due to categorical warp according to primary blue's excessive saturation compared to neighbouring lavender tones; functionally, it will still act like a lavender. 

## 14-Tone Intervals

Harmonic coloring is based on two principles:

1. Complementary tones are maximally dissonant and should always be avoided.
2. Contrast should be employed optimally to establish importance of elements.

These are forces in opposition: clearly complementary colors have the greatest contrast, yet the best choice for a dichromatic piece is an interval which is as large as possible without being dissonant. Since complementaries are separated by 7 steps, this optimal choice is a 6-step interval we call a **resonant**. Every tone has two resonants going clockwise or counter-clockwise. This is the 14 tones connected by resonance; since 6 is even (and so is 14), resonances have the same parity, thus there is an even cycle of resonants **voygnlp** and an odd cycle **chxtbmr**, analogous to the circle of fifths in music.

![](assets/star.png)

Working only with resonants is quite boring and static; some variation in relationship between the hues can expand the emotional range of a piece. While the contrast is lesser, an interval of 5 steps is also quite pleasant, and it could be called a **semi-resonant**. Here is all the possible intervals, sorted by increasing dissonance, with my interpretation:

|N | Interval | Quality | Left | Right|
|---|---|---|---|---|
|0 | Unison | Complex (see below) | <i class="ctile-13"></i> <i class="ctile-13"></i>|
|6 | Resonant | Mutual emphasis: maximum contrast without complementarity |   <i class="ctile-13"></i> <i class="ctile-5"></i> |   <i class="ctile-13"></i> <i class="ctile-7"></i> |
|5 | Semi-resonant | Similar to resonant, but with lesser contrast |   <i class="ctile-13"></i> <i class="ctile-4"></i> |   <i class="ctile-13"></i> <i class="ctile-8"></i> |
|3 | Adjoint | Adjoint tones work together towards the same goal |   <i class="ctile-13"></i> <i class="ctile-2"></i> |   <i class="ctile-13"></i> <i class="ctile-10"></i> |
|2 | Major Analogous | Slightly dissonant: complementary of semi-resonant |    <i class="ctile-13"></i> <i class="ctile-1"></i> |   <i class="ctile-13"></i> <i class="ctile-11"></i> |
|4 | Mediant | Odd and disorienting |   <i class="ctile-13"></i> <i class="ctile-3"></i> |   <i class="ctile-13"></i> <i class="ctile-9"></i> |
|1 | Minor Analogous | Dissonant: complementary of resonant |   <i class="ctile-13"></i> <i class="ctile-0"></i> |   <i class="ctile-13"></i> <i class="ctile-12"></i> |
|7 | Complementary | Highly displeasing | <i class="ctile-13"></i> <i class="ctile-6"></i> |



### Axial complementarity

*wip*

## Triads

The simplest non-trivial palette structure is the triangular field between three vertices, one of which is marked as the **key**, and the other two for the **base**. The key is the chromatic focal point of the piece and the purpose of the triad is to emphasize the key by establishing a context in which it is intensified by contrast.

The triad should be thought of as an arrow pointing to the key, it is inherently a directed and dynamical structure. The motion is from base to key, but also background to foreground, general to specific, and it is the physical guided motion of the eye as it absorbs the piece.

A triad can certainly not have any complementaries among its vertices. In addition, it should envelop the harmonic center, so the base vertices should be on opposite sides of the complementary to the key. Finally, since the key should be emphasized, the interval of the base must be smaller than those from the base to the key. This only leaves four possibilities:

![](assets/triads.png)

The key is positioned on the right, and I have further marked tones that have been excluded due to complementarity. 

The **ultra-narrow** triad is functional but has undesirable properties: the base is dissonant and the range is small. This palette is difficult to extend as further fields quickly drown out the main one. In addition, the symmetry with respect to the key axis makes it rather static.

The **narrow** triads are similar, but not identical, to Fletcher's. They are asymmetric and come in two chiral forms. The intervals are varied and harmonious: a resonant, a semi-resonant, and an adjoint. These palettes exhalt the key efficiently while also incorporating a great degree of playful secondary motion. They provide the range for rather surprising and dynamic artwork, and have the greater potential to be extended. Generally narrow triads are the most promising choice for main fields.

**Wide** triads are unweildy. The primality of the key is extremely weak (in fact, in the 12 tone system these are equilateral triangles), the base interval is a mediant which is generally unpleasant or uncertain in character, and finally there is symmetry. The range is vast, but this is potentially at the detriment of the piece.

### Tonnetz

Given what we know about harmony so far, the abstract color wheel with analogous tones adjacent is not a particularly convenient representation. We can lift a page from music theory again and construct a two-dimensional lattice (tonnetz) where each edge represents a particular consonant interval:

![](assets/lattice.png)

A portion of a Neo-Riemannian style lattice for the 14-tone system. The lattice is periodic. Dotted lines represent adjoints, solid line are resonants, and dashed lines are semi-resonants. Every triangle is a narrow triad, and each tone is the key for the triads directly above and below it.

For example, y is the bottom vertex of the narrow triad y/b/p, with y as the key, p as the base resonant, and b as the semi-resonant. However, it shares an edge with the narrow triad p/y/t, with now p as the key, y as the base resonant, and t as the semi-resonant. Joined together, the two triads assemble in a four-tone object including y,b,p and t, and with the new interval b-t which is a major analogous.

The tonnetz also places at maximum distance colors which are complementaries or mediants, providing a further rationalization of why they're perceived as displeasing -- they are "distant" in terms of the number of resonant steps that must be taken to bring one to the other. In this picture (which is to be understood as more poetic than literal), r and g are offending to the eye because the eye is invited to imagine, for example, that the relationship of g to r is that g is resonant to p, which is semi-resonant to t, which is resonant to r, but also the inverted path r -> x -> v -> g. The complexity and ambiguity of the relationship is too much for intuitive comprehension, which contrasts with the immediacy, for example, of the resonance between r and t.

The tonnetz is actually a finite structure, periodic in two directions. It is in fact a torus, tessellated into 28 triangles (the narrow triads) and with the 14 tones as vertices. It can be in fact visualized as a geometric torus:

![](assets/tonnetz_torus.png)

Or see also [as an interactive 3d model](https://cancrizans.github.io/hycol_tool/torus).

### Value

*wip*

### Triadic Painting

Here's a rough practical tutorial on how to lay out a palette for a simple piece in a single narrow triad.

Open the HYCOL tool [here](https://cancrizans.github.io/hycol_tool/). Disable all displays except for the hue wheel, and ensure the number of hue divisions is set at 14. Select the SKN whitepoint to center the view on the skin tone.

We now select a key tone; honey is a good choice. There are two possible triads with h as the key: h/b/p and h/m/n (we can see this from the tonnetz or directly from the color wheel). Let's go with h/b/p. The resonant is blue, which we remind is a vaguely purplish, late-afternoon sky class of blues, while the semi-resonant pink is bordeauxs/violet-reds/burgundies.

Going back to the interface, we add poles until we have three, then we start color picking them. Looking at the 3d view from above, I line up the colors with the corresponding hue lines:

![](assets/tooltut_0.png)

I've chosen very saturated tones for all three, and a rich dark burgundy for pink (if it seems strange, remember categorical shift!). The saturations and values are up to you; harmony only depends on the correct alignment in hue. However, you should try to avoid too desaturated tones, or very low separation in values. I check the values by rotating into side view, and choosing display value levels:

![](assets/tooltut_1.png)

The value range is decent for a soft piece. (If we want to use darker values, we might want to add a scotopic tail, but for now we'll keep it simple.) In this case, the key is the lightest pole, which is usually pretty easy to work with, but it's not mandatory. The key will be emphasized nevertheless by the harmonic information.

In the tool, I create a field, and select poles 0, 1, 2. The following palette is produced by geodesic interpolation of the three poles:

![](assets/tooltut_2.png)

Of course, I can still modify the poles and the field will be updated accordingly. Now as a first approximation, I can basically use any color from this triangle anywhere in the piece and it should look ok, but a better result can be obtained by employing the structure *functionally*. The first note I would make is to mark out the rough region of colors, shaded in red, that are close enough to the complementarity axis of the key to be potentially muddy. This region of the triad is not forbidden, but you want to resist the temptation to linger too much on it if barely. For example, when interpolating from the burgundy to the blue pole, you might wanna "speed up" halfway through to skip the dissonant peak.

![](assets/tooltut_3.png)

In general, you want to bias your palette use towards the edges of the field. In addition, you can divide your image into "background" (context) and "foreground" (subject), and for the background only use the base from burgundy to blue, without mixing in the key. For the foreground you deploy the key, but you can use the entire palette really, interleaving colors from the three ramps (and from inside the field as well) to create an interesting and multidimensional texture.

Note that this automatically gives us a value range hierarchy between background and foreground, since the former is limited in the range of values between the p and b poles, while the latter can go all the way from p to h (the fact that the geodesic interpolation is interpolating the luma at the same rate as the chroma is what guarantees this).

## Polytriadic Structure

*wip*

## Combinatoric Modes and Non-functional Approach

As an alternative to working with triads, there is potential for an eclectic and simpler color picking system in which tones are selected exclusively to avoid dissonance but with no semantic intentionality. This coloring style is potentially well suited for comics and illustration, and it provides large and rich non-offending palettes, though color does tend to be employed in a more literal sense, losing its emotional charge.

This, in my opinion, mirrors non-functional harmony in music: colors do not have a color-theoretic function, but are simply meaningless expressive units that the artist can assemble freely.

In a non-functional palette, blending fields should be avoided altogether, preferring instead to pick colors from a discrete set of hues, in turn chosen to prevent dissonance. Generally, one chooses from the set the hue that best approximates the intended color (typically literal: trees are green, sky is blue etc.).

The 14-tone system allows a maximum of 7 colors without complementaries amongst themselves. There is a total of 128 of these maximal sets, but up to rotations there are only 10 distinct possibilities, which we term combinatoric modes:

![](assets/combinatoric_modes.png)

Note that modes I and II are chiral and are mirror images of eachother, all other modes are symmetric with respect to an axis. Mode VIII is simply the resonant cycle of a given parity, as seen before.

Mode III and VII might be excluded on the ground of being severely unbalanced.

Past that, the choice of mode is informed primarily by which secondary dissonance, after complementaries, is deemed more problematic: the minor analogous or the mediant. Mode VIII for example has no minor analogies but many mediants. Mode X is particularly nice, with only two minor analogies and only one mediant.

Let's try to build a palette in mode X just as an example. Let's say that the point in the bottom left through the axis of symmetry is <i class='ctile-10'>l</i> - we rotate the mode X schema over the color wheel and then pick hues accordingly: going counter-clockwise, after l we skip two places, then pick r, then skip one place, then pick o, and so on, to obtain:

<i class='ctile-10'>l</i> <i class='ctile-13'>r</i> <i class='ctile-1'>c</i> <i class='ctile-2'>o</i> <i class='ctile-4'>y</i> <i class='ctile-5'>x</i> <i class='ctile-7'>t</i>

This palette is "safe to pick", with the right caveats:

- Low-saturation (with saturation as in the skin frame!) colors should be limited, as is having too many colors for the same tone.
- The use of the tones should be "atonal" in the sense that the artist should strive specifically to avoid explicit functional relationships that might undermine the intention of the piece. We do not want to accidentally be using any tone as a triadic key, for example.