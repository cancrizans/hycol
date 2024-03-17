from colormath.color_objects import XYZColor, sRGBColor, LCHabColor
from colormath.color_conversions import convert_color
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.patches import Rectangle


for L in (15,30,50,70):
    fig,axs = plt.subplots(4,4,figsize=(3,3))
    # assert isinstance(ax,plt.Axes)

    h = np.linspace(0,360,16)

    for i,hue in enumerate(h):
        ax : plt.Axes = axs[i//4,i%4]

        for chroma in np.flip(np.linspace(30,100,20)):
            col : sRGBColor = convert_color(LCHabColor(L,chroma,hue,illuminant='d65'),sRGBColor)
            rgb = col.get_value_tuple()

            ingamut = all(0.<=c<=1. for c in rgb)
            if ingamut:
                break
        
        ax.add_patch(Rectangle((0,0),1,1,color = rgb))
        ax.set_axis_off()
        # ax.set_facecolor(rgb)
        ax.set_xlim(0,1)
        ax.set_ylim(0,1)

    grey = convert_color(LCHabColor(L,0,0,illuminant='d65'),sRGBColor).get_value_tuple()
    fig.patch.set_facecolor(grey)

    fig.savefig(f'assets/hk_{L:d}.png')

    


