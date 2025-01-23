{
  int x;
  int N;
  x = 0;
  N = rand(0,100);
  while (rand(0,1)==0) {
    if (x < N) x = x + 1;
  }
  print(x);
}
