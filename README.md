Given the position of an orbital camera in polar coordinates: $`(\theta \phi r)`$

We want to compute the position and the up-vector in cartesian coordiantes:

$`{position}=\left[\begin{matrix}r \sin{\left(\theta \right)} \cos{\left(\phi \right)}\\r \sin{\left(\phi \right)} \sin{\left(\theta \right)}\\r \cos{\left(\theta \right)}\end{matrix}\right]`$

The up vector always points towards the north pole of the sphere the camera is orbiting. In a Y-up coordinate system this means towards
$`{Y}=\left[\begin{matrix}0 & 1 & 0\end{matrix}\right]`$.



$`{up}=\left[\begin{matrix}- \sin{\left(\phi \right)} \sin^{2}{\left(\theta \right)} \cos{\left(\phi \right)}\\\sin^{2}{\left(\theta \right)} \cos^{2}{\left(\phi \right)} + \cos^{2}{\left(\theta \right)}\\- \sin{\left(\phi \right)} \sin{\left(\theta \right)} \cos{\left(\theta \right)}\end{matrix}\right]`$

Do note that the _up_ vector is **not normalized**!
