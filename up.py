#!/usr/bin/env python3

from sympy import *

init_printing()

# given
theta, phi, r = symbols("theta phi r")
forward, up, right = symbols("forward up right")


def pos(theta, phi, r):
    x = sin(theta) * cos(phi) * r
    y = sin(theta) * sin(phi) * r
    z = cos(theta) * r

    return Matrix([[x, y, z]]).T


forward = -pos(theta, phi, r)
right = forward.cross(Matrix([[0, 1, 0]]).T)
up = forward.cross(right)
print(forward)
print(right)
print(up)
