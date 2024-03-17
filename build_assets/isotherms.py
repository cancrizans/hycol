from hycol_py import convert_srgb_hycol,convert_hycol_srgb
import numpy as np
import matplotlib.pyplot as plt

Ls = (45,55,65,75,85)
fig,axs=plt.subplots(1,len(Ls),sharey=True,figsize=(14,5))
axs[0].set_ylabel("Temperature")

for ax,L in zip(axs,Ls):
    t = np.linspace(-1.2,1.7,32)
    p = np.linspace(-2.3,2.2,40)

    tt,pp = np.meshgrid(t,p)

    z = np.exp(tt)*pp + np.exp(tt)*1j

    w = (z-1j)/(z+1j)

    rgb = np.array(convert_hycol_srgb([(L,wv) for wv in w.flatten()])).reshape(w.shape+(3,))

    mask = np.sum(rgb,axis=-1) > 0.01
    rgb = np.clip(rgb,0.0,1.0)

    ttm = tt[mask]
    ppm = pp[mask]
    rgbm = rgb[mask]

    print(tt.shape)
    print(rgb.shape)


    assert isinstance(ax,plt.Axes)

    ax.scatter(ppm,ttm, c = rgbm.reshape(-1,3), marker='s')

    ax.set_facecolor(convert_hycol_srgb([(80,0)])[0])
    ax.get_xaxis().set_visible(False)
    ax.annotate(f"L={L}",(0,0),ha='center',va='center')

# axs[1].set_xlabel('Distance from thermal axis')
    
fig.savefig('assets/isotherm_strips.png',bbox_inches='tight')