import cv2

image = cv2.imread('steganography.png', cv2.IMREAD_COLOR)
image = image[:, :, 2]

bitImages = []
for bit in range(8):
    bitMask = 1 << bit
    bitPlane = cv2.bitwise_and(image, bitMask)
    bitImages.append(bitPlane)

for i, bitImage in enumerate(bitImages):
    cv2.imwrite(f'bit_{i}.png', bitImage)