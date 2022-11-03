import numpy as np
import matplotlib.pyplot as plt


def gaussian(x, mu, sigma):
	norm = 1 / np.sqrt(2 * np.pi) / sigma
	diff = 0.5 * np.power(x - mu, 2) / sigma
	return norm * np.exp(-diff)

def action(field):
	dtphi = 0#field[1:, :] - field[:-1, :]
	dxphi = 0 # field[:, 1:] - field[:, :-1]
	phi2 = np.power(field[1:, :], 2)
	return 0.5 * a / N * (np.sum(np.power(dtphi, 2))  - np.sum(np.power(dxphi, 2)) - np.sum(phi2))

L = 10
N = 64
a = 2 / N
Niter = 1 << 15

field = np.random.uniform(0, L, (N, N))
field[0, :] = gaussian(np.linspace(-L, L, N), 0, 1)

passed = 0
mean_prob = 0
for _ in range(1 << 14):
	old = field
	new = field
	new[1:, :] = new[1:, :] + np.random.normal(0, 0.4, (N - 1, N))
	ds = action(old) - action(new)
	proba = np.exp(ds)
	mean_prob += proba
	alpha = np.random.uniform(0, 1, 1)
	if alpha < proba:
		passed += 1
print(100 * passed / Niter)
print(mean_prob / Niter)
#plt.plot(np.linspace(-L, L, N), field[0])
plt.imshow(field)
plt.show()

