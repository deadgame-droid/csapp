#include <stdio.h>
#include <string.h>
int strlonger(char *s, char *t) {
  //   return strlen(s) - strlen(t) > 0;
  return strlen(s) > strlen(t);
}
int main() {
  char *s = "qwer";
  char *t = "qwert";
  printf("%s > %s = %d\n", s, t, strlonger(s, t));
}