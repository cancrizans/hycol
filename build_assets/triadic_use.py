import matplotlib.pyplot as plt
from matplotlib import patches
import matplotlib.collections as mc
import numpy as np

fig,ax = plt.subplots()
assert isinstance(ax,plt.Axes)
ax.set_axis_off()

pts = (
    (1,0),
    (-np.cos(2*np.pi/7),-np.sin(2*np.pi/7)),
    (-np.cos(np.pi/7),np.sin(np.pi/7)),
)

ax.add_patch(patches.Arc(pts[0],0.2,.2,theta1=90,theta2=270,color='k',fill=None))

for pt in pts:
    ax.add_patch(patches.Circle(pt,0.05,color='k'))

ax.plot(*np.array(pts + (pts[0],)).T,color='k')

ax.plot((-1,1),(0,0),ls=':',c='k')

ax.add_patch(patches.Circle((0,0),0.03,color='k'))
ax.annotate('SKN',(0,0))


ax.set_ylim(-1.2,1.2)
ax.set_xlim(-1.2,1.2)
ax.set_aspect('equal')


dl = .8
th = np.linspace(0,2*np.pi,200)
r = ( dl + (1-dl)*np.power(1-np.abs(np.cos((th-np.pi)/2)),14) ) 

# print(r)
plt.plot(r*np.cos(th),0.3*r*np.sin(th),c='k')

plt.show()