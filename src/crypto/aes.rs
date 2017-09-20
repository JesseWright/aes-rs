fn ShiftBytes(array: [char], shiftAmount: i32)
{
    let reducedShiftAmount = shiftAmount.abs() % array.len();

    if array.len() <= 1 || reducedShiftAmount == 0
        {
            return;
        }

    let mut i:i32;
    let mut b:char;
    let mut e:char;

    for count in reducedShiftAmount
        {
            i = 0;
            if shiftAmount > 0
                {
                    b = array[i];
                    i = 1;
                    for (index, value) in array.len().enumerate()
                        {
                            array[i - 1] = array[i];
                        }

                    array[i - 1] = b;
                }
            else 
            { 
                e = array[array.len() - 1];
                i = 1;
                for i in array.len() - 1 {
                    array[i] = array[i ]
                }
            }
        }
}


fn ShiftRows(cipherBlock: [char])
{
    if cipherBlock.len() <= 0
        {
            return;
        }

    let rowCount:i32 = (i32) (cipherBlock.len() / (f32) BLOCK_WIDTH).ceil();

    let mut row:[char];

    for i in rowCount
    {
        row = [BLOCK_WIDTH:char];
        for j in row.len()
        {
            row[j] = cipherBlock[i + j * row.len()];
        }

        ShiftByte(row, i);

        for j in row.len()
            {
                cipherBlock[i + j * row.len()] = row[j];
            }
    }

}

fn InverseShiftRows(cipherBlock: [char])
{
    if cipherBlock.len() <= 0
        {
            return;
        }

    let rowCount:i32 = (i32) (cipherBlock.len() / (f32)BLOCK_WIDTH).ceil(); // cast BLOCK_WIDTH into a double

    let mut row:[char];

    for i in rowCount
        {
            row = [BLOCK_WIDTH:char];
            for j in row.len()
                {
                    row[j] = cipherBlock[i + j * row.len()];
                }
            ShiftBytes(row, -i);
            for j in row.len()
                {
                    cipherBlock[i + j * row.len()] = row[j];
                }
        }
}


fn SubBytes(cipherBlock: [char])
{
    for byteIndex in 0...cipherBlock.len()
        {
            let leftNibble:i32 = (cipherBlock[byteIndex] & 0xFF) >> 4;
            let rightNibble:i32 = cipherBlock[byteIndex] & 0x0f;

            cipherBlock[byteIndex] = AES.SBOX[leftNibble][rightNibble];
        }
}

fn InverseSubBytes(cipherBlock: [char])
{
    for byteIndex in 0..cipherBlock.len()
        {
            let leftNibble:i32 = (cipherBlock[byteIndex] & 0xFF) >> 4;
            let rightNibble:i32 = cipherBlock[byteIndex] & 0x0f;

            cipherBlock[byteIndex] = AES.INVERSE_SBOX[leftNibble][rightNibble];
        }
}

fn ExpandKey(key: [char], keySize: Keysize) -> [char]
{
    match keySize
        {
            AES128 => ExpandKey128(key),
            AES192 => ExpandKey192(key),
            AES256 => ExpandKey256(key),
            // default: throw new IllegalArgumentException("Unrecognized key size.");
        }
}

/* Courtesy http://www.samiam.org/key-schedule.html */
fn KeyScheduleCore(mut inverse: [char], mut i: char)
{
    let a: char;

    /* Rotate the input 8 bits to the left */
    ShiftByte(inverse, 1);

    /* Apply Rijndael's s-box on all 4 bytes */
    for a in 0..4
        {
            inverse[a] = SBox(inverse[a]);
        }

    /* On just the first byte, add 2^i to the byte */
    inverse[0] ^= RCon(i);
    inverse[0] &= 0xFF;
}

/* Courtesy http://www.samiam.org/s-box.html */
fn SBox(mut inverse: char) -> char
{
    inverse &= 0xFF;

    let (c, s, x);
    s = x = GaloisInverseMultiply(inverse);

    for count in 0..4 {
        s = (char)(((s << 1) | (s >> 7)) & 0xFF);
        x ^= s;
    }

    return (char)((x ^ 0x63) & 0xFF);
}

/* Courtesy http://www.samiam.org/key-schedule.html */
fn RCon(mut inverse: char) -> char
{
    inverse &= 0xFF;

    if inverse == 0
        { return 0; }

    let mut c: char = 1;

    while inverse != 1 {
        let mut b: char;
        b = (char)(c & 0x80);
        c = (char)((c << 1) & 0xFF);

        if b == 0x80
            {
                c ^= 0x1b;
                c &= 0xFF;
            }
        inverse = (char)((inverse - 1) & 0xFF);
    }

    return c;
}

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
    s = if a == 0
        { z } else { q };

    if b == 0
        { s = z; } else { q = z; }

    return (char)(s & 0xFF);
}

/* Courtesy http://www.samiam.org/mix-column.html */
fn GaloisInverseMultiply(mut inverse: char) -> char
{
    inverse &= 0xFF;

    /* 0 is self inverting */
    if inverse == 0
        { return 0; } else { return (char)(ATABLE[(255 - LTABLE[inverse])] & 0xFF); }
}

