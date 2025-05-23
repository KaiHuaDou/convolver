from PIL import Image

image = 0
try:
    image = Image.open('initial.png')
except:
    image = Image.open('initial.jpg')

result = image.convert('RGBA')
result.save('initial.png', progressive=False)
