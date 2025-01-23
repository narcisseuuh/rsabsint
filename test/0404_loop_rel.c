{
  int N;
  int x;
  N = rand(0,50);
  x = 0;
  while (x < N) {
    print(x,N);
    x = x + 1;
  }
  assert(x==N);
}
