from PIL import Image

image = 0
try:
    image = Image.open('input.png')
except:
    image = Image.open('input.jpg')

result = image.convert('RGBA')
result.save('input.png', progressive=False)
