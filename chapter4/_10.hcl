bool xor = (!a && b) || (a && !b);
bool eq = !(a xor b);
// 组成字级相等电路，将64个eq位级电路的值-> 与门->结果
