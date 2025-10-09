# Droplet

Droplet is a program for computing mathematical constants using BBP-style formulas of the form:

$$
\alpha = \sum_{k=0}^{\infty}\left[\frac{1}{b^k} \sum_{j=1}^n \frac{a_j}{\left(nk+j\right)^s}\right]
$$

BBP-style formulas allow for efficient computation of an arbitrary digit $d$ in base $b$ using the following observation:

$$
\mathrm{frac}(b^d\ \alpha) =
    \mathrm{frac}\left(
        \sum_{k=0}^{d}\left[\sum_{j=1}^n a_j\frac{b^{d-k}\ \mathrm{mod}\ \left(nk+j\right)^s}{\left(nk+j\right)^s}\right] +
        \sum_{k=d+1}^{\infty}\left[\sum_{j=1}^n \frac{b^{d-k}\ a_j}{\left(nk+j\right)^s}\right]
    \right)
$$

The first sum can be computed efficiently using modular arithmetic, and the second sum only requires a few terms to be accurate enough for calculation of a single digit (or group of digits).

## Copyright & Licensing

Droplet is licensed under the MPL-2.0 License.

Copyright 2025 Ethan Jaszewski