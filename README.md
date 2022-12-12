# Shamir's Secret Sharing

[This fine article](https://medium.com/bootdotdev/shamirs-secret-sharing-step-by-step-42eb8cafa102) provides a helpful intuition.

*k* points uniquely define a polynomial of degree *k - 1*, but allow for an infinite number of polynomials of degree *k*.

- 1 point? Infinity lines, x = 3 or whatever
- 2 points? You have a line, x = 3. And only that line! But infinity quadratic curves, *y = x + x^2*
- 3 points? You have uniquely defined a quadratic curve, *y = a + x + x^2*


To summarize the article, if you have a family of 4, who want to reconstruct the private key that controls their wallet with just 3 shares, you can do something like this.

The secret will actually be a very long number in hexadecimal, 0xfa....b9d or something but imagine it is just 1954.

We want *n* shares, here 4, and a threshold of *k*, here 3.

We want to build an equation that looks like $$y = a_0 + a_1 x + a_2 x^2$$.

We'll do $$y = secret + a_1 x + a_2 x^2$$. If we choose a random *a_1* and *a_2*, we have made a random curve.

In addition, for each share, you can choose a random *x* - as long as every share uses the same constants/coefficients (the *a*'s) then they will all sit on the same line. And if they all sit on the same line, we can put it together to find the original secret.

This will be hard for any attacker to crack!


![image](https://miro.medium.com/max/1400/0*BGfoI3PKDmslrex1.webp)

## High Level Pseudocode

```
function ShamirSecretSharing(secret, n, k)
  if n < k
    return error
  end if

  // Generate a random polynomial of degree k-1
  coefficients = random_polynomial(secret, k)

  // Generate n points on the polynomial
  shares = []
  for i = 1 to n
    shares[i] = (i, evaluate_polynomial(coefficients, i))
  end for

  return shares
end function

function random_polynomial(secret, k)
  // Generate a list of k random coefficients
  coefficients = []
  for i = 0 to k-1
    coefficients[i] = random_coefficient()
  end for

  // Set the constant term to the secret
  coefficients[0] = secret

  return coefficients
end function

```

## Reassembling the Secret
In practice, you might keep your own BTC private key, and distribute the shards among trusted friends and family.

You could in theory destroy the original key, but you might as well use a multisig in that case.
