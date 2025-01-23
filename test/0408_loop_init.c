{
  int init;
  int x;
  init = 0;
  while (rand(0,1)==0) {
    if (init == 0) {
      x = 0;
      init = 1;
    }
    else {
      x = x + 1;
    }
    assert(x >= 0);
  }
}
