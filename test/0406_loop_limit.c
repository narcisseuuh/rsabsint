{
  int x;
  x = 0;
  while (rand(0,1)==0) {
    if (x < 100) x = x + 1;
  }
  print(x);
}
