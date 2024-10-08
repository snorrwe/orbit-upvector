#!/usr/bin/env python3

from sympy import *

init_printing()

# given
theta, phi, r = symbols("theta phi r")


x = sin(theta) * cos(phi)
y = sin(theta) * sin(phi)
z = cos(theta)


forward = -Matrix([[x, y, z]]).T
position = forward * -r


right = forward.cross(Matrix([[0, 1, 0]]).T)
up = -forward.cross(right)
print("cartesian position")
pprint(position)
print("Up vector")
pprint(up)
