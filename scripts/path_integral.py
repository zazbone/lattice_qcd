import numpy as np
import matplotlib.pyplot as plt


N = 8
T = 4
a = T / N
A = np.power(1 / (2 * np.pi * a), N // 2)

def sqrpot(x):
	return np.power(x - 1, 2)

def Action(x):
	return np.sum(sqrpot(x[1:]) - 0.5 * (x[1:] - x[:-1])) * a

def incr_path(x):
	for i in range(N - 3, 0, -1):
		if x[i] >= T:
			x[i] = 0
			x[i + 1] += a
	x[1] += a

def integrate(x0, xf):
	x = np.zeros(N)
	x[0] = x0
	x[-1] = xf
	I = 0
	while True:
		if x[-2] > T: break
		incr_path(x)
		I += np.exp(-Action(x))
	return A * I

n = 10
xf_axis = np.linspace(0.1, T, n)
H = np.zeros(n)

for i, xf in enumerate(xf_axis):
	H[i] = integrate(0, xf)

plt.plot(xf_axis, H)

plt.show()