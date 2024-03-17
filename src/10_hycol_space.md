---
title: The HYCOL color space
---

HYCOL is a color space I've designed which represents colors as points in a hyperbolic geometry. Specifically, a color is represented by its luma (aka luminance, or value), and chroma (aka chromaticity), and chroma is understood as a point of the two-dimensional hyperbolic plane, $\mathbb{H}^2$, while luma lies on a half-line. HYCOL is constructed to have, to reasonable accuracy, the following useful properties:

- **Perceptual uniformity**, meaning that the perceived color difference, or perceived total variation in a gradient, should be proportional to the geometric distance according to the metric.
- **Perceived luminance and chromaticity constancy**, meaning that colors at the same luma should be perceived as equally luminous, and colors at the same chroma at different lumas should be perceived as roughly different shades of the same hue and saturation.
- **Simple and highly symmetric geometry**, in our case $\mathbb{H}^2$, to provide clear and efficiently computable geometric formulations of color theory concepts.

I want the enforcement of these principles to only really be realized within the sRGB gamut and for LCD screens, which is a small subset of the full gamut of physically realizable colors, and then also only really within the range of variation in digital displays and viewing conditions around the sRGB standard.

This colorspace is assembled by using several known insights from the literature in combination. These sources are:

1. The **1976 CIELAB color space**, which is an early attempt at a perceptually uniform Euclidean color space.
2. *Cui, G., Luo, M.R., Rigg, B., Roesler, G. and Witt, K. (2002), Uniform colour spaces based on the DIN99 colour-difference formula. Color Res. Appl., 27: 282-290.*, where a set of CIELAB variations, **DIN99x**, are presented, which improve on CIELAB's luma uniformity and problematic hue shifts in the blue region.
3. *Ivar Farup, "Hyperbolic geometry for colour metrics," Opt. Express 22, 12369-12378 (2014),* which evidences how the phenomenon of hue superimportance can be properly addressed by converting the chroma part of standard color spaces into a hyperbolic plane; identifying the best match to the data in **DIN99c** with a specific hyperbolic radius.
4. *Fairchild, M.D. and Pirrotta, E. (1991), Predicting the lightness of chromatic object colors using CIELAB. Color Res. Appl., 16: 385-393.*, which propose a functioning model for the strength of the **Helmholtz-Kohlrausch effect**.

It's well known that color spaces based on addition of three primaries, like forms of RGB (the difference with sRGB is immaterial for the present discussion), are highly non-uniform and unsuitable for our purposes. CIELAB, CIELUV, CIECAM02, etc. and similar do improve considerably on these aspect as best as possible while mantaining a Euclidean metric. However, they do not account correctly for two phenomena that I think are very important for color perception in digital art: **hue superimportance** and the **Helmholtz-Kohlrausch effect**. In addition, the inaccuracy of the metric has further negative implications in the colorspace's ability to perform blending and illumination changes in a way that matches up with the expectation of an artist and a consumer of art.

### Hue Superimportance

Hue superimportance is the observation that for saturated colors variations in hue produce a perceived variation which is greater than expected. We can make this more precise. In the wheel below, the spokes are all of equal length, in the sense that from grey up to the rim of the wheel we perceive roughly the same total variation as the color saturates. This total variation is the radius $R$ of the wheel.

![](assets/wheel_euclidean.png)

If Euclidean geometry applied, then, the circumference itself should have a length $2\pi R$, and we should perceive colors to be varying around at the same speed as in the spokes, because the literal length of the Euclidean circle on our hopefully flat screen is supposedly proportional to that variation. Instead, if you pay close attention, you'll see that the change is quite a bit faster, almost twice as fast. We could draw a more accurate wheel like this:

![](assets/wheel_wavy.png)

This fact, which is sometimes informally stated as "there are more than 360° worth of hue at high saturation", implies that the geometry of the chroma plane is not flat, and since it's always a super-importance and never a sub-importance, it must have negative curvature.

### Helmholtz-Kohlrausch effect

The HK effect is an odd perceptual phenomenon where certain very saturated colors appear brighter than they actually are.  For example, the following colours in each image have all the same CIELAB luminance, which is the same as the grey in the background:

![](assets/hk_15.png)
![](assets/hk_30.png)
![](assets/hk_50.png)
![](assets/hk_70.png)

Most existing colorspaces choose not to account for the HK effect, as it can be unreliable, has psychological components, and is most importantly dependent on the quality of the observed surface.

Some saturated colors look very luminous and "pop" off the background, even though they are at the same brightness in physiological terms. The HK effect is strong and consistent for digital displays, and I found it necessary to take into account since it affects greatly the perception of values, whose control is fundamental for illustration. Failure to account for HK can result in less legible designs. It is somewhat less relevant for traditional paints, in part because they can often not achieve the same degree of purity (saturation) as some corners of additive RGB, and in part because the surface quality of paint is distinct from an LCD screen. 

The HK effect increases with saturation, and it is stronger for darker luminances (an already bright color is brightened less). Its dependence on hue is very complicated and the subject of several studies, but generally it's accepted that it's always positive - saturation can never make a color look brighter than it objectively is, and that it's most intense on blues, rather intense on magentas all the way to reds, and has a narrow dip in strength around yellow, being mostly non-existent for yellow hues.

## Details of the Mapping

### Chroma to DIN99c

Let a colour be expressed in CIELAB space as luma $L^*$ and chromaticity $a^*,b^*$. 

First, we convert chroma to the H99 space:

$$\tilde{b} = 0.94\cdot b^*$$

$$C_{\mathrm{99c}} = 23 \ln\left(1+0.066 \sqrt{(a^*)^2 + \tilde{b}^2}\right)$$

$$h_{\mathrm{99c}} = \operatorname{atan2}(\tilde{b}/a^*)$$

### Helmholtz-Kohlrausch Correction

Then, we perform correction of luminance to account for the Helmholtz-Kohlrausch effect, using the Pirrotta-Fairchild model. This is performed in DIN99c chroma coordinates instead of CIELAB, with minimal impact.

First we compute the strength of the HK effect according to hue, with the characteristic dip around yellow.

$$f_1(h_\mathrm{99c}) = 0.116 \left|\sin\left(\frac{h_\mathrm{99c}-90°}{2} \right)\right| +0.085$$

Then we have the corrected luma:

$$L^{**} = L^* + \mu - \nu L^*\,,\quad \mu = 2.5\,f_1 C_{\mathrm{99c}}\,,\quad \nu = 0.025 f_1 C_{\mathrm{99c}}$$

### Luma to DIN99c

Finally, we perform the conversion to DIN99c luma, but employing the HK-corrected luma as input.

$$l = l_{\mathrm{99c}} = 317.65 \ln(1+0.0037 L^{**})$$

At this point, we have $(l,C_{\mathrm{99c}},h_{\mathrm{99c}})$ polar DIN99c coordinates with HK-effect correction.

### Hyperbolicization

We then perform "hyperbolicization" by interpreting $C_{\mathrm{99c}}$ as a geodesic radius in the hyperbolic plane:

$$\rho = C_{\mathrm{99c}} / R$$

$R = 28.6$ is the radius of curvature of the chromaticity plane in units of DIN99c chromaticity, and thus $\rho$ is the geodesic distance of the colour from the pure grey (D65) of the same luminance. Therefore, finally:

$$w_0 = \tanh\left(\frac{\rho}{2}\right) e^{i(h_{\mathrm{99c}} + \theta_T)}$$

where the thermal angle $\theta_T = 40°$ is such that the warmest hue (an orange-ish red) is along the positive real axis.

$$w_0$$ lies in the disk $$\vert w_0\vert < 1$$ and represents a point in the Poincaré disk model of the hyperbolic plane. The colour space thus defined through $$(l,w)$$ with the colour distance geometry $$\mathbb{R}^+\times \mathbb{H}^2$$, or precisely

$$dE^2 = dl^2 + R^2\left(\frac{4|dw_0|^2}{(1-|w_0|^2)^2} \right)$$

is hypothesized to be perceptually uniform.

<!-- ### Whitepoint temperature shift

The space is highly symmetric which makes the whitepoint arbitrary. For colours on the real axis $w_0 \in \mathbb{R}$, the temperature is $T = 2 \tanh^{-1}(w_0)$ (later we will extend the definition to all colours). We perform a shift to a warmer whitepoint as such:

$$w = \frac{\ch{\frac{T}{2}}w_0 - \sh{\frac{T}{2}}}{-\sh{\frac{T}{2}}w_0 + \ch{\frac{T}{2}}}$$

with the temperature of the warm whitepoint $N_W$ set to $T_W = 0.25$. This is an isometry, so $w$ also sees the same metric as $$w_0$$. -->

## Summary and Gamuts

The final $$(l,w)$$ tuple constitute our hyperbolic model of colours. Only a small fraction of the domain is actually occupied by a given gamut. Our interest is specifically in the sRGB gamut. We assume the standard sRGB primary specifications and the D65 whitepoint for conversion sRGB -> CIEXYZ -> CIELAB.

The range of $$l$$ is from 0 (sRGB black) to 100 (sRGB white). $$w$$ lies in the unit disk, but in practice for reasonable whitepoints most of the gamut is within $$\vert w\vert<0.75$$. At fixed $$l$$ we have a notion of *slice gamut*, which is always contained in the **full gamut**, which is the region in $$\mathbb{H}^2$$ of all possible values of $$w$$ for colours in the sRGB gamut. 

In the diagram, a few slice gamuts are displayed in the Poincaré disk model of chroma space, bounded by the outline of the full gamut.

![](assets/gamuts.png)

The full gamut is the shape, vaguely circular, outlined by six smooth segments. This boundary is the image of the hexagon of the sRGB primaries and secundaries in the order red -> yellow -> green -> cyan -> blue -> magenta -> red. The images of these segments are not themselves straight segments, and do bulge out considerably.

It would be misleading to display the *interior* of the full gamut all at once on the same plane, because no single value of luma covers it all simultaneously, and variation in luminance will affect colour difference and thus deform the metric.

## Inverse transformation

The transformation CIELAB -> hyperbolic model is composed of individually invertible elements. Schematically, starting from a hyperbolic colour in the warm frame:

* The radial distance is "de-hyperbolicized" and thermal angle removed to recover DIN99c chroma.
* HK-corrected luma is recovered from DIN99c by inverting the mapping.
* Uncorrected CIELAB luma is recovered by inverting the (linear) HK-correction.
* DIN99c chroma is easily mapped back to CIELAB chroma