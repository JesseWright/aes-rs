fn GaloisMultiply(mut a: char, mut b: char) -> char
{
    a &= 0xFF;
    b &= 0xFF;

    let (s, q);
    let z = 0;

    s = LTABLE[a] + LTABLE[b];
    s %= 255;

    /* Get the antilog */
    s = ATABLE[s];

    /* Now, we have some fancy code that returns 0 if either
       a or b are zero; we write the code this way so that the
       code will (hopefully) run at a constant speed in order to
       minimize the risk of timing attacks */

    q = s;
    s = if a == 0 { z }
        else { q };

    if b == 0
    { s = z; }
    else
    { q = z; }

    return (char) (s & 0xFF);
}
