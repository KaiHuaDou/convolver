from PIL import Image

image = Image.open('initial.png')
rgba_image = image.convert('RGBA')
rgba_image.save('initial.png', progressive=False)