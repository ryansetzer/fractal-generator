import imageio.v2 as imageio
import os


images = []
files = os.listdir("./images")
files.sort()

for file in files:
    filename = f"images/{file}"
    images.append(imageio.imread(filename))
imageio.mimsave("julia.gif", images, duration=50)
