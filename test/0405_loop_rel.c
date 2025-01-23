{
  int N;
  int x;
  N = rand(0,50);
  x = 0;
  while (x < N) {
    print(x,N);
    x = x + rand(0,3);
  }
  assert(x>=N && x<N+3);
}
