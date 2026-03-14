# Droplet

Droplet is a program for computing mathematical constants using base-$2^b$ BBP-style formulas of the form:

$$
\begin{align}
\alpha = \sum_{n=0}^{\infty}
\left[
    \left(\frac{\pm 1}{2^b}\right)^n
    \frac{P(n)}{Q(n)}
\right]
\end{align}
$$

Where $b \in \mathbb{Z}$ and $P$ and $Q$ are polynomials with integer coefficients. BBP-style formulas allow for efficient computation of an arbitrary digit $d$ in base $2^b$ using the following observation:

$$
\begin{align}
\mathrm{frac}((2^b)^d\ \alpha) =
    \mathrm{frac}\left(
        \sum_{n=0}^{d}
        \left[
            \frac
                {(\pm 1)^n \cdot (2^b)^{d-n} \cdot P(n)}
                {Q(n)}
        \right]
        +
        \sum_{n=d+1}^{\infty}
        \left[
            \frac
                {(\pm 1)^n \cdot P(n)}
                {(2^b)^{n-d} \cdot Q(n)}
        \right]
    \right)
\end{align}
$$

The first sum can be computed efficiently using modular arithmetic, and the second sum only requires a few terms to be accurate enough for calculation of a single digit (or group of digits).

## Parametrization

There are a number of different parametrizations of BBP-type formulas that all have positive and negative attributes. The one that is arguably most well known is:
$$
    P(s,b,n,a)=
        \sum_{k=0}^{\infty}
            \frac{1}{b^k}
            \sum_{j=0}^{n}\frac{a_j}{\left(kn+j\right)^s}
$$
A large number of formulas that can be expressed this way has been published (and updated periodically) by Dan Bailey.[^1]

However, this parametrization both limits what formulas can be expressed and presents challenges to efficient implementation. First, it arbitrarily limits the numerator to constants when any polynomial is easily accomodated. Second, allowing for arbitrary bases makes implementing the computation of the second summand in $\mathrm{Eq.}\ 2$ more complex for arbitrary-precision calculation.

The parametrization chosen for **droplet** limits the base to powers of two to more easily support calculating large numbers of digits with one calculation. That said, all currently known BBP-style formulas for pi, as well as the formulas for a large number of other constants are expressed in power-of-two bases already.

## Testing

### Pi Day 2026

On $\pi$ day 2026, I tested `droplet` by computing 314 hex digits of pi at hex digit offset 3,141,592,650. The calculation took 226.70s on my Ryzen 9 3900X processor. The digits are below, and have been verified to be correct.

```
00: e26d0cc081590def bc8f0088cb533c9f
02: c988bb418050cbb5 121ec61f9f4e7278
04: de896e5dda293e65 8ad3a947debfade3
06: 57afaeb9daaed8f1 fd80010977d2f2f4
08: e086ad906d9f3cbc 41af114cb890bdb2
10: e7a36d245345907c f4d4c0c674f1c22e
12: acb18073356a6fc2 0cbe67d396a7a937
14: 68bc8f2f59cf9a7d 448f0fe10e028981
16: e3b35ed7aada9948 ff4e8de652f50e7c
18: fc3e6723ba73f7d8 3d808ee3f0
```

This is a reformatted result from the `compute` binary.

## References

[^1]: https://www.davidhbailey.com/dhbpapers/bbp-formulas.pdf

## Copyright & Licensing

Droplet is licensed under the MPL-2.0 License.

Copyright 2025 Ethan Jaszewski