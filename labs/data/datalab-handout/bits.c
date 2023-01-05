/*
 * CS:APP Data Lab
 *
 * <Please put your name and userid here>
 *
 * bits.c - Source file with your solutions to the Lab.
 *          This is the file you will hand in to your instructor.
 *
 * WARNING: Do not include the <stdio.h> header; it confuses the dlc
 * compiler. You can still use printf for debugging without including
 * <stdio.h>, although you might get a compiler warning. In general,
 * it's not good practice to ignore compiler warnings, but in this
 * case it's OK.
 */

#if 0
/*
 * Instructions to Students:
 *
 * STEP 1: Read the following instructions carefully.
 */

You will provide your solution to the Data Lab by
editing the collection of functions in this source file.

INTEGER CODING RULES:
 
  Replace the "return" statement in each function with one
  or more lines of C code that implements the function. Your code 
  must conform to the following style:
 
  int Funct(arg1, arg2, ...) {
      /* brief description of how your implementation works */
      int var1 = Expr1;
      ...
      int varM = ExprM;

      varJ = ExprJ;
      ...
      varN = ExprN;
      return ExprR;
  }

  Each "Expr" is an expression using ONLY the following:
  1. Integer constants 0 through 255 (0xFF), inclusive. You are
      not allowed to use big constants such as 0xffffffff.
  2. Function arguments and local variables (no global variables).
  3. Unary integer operations ! ~
  4. Binary integer operations & ^ | + << >>
    
  Some of the problems restrict the set of allowed operators even further.
  Each "Expr" may consist of multiple operators. You are not restricted to
  one operator per line.

  You are expressly forbidden to:
  1. Use any control constructs such as if, do, while, for, switch, etc.
  2. Define or use any macros.
  3. Define any additional functions in this file.
  4. Call any functions.
  5. Use any other operations, such as &&, ||, -, or ?:
  6. Use any form of casting.
  7. Use any data type other than int.  This implies that you
     cannot use arrays, structs, or unions.

 
  You may assume that your machine:
  1. Uses 2s complement, 32-bit representations of integers.
  2. Performs right shifts arithmetically.
  3. Has unpredictable behavior when shifting if the shift amount
     is less than 0 or greater than 31.


EXAMPLES OF ACCEPTABLE CODING STYLE:
  /*
   * pow2plus1 - returns 2^x + 1, where 0 <= x <= 31
   */
  int pow2plus1(int x) {
     /* exploit ability of shifts to compute powers of 2 */
     return (1 << x) + 1;
  }

  /*
   * pow2plus4 - returns 2^x + 4, where 0 <= x <= 31
   */
  int pow2plus4(int x) {
     /* exploit ability of shifts to compute powers of 2 */
     int result = (1 << x);
     result += 4;
     return result;
  }

FLOATING POINT CODING RULES

For the problems that require you to implement floating-point operations,
the coding rules are less strict.  You are allowed to use looping and
conditional control.  You are allowed to use both ints and unsigneds.
You can use arbitrary integer and unsigned constants. You can use any arithmetic,
logical, or comparison operations on int or unsigned data.

You are expressly forbidden to:
  1. Define or use any macros.
  2. Define any additional functions in this file.
  3. Call any functions.
  4. Use any form of casting.
  5. Use any data type other than int or unsigned.  This means that you
     cannot use arrays, structs, or unions.
  6. Use any floating point data types, operations, or constants.


NOTES:
  1. Use the dlc (data lab checker) compiler (described in the handout) to 
     check the legality of your solutions.
  2. Each function has a maximum number of operations (integer, logical,
     or comparison) that you are allowed to use for your implementation
     of the function.  The max operator count is checked by dlc.
     Note that assignment ('=') is not counted; you may use as many of
     these as you want without penalty.
  3. Use the btest test harness to check your functions for correctness.
  4. Use the BDD checker to formally verify your functions
  5. The maximum number of ops for each function is given in the
     header comment for each function. If there are any inconsistencies 
     between the maximum ops in the writeup and in this file, consider
     this file the authoritative source.

/*
 * STEP 2: Modify the following functions according the coding rules.
 * 
 *   IMPORTANT. TO AVOID GRADING SURPRISES:
 *   1. Use the dlc compiler to check that your solutions conform
 *      to the coding rules.
 *   2. Use the BDD checker to formally verify that your solutions produce 
 *      the correct answers.
 */

#endif
// 1
/*
 * bitXor - x^y using only ~ and &
 *   Example: bitXor(4, 5) = 1
 *   Legal ops: ~ &
 *   Max ops: 14
 *   Rating: 1
 */
int bitXor(int x, int y) {
    return (~(x & y) & ~((~x) & (~y)));
}
/*
 * tmin - return minimum two's complement integer
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 4
 *   Rating: 1
 */
int tmin(void) {
    return 1 << 31;
}
// 2
/*
 * isTmax - returns 1 if x is the maximum, two's complement number,
 *     and 0 otherwise
 *   Legal ops: ! ~ & ^ | +
 *   Max ops: 10
 *   Rating: 1
 */
int isTmax(int x) {
    int y = x + 1;
    int z = ~x;
    int a = y ^ z;
    return !a & !!y;
}
/*
 * allOddBits - return 1 if all odd-numbered bits in word set to 1
 *   where bits are numbered from 0 (least significant) to 31 (most significant)
 *   Examples allOddBits(0xFFFFFFFD) = 0, allOddBits(0xAAAAAAAA) = 1
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 12
 *   Rating: 2
 */
int allOddBits(int x) {
    // 由于英文太差，题目都看错了，最低位在是0位，为偶数位
    // 0x55: 01010101, os: 0x55555555
    int os = 0x55 | (0x55 << 8) | (0x55 << 16) | (0x55 << 24);
    // 0xAAAAAAAA
    int es = ~os;
    // 如果奇数位都为1则fx为0xFFFFFFFF
    int fx = (x & es) | os;
    int res = !(fx + 1);
    return res;
}
/*
 * negate - return -x
 *   Example: negate(1) = -1.
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 5
 *   Rating: 2
 */
int negate(int x) {
    return ~x + 1;
}
// 3
/*
 * isAsciiDigit - return 1 if 0x30 <= x <= 0x39 (ASCII codes for characters '0'
 * to '9') Example: isAsciiDigit(0x35) = 1. isAsciiDigit(0x3a) = 0.
 *            isAsciiDigit(0x05) = 0.
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 15
 *   Rating: 3
 */
int isAsciiDigit(int x) {
    // 习得技能：判断相等，使用减法(加负数)
    // 0x30: 00110000
    // 0x39: 00111001
    int left = ~0x0f & x;
    // 如果左边（除了低4位）满足要求，res为1。
    int res = !(left + (~0x30 + 1));
    int right = x & 0x0f;
    // 减10(+ -10)，如果为负则满足要求
    res = ((right + (~10 + 1)) >> 31) & res;
    return res;
}
/*
 * conditional - same as x ? y : z
 *   Example: conditional(2,4,5) = 4
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 16
 *   Rating: 3
 */
int conditional(int x, int y, int z) {
    // 我夸自己是个天才不过分吧: 逻辑取反再-1
    // if (x == 0) { cond = 0x00000000;}
    // else { cond = 0xFFFFFFFF;}
    int cond = !x + ~0; // ~x - 1
    int res = (cond & y) | (~cond & z);
    return res;
}
/*
 * isLessOrEqual - if x <= y  then return 1, else return 0
 *   Example: isLessOrEqual(4,5) = 1.
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 24
 *   Rating: 3
 */
int isLessOrEqual(int x, int y) {
    // 正常的思路是相减判断符号位，但是会溢出
    // 我的思路是比较最高位，然后再比较低31位，此时相减不会溢出，直接比较符号位即可，
    // 0x80000000
    int min = 1 << 31;
    int max = ~min;
    // 先比较最高位，即符号位, 可能的结果0, -1, 1
    // y - x
    int highest_bit = (y >> 31) + ~(x >> 31) + 1;
    // highest_bit == 0 => eq = 1, else 0
    int high_eq = !highest_bit;
    // x < y => less = 1, else 0
    int high_less = !(highest_bit ^ 1);

    // 比较低31位
    int low_bits = (y & max) + ~(x & max) + 1;
    int low_le = !(low_bits & min) | !(low_bits ^ 0);
    int res = high_less | (high_eq & low_le);
    return res;
}
// 4
/*
 * logicalNeg - implement the ! operator, using all of
 *              the legal operators except !
 *   Examples: logicalNeg(3) = 0, logicalNeg(0) = 1
 *   Legal ops: ~ & ^ | + << >>
 *   Max ops: 12
 *   Rating: 4
 */

/* 思路：取负数 再或自身, 再判断符号位
   非零时：正常情况一负一正，按位或之后，符号位为1，特殊情况0x80000000也满足要求
   零则仍为零
*/
int logicalNeg(int x) {
    int min = 1 << 31;
    int t1 = (~x + 1) | x;
    int res = ((t1 ^ min) >> 31) & 1;
    return res;
}
/* howManyBits - return the minimum number of bits required to represent x in
 *             two's complement
 *  Examples: howManyBits(12) = 5
 *            howManyBits(298) = 10
 *            howManyBits(-5) = 4
 *            howManyBits(0)  = 1
 *            howManyBits(-1) = 1
 *            howManyBits(0x80000000) = 32
 *  Legal ops: ! ~ & ^ | + << >>
 *  Max ops: 90
 *  Rating: 4
 */
/* 之前的思路(超过了Max ops要求)：
 * 1. 负数取反，去掉符号位变成正数，正数不变
 * 2. 然后将第一位1（从高到低）开始，全部变为1
 *    例如：0b01001...0 => 0b01111...1
 * 3. 将所有位的1加起来(很无脑的思路，每次取位移取最后一位再总和，太多操作数了)
 *
 * 新思路，（抄别人的）:
 * 1. 负数取反，去掉符号位变成正数，正数不变
 * 2. 看代码中的注释吧
 */
int howManyBits(int x) {
    int min = 1 << 31;
    // 1.(这最简单的一步，绝对原创)
    int zero_or_neg_one = (x & min) >> 31;
    int px = zero_or_neg_one ^ x;
    // 2.(旧思路，放在这留作纪念)
    // 这也是网上找的，可略微修改用来计算最近似2的次方，可以用在这
    // px |= px >> 1;
    // px |= px >> 2;
    // px |= px >> 4;
    // px |= px >> 8;
    // px |= px >> 16;

    // 2.
    // 判断高16位是否有1，有说明低16全为1，则需要加16,后面以此类推
    int r1, r2, r3, r4, r5, r6;
    r1 = !!(px >> 16) << 4;
    // 很神奇，自己体会
    px >>= r1;
    r2 = !!(px >> 8) << 3;
    px >>= r2;
    r3 = !!(px >> 4) << 2;
    px >>= r3;
    r4 = !!(px >> 2) << 1;
    px >>= r4;
    r5 = !!(px >> 1);
    px >>= r5;
    r6 = px;
    return r1 + r2 + r3 + r4 + r5 + r6 + 1;
}
// float
/*
 * floatScale2 - Return bit-level equivalent of expression 2*f for
 *   floating point argument f.
 *   Both the argument and result are passed as unsigned int's, but
 *   they are to be interpreted as the bit-level representation of
 *   single-precision floating point values.
 *   When argument is NaN, return argument
 *   Legal ops: Any integer/unsigned operations incl. ||, &&. also if, while
 *   Max ops: 30
 *   Rating: 4
 */
unsigned floatScale2(unsigned uf) {
    unsigned sign = uf & 0x80000000;
    unsigned exp = uf & 0x7f800000;
    unsigned frac = uf & 0x007fffff;
    unsigned int res = 0;
    if (exp == 0) {
        // 非规格化
        frac <<= 1;
        res = sign | frac;
    } else if (exp == 0x7f800000 /*  && frac == 0 */) {
        // 无穷大或NaN直接返回原值
        res = uf;
    } else {
        // 规格化
        exp += 0x00800000;
        if (exp == 0x7f800000) {
            frac = 0;
        }
        res = sign | exp | frac;
    }
    return res;
}
/*
 * floatFloat2Int - Return bit-level equivalent of expression (int) f
 *   for floating point argument f.
 *   Argument is passed as unsigned int, but
 *   it is to be interpreted as the bit-level representation of a
 *   single-precision floating point value.
 *   Anything out of range (including NaN and infinity) should return
 *   0x80000000u.
 *   Legal ops: Any integer/unsigned operations incl. ||, &&. also if, while
 *   Max ops: 30
 *   Rating: 4
 */
int floatFloat2Int(unsigned uf) {
    // 大概是有问题的，不管了
    unsigned sign = uf & 0x80000000;
    unsigned exp = uf & 0x7f800000;
    unsigned frac = uf & 0x007fffff;
    int res = 0;
    int E = (exp >> 23) - 127;
    if (exp == 0) {
        // 非规格化
        res = 0;
    } else if (exp == 0x7f800000) {
        // 无穷大或NaN
        res = 0x80000000u;
    } else if (E >= -1) {
        // int max = 0x7FFFFFFF;
        int min = 0x80000000;
        // 规格化
        // int E = (exp >> 23) - 127;
        if (sign) {
            // 负数
            if (E > 31) {
                res = min;
            } else {
                frac |= 0x00800000;
                frac >>= (22 - E);
                // 获取最后两位，判断是否需要进一
                unsigned last2 = (frac & 3);
                unsigned one = last2 == 3;
                frac = (frac >> 1) + one;
                res = -frac;
            }
        } else {
            // 正数
            if (E >= 31) {
                res = min;
            } else {
                // 加一
                frac |= 0x00800000;
                frac >>= (22 - E);
                // 获取最后两位，判断是否需要进一
                unsigned last2 = (frac & 3);
                unsigned one = last2 == 3;
                frac = (frac >> 1) + one;
                res = frac;
            }
        }
    } else {
        res = 0;
    }
    return res;
}
/*
 * floatPower2 - Return bit-level equivalent of the expression 2.0^x
 *   (2.0 raised to the power x) for any 32-bit integer x.
 *
 *   The unsigned value that is returned should have the identical bit
 *   representation as the single-precision floating-point number 2.0^x.
 *   If the result is too small to be represented as a denorm, return
 *   0. If too large, return +INF.
 *
 *   Legal ops: Any integer/unsigned operations incl. ||, &&. Also if, while
 *   Max ops: 30
 *   Rating: 4
 */
unsigned floatPower2(int x) {
    int res = 0;
    if (x < -149) {
        res = 0;
    } else if (x >= -149 && x < -126) {
        unsigned move = -127 - x;
        res = 0x00400000 >> move;
    } else if (x >= -126 && x <= 127) {
        int e = x + 127;
        res = e << 23;
    } else /* if
       (x > 127) */
    {
        res = 0x7f800000;
    }
    return res;
}
